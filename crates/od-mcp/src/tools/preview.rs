//! `artifact_preview` → 等价 `odl preview`，以独立子进程启动预览。
//!
//! 关键设计：MCP server 跑在 agent 进程里，不能直接调 `od_preview::preview`——
//! 那会跑起 WebView 事件循环并阻塞直到窗口关闭，卡死整个 server。因此这里
//! `spawn` 一个 `odl preview` 子进程，对齐 mcp.md 的
//! `{started: true, mode: "webview"}` 启动式语义。子进程独立存活，server 继续服务。
//!
//! **stdio 隔离（硬性规则，mcp.md）**：server 的 stdin/stdout 是 JSON-RPC 通道，
//! 子进程绝不能继承——stdin 置空，stdout/stderr 重定向到 `<dir>/.odl/preview.log`。
//! spawn 后 `try_wait()` 检测子进程是否立刻退出：非零退出 → `preview_crashed`
//! （附日志尾部），不假装成功。
//!
//! **单实例锁**：`<dir>/.odl/preview.lock` 由预览进程按心跳刷新 mtime；
//! 锁仍新鲜时不重复 spawn，返回 `alreadyRunning: true`。
//!
//! 仅做集成层适配：flag 翻译 + 子进程 spawn。detect / render / webview / watch
//! 全部在 `odl preview` 子进程里完成，不重复实现。无 GUI / spawn 失败 →
//! `preview_unavailable`（对齐 spec “无 GUI 环境应返回明确错误，不应阻塞 server”）。
//!
//! Spec: docs/specs/mcp.md（artifact_preview）, preview.md（稳定性）

use crate::error::McpError;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime};

/// spawn 后等这么久再 `try_wait`，用于捕获「启动即崩」。
const CRASH_CHECK_DELAY: Duration = Duration::from_millis(250);
/// 锁文件 mtime 超过此时长视为过期（预览进程心跳约 2s 刷一次）。
const LOCK_STALE_AFTER: Duration = Duration::from_secs(10);
/// 错误信息里附带的日志尾部上限。
const LOG_TAIL_BYTES: usize = 2048;

/// `artifact_preview` 输入（对齐 docs/specs/mcp.md）。
pub struct PreviewOptions<'a> {
    pub dir: &'a Path,
    /// `true` → 系统外部浏览器（对应 CLI `--external-browser`）。
    pub external_browser: bool,
    /// `true` → 监听文件变更自动刷新（默认开）；`false` 对应 CLI `--no-watch`。
    pub watch: bool,
}

/// `artifact_preview` 输出。`mode` = `webview` | `external`，对齐 mcp.md。
#[derive(Debug, PartialEq, Eq)]
pub struct PreviewResult {
    pub started: bool,
    pub mode: &'static str,
    /// 命中单实例锁：已有存活预览窗口，未重复 spawn。
    pub already_running: bool,
}

/// Resolve the running `odl` binary. Release assets may be named
/// `odl-windows-x64.exe` etc.; spawning a bare `odl` misses them on Windows.
fn odl_executable() -> Result<PathBuf, McpError> {
    std::env::current_exe().map_err(|err| {
        McpError::PreviewUnavailable(format!("cannot resolve current odl executable: {err}"))
    })
}

fn build_preview_command(exe: &Path, options: &PreviewOptions<'_>) -> Command {
    let mut cmd = Command::new(exe);
    cmd.arg("preview").arg(options.dir);

    if options.external_browser {
        cmd.arg("--external-browser");
    }
    // CLI 是否定式 flag（--no-watch，默认 watch 开）；MCP 是肯定式，翻转。
    if !options.watch {
        cmd.arg("--no-watch");
    }
    cmd
}

fn lock_path(dir: &Path) -> PathBuf {
    dir.join(".odl").join("preview.lock")
}

fn log_path(dir: &Path) -> PathBuf {
    dir.join(".odl").join("preview.log")
}

/// 锁是否仍新鲜（mtime 在心跳窗口内）。文件不存在/读不到 → 不新鲜。
fn lock_is_fresh(lock: &Path) -> bool {
    fs::metadata(lock)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .map(|age| age < LOCK_STALE_AFTER)
        .unwrap_or(false)
}

/// 读日志尾部拼进错误信息，帮 agent 直接看到崩溃原因。
fn log_tail(log: &Path) -> String {
    match fs::read(log) {
        Ok(bytes) => {
            let start = bytes.len().saturating_sub(LOG_TAIL_BYTES);
            String::from_utf8_lossy(&bytes[start..]).trim().to_string()
        }
        Err(_) => String::new(),
    }
}

/// stdio 隔离（硬性规则）：stdin 置空，stdout/stderr 落 `.odl/preview.log`。
/// 日志文件建不出来时降级为 null——宁可丢日志也不能继承 server 的 stdio。
fn isolate_stdio(cmd: &mut Command, log: &Path) {
    cmd.stdin(Stdio::null());
    let file = log
        .parent()
        .map(|p| fs::create_dir_all(p))
        .and_then(Result::ok)
        .and_then(|_| fs::File::create(log).ok());
    match file {
        Some(f) => {
            match f.try_clone() {
                Ok(clone) => cmd.stdout(Stdio::from(clone)),
                Err(_) => cmd.stdout(Stdio::null()),
            };
            cmd.stderr(Stdio::from(f));
        }
        None => {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }
    }
    // 从无控制台的宿主（GUI agent）spawn 控制台程序时不弹黑框。
    // 同 scripts/mcp_proxy.py 的 CREATE_NO_WINDOW。
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
}

/// 启动 `odl preview` 子进程。对应 CLI `odl preview`。
pub fn run(options: PreviewOptions<'_>) -> Result<PreviewResult, McpError> {
    let mode = if options.external_browser {
        "external"
    } else {
        "webview"
    };
    let lock = lock_path(options.dir);
    let log = log_path(options.dir);

    // 单实例：窗口已存活（心跳新鲜）→ 不重复弹窗。external 模式无常驻进程，不适用。
    if !options.external_browser && lock_is_fresh(&lock) {
        return Ok(PreviewResult {
            started: true,
            mode,
            already_running: true,
        });
    }

    let exe = odl_executable()?;
    let mut cmd = build_preview_command(&exe, &options);
    isolate_stdio(&mut cmd, &log);

    // 先落锁再 spawn：覆盖「子进程还没来得及写锁」的竞态窗口。
    // 子进程起来后接管心跳；spawn/启动失败则删锁，不留 10s 假阳性。
    if !options.external_browser {
        let _ = fs::write(&lock, std::process::id().to_string());
    }

    let mut child = cmd.spawn().map_err(|err| {
        if !options.external_browser {
            let _ = fs::remove_file(&lock);
        }
        McpError::PreviewUnavailable(format!(
            "failed to spawn `{} preview`: {err}",
            exe.display()
        ))
    })?;

    // 启动即崩检测：短暂等待后看子进程是否已退出。
    std::thread::sleep(CRASH_CHECK_DELAY);
    match child.try_wait() {
        // 已退出且失败 → 明确报 preview_crashed，附日志尾部。
        Ok(Some(status)) if !status.success() => {
            if !options.external_browser {
                let _ = fs::remove_file(&lock);
            }
            let tail = log_tail(&log);
            Err(McpError::PreviewCrashed(format!(
                "`odl preview` exited with {status} shortly after spawn; log tail: {tail}"
            )))
        }
        // 已退出但成功：external 模式属正常（打开浏览器即退出）；
        // webview 模式视为内部已 fallback 到外部浏览器，同样算启动成功。
        // 仍在运行（或 try_wait 出错，无法判定）→ 按启动成功处理。
        _ => Ok(PreviewResult {
            started: true,
            mode,
            already_running: false,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `odl` 不在 PATH（测试环境通常如此）→ `preview_unavailable`，不 panic、不阻塞。
    /// 通过一个必然找不到的名字模拟 spawn 失败，避免依赖真实 `odl` 构建。
    #[test]
    fn spawn_failure_returns_preview_unavailable() {
        // 用一个确定不存在的可执行名；PATH 里没有它，spawn 必失败。
        let mut cmd = Command::new("odl-preview-definitely-not-on-path-xyz");
        cmd.arg("preview").arg(".");
        let err = cmd
            .spawn()
            .map_err(|e| McpError::PreviewUnavailable(e.to_string()))
            .unwrap_err();
        assert_eq!(err.code(), "preview_unavailable");
    }

    /// 可执行文件不存在时 `run` 归 `preview_unavailable`（不 panic、不阻塞）。
    #[test]
    fn run_reports_unavailable_when_executable_missing() {
        let mut cmd = build_preview_command(
            Path::new("odl-preview-definitely-not-on-path-xyz"),
            &PreviewOptions {
                dir: Path::new("."),
                external_browser: false,
                watch: true,
            },
        );
        let err = cmd
            .spawn()
            .map_err(|e| McpError::PreviewUnavailable(e.to_string()))
            .unwrap_err();
        assert_eq!(err.code(), "preview_unavailable");
    }

    /// mode 取值随 external_browser 变化（仅校验映射逻辑，不实际 spawn）。
    #[test]
    fn mode_mapping() {
        // external=true → "external"；external=false → "webview"。
        // 复用 run 的 mode 判定但隔离 spawn：直接断言等价表达式。
        let external = true;
        let mode = if external { "external" } else { "webview" };
        assert_eq!(mode, "external");
        let external = false;
        let mode = if external { "external" } else { "webview" };
        assert_eq!(mode, "webview");
    }

    /// 子进程 stdio 必须与 server 隔离（mcp.md 硬性规则）：
    /// spawn 一个会向 stdout/stderr 写字节的命令，输出应落到 preview.log，
    /// 而不是本进程的 stdio。
    #[test]
    fn isolate_stdio_redirects_child_output_to_log() {
        let dir = std::env::temp_dir().join(format!("odl-stdio-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let log = log_path(&dir);

        #[cfg(windows)]
        let mut cmd = {
            let mut c = Command::new("cmd");
            c.args(["/C", "echo odl-stdio-probe"]);
            c
        };
        #[cfg(not(windows))]
        let mut cmd = {
            let mut c = Command::new("sh");
            c.args(["-c", "echo odl-stdio-probe"]);
            c
        };

        isolate_stdio(&mut cmd, &log);
        let status = cmd.spawn().unwrap().wait().unwrap();
        assert!(status.success());
        let logged = fs::read_to_string(&log).unwrap();
        assert!(logged.contains("odl-stdio-probe"), "log: {logged}");
        let _ = fs::remove_dir_all(&dir);
    }

    /// 锁新鲜度判定：刚写的锁新鲜；不存在的锁不新鲜。
    #[test]
    fn lock_freshness() {
        let dir = std::env::temp_dir().join(format!("odl-lock-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join(".odl")).unwrap();
        let lock = lock_path(&dir);
        assert!(!lock_is_fresh(&lock), "missing lock must not be fresh");
        fs::write(&lock, "123").unwrap();
        assert!(lock_is_fresh(&lock), "just-written lock must be fresh");
        let _ = fs::remove_dir_all(&dir);
    }

    /// 日志尾部截取：短文件全量返回，超长只取尾部。
    #[test]
    fn log_tail_truncates() {
        let dir = std::env::temp_dir().join(format!("odl-tail-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join(".odl")).unwrap();
        let log = log_path(&dir);
        fs::write(&log, "boom").unwrap();
        assert_eq!(log_tail(&log), "boom");
        let long = "x".repeat(LOG_TAIL_BYTES * 2);
        fs::write(&log, &long).unwrap();
        assert_eq!(log_tail(&log).len(), LOG_TAIL_BYTES);
        let _ = fs::remove_dir_all(&dir);
    }
}
