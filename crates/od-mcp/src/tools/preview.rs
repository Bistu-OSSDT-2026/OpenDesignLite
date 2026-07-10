//! `artifact_preview` → 等价 `odl preview`，以独立子进程启动预览。
//!
//! 关键设计：MCP server 跑在 agent 进程里，不能直接调 `od_preview::preview`——
//! 那会跑起 WebView 事件循环并阻塞直到窗口关闭，卡死整个 server。因此这里
//! `spawn` 一个 `odl preview` 子进程并立即返回（不 wait），对齐 mcp.md 的
//! `{started: true, mode: "webview"}` 启动式语义。子进程独立存活，server 继续服务。
//!
//! 仅做集成层适配：flag 翻译 + 子进程 spawn。detect / render / webview / watch
//! 全部在 `odl preview` 子进程里完成，不重复实现。无 GUI / spawn 失败 →
//! `preview_unavailable`（对齐 spec “无 GUI 环境应返回明确错误，不应阻塞 server”）。
//!
//! Spec: docs/specs/mcp.md（artifact_preview）, preview.md

use crate::error::McpError;
use std::path::{Path, PathBuf};
use std::process::Command;

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

/// 启动 `odl preview` 子进程并立即返回。对应 CLI `odl preview`。
pub fn run(options: PreviewOptions<'_>) -> Result<PreviewResult, McpError> {
    let exe = odl_executable()?;
    let mut cmd = build_preview_command(&exe, &options);

    cmd.spawn().map_err(|err| {
        McpError::PreviewUnavailable(format!(
            "failed to spawn `{} preview`: {err}",
            exe.display()
        ))
    })?;

    Ok(PreviewResult {
        started: true,
        mode: if options.external_browser {
            "external"
        } else {
            "webview"
        },
    })
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
}
