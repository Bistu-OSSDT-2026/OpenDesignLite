//! WebView 生命周期（wry + 事件循环）。
//! 安全规则：预览页面不得获得 shell / 文件系统 / 命令执行 IPC。
//!
//! 加载策略：custom protocol `odl-shell` 服务壳页面，产物在固定视口 iframe 里
//! 渲染（见 `shell.rs`；examples/shell_spike.rs 验证记录）。
//!
//! Spec: docs/specs/preview.md（加载策略 / 窗口与视口 / 稳定性 / 安全规则）

use crate::shell::{self, ShellConfig};
use crate::{PreviewError, PreviewOptions};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebContext, WebViewBuilder};

/// 事件循环轮询通道的间隔。太短烧 CPU，太长 reload 滞后。50ms 折中。
const RELOAD_POLL_INTERVAL: Duration = Duration::from_millis(50);

/// 单实例锁心跳间隔：每次刷新锁文件 mtime。MCP 侧以 10s 为过期窗口，
/// 2s 心跳留足余量（spec：稳定性/单实例锁）。
const LOCK_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);

/// 窗口尺寸 = 固定视口 + 壳顶栏（设计常量，非百分比），保证内容区精确等于
/// 部署后浏览器视口（spec：窗口与视口）。
fn window_size_for(config: &ShellConfig) -> LogicalSize<f64> {
    let (vw, vh) = shell::viewport_for(config.kind);
    LogicalSize::new(f64::from(vw), f64::from(vh + shell::SHELL_HEADER_HEIGHT))
}

/// WebView2 user data folder：固定到每用户目录并全实例共享持久复用。
/// 不固定时 WebView2 会落到可执行文件旁或临时目录——只读安装位置或
/// 多进程抢占默认目录都会让 WebView 初始化直接失败（spec：稳定性）。
fn webview_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(not(target_os = "windows"))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share"))
        });
    let dir = base?.join("OpenDesignLite").join("webview-data");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

/// 打开壳窗口并在固定视口 iframe 里加载产物。
///
/// `reload_rx` 用于 watch 联动：watch 后台线程收到文件变更信号后塞进通道，
/// 事件循环每轮轮询通道，收到信号只刷新 iframe（壳与面板状态保留）。
///
/// `lock` 为单实例锁文件路径：事件循环按心跳刷新其 mtime，`CloseRequested`
/// 时删除。异常退出不清理也没关系——锁靠 mtime 过期兜底。
pub fn open_webview(
    options: &PreviewOptions,
    config: ShellConfig,
    reload_rx: Receiver<()>,
    lock: Option<PathBuf>,
) -> Result<(), PreviewError> {
    let size = window_size_for(&config);
    let title = format!("{} — Open Design Lite", config.title);

    let event_loop = EventLoop::new();
    // 固定尺寸、不可缩放：resizable(false) + min/max 双保险
    //（部分平台/窗口管理器会无视 resizable）。
    let window = WindowBuilder::new()
        .with_title(&title)
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .build(&event_loop)
        .map_err(|e| PreviewError::WebviewFailed(format!("create window: {e}")))?;

    // WebContext 承载 user data folder；它必须活得比 webview 久，
    // 所以随 webview 一起 move 进事件循环闭包。
    let mut web_context = WebContext::new(webview_data_dir());
    let handler_config = Arc::new(config);
    let mut builder = WebViewBuilder::with_web_context(&mut web_context)
        .with_custom_protocol("odl-shell".to_string(), {
            let config = Arc::clone(&handler_config);
            move |_id, request| shell::handle_request(&config, &request)
        })
        .with_url(shell::entry_url());
    if options.devtools {
        builder = builder.with_devtools(true);
    }

    // _webview 必须保活到事件循环结束，否则窗口立刻关闭。
    // 用 Option 以便在 reload 分支里取用 evaluate_script。
    let webview = Some(
        builder
            .build(&window)
            .map_err(|e| PreviewError::WebviewFailed(format!("build webview: {e}")))?,
    );

    let mut last_heartbeat = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        // WebContext 随闭包保活（webview 依赖它）。
        let _ = &web_context;

        // 用 WaitUntil 而不是 Wait：Wait 只在窗口事件时醒来，
        // 通道里的 reload 信号不是窗口事件，会被永远忽略。
        // WaitUntil 设 50ms 超时，定期醒来轮询通道，又不至于忙等烧 CPU。
        *control_flow = ControlFlow::WaitUntil(Instant::now() + RELOAD_POLL_INTERVAL);

        // 单实例锁心跳：定期刷新 mtime，向 MCP 侧证明窗口还活着。
        if let Some(lock_path) = lock.as_ref() {
            if last_heartbeat.elapsed() >= LOCK_HEARTBEAT_INTERVAL {
                let _ = std::fs::write(lock_path, std::process::id().to_string());
                last_heartbeat = Instant::now();
            }
        }

        // 轮询 reload 通道：非阻塞地看一眼有没有信号。
        // try_recv 在空通道时返回 Err，正常情况不是错误。
        // 只刷新 iframe（带时间戳防缓存），壳与面板折叠状态不重置。
        while let Ok(()) = reload_rx.try_recv() {
            if let Some(wv) = webview.as_ref() {
                if let Err(e) = wv.evaluate_script(shell::RELOAD_SCRIPT) {
                    eprintln!("[od-preview] reload failed: {e}");
                }
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            // 关窗时清理锁；异常路径靠 mtime 过期兜底，不强依赖这里。
            if let Some(lock_path) = lock.as_ref() {
                let _ = std::fs::remove_file(lock_path);
            }
            *control_flow = ControlFlow::Exit;
        }
    });
}
