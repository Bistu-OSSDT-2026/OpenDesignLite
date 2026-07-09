//! WebView 生命周期（wry + 事件循环）。
//! 安全规则：预览页面不得获得 shell / 文件系统 / 命令执行 IPC。
//!
//! Spec: docs/specs/preview.md（加载策略 / 安全规则）

use crate::{PreviewError, PreviewOptions};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

/// 事件循环轮询通道的间隔。太短烧 CPU，太长 reload 滞后。50ms 折中。
const RELOAD_POLL_INTERVAL: Duration = Duration::from_millis(50);

/// 预览窗口初始尺寸（逻辑像素）。窗口仍可自由缩放，只是首次打开给一个
/// 适合看产物的标准大小，而不是 tao 默认的极小窗口。
const DEFAULT_WINDOW_SIZE: LogicalSize<f64> = LogicalSize::new(1280.0, 800.0);

/// 打开 WebView 并加载 `url`（M1 为 `file://` 主文件或渲染后的临时 HTML）。
///
/// `reload_rx` 用于 watch 联动：watch 后台线程收到文件变更信号后塞进通道，
/// 事件循环每轮轮询通道，收到信号就 `load_url` 重新加载。
/// 传 `None`（例如外部浏览器分支）时不会触发 reload。
pub fn open_webview(
    options: &PreviewOptions,
    url: &str,
    reload_rx: Receiver<()>,
) -> Result<(), PreviewError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Open Design Lite")
        .with_inner_size(DEFAULT_WINDOW_SIZE)
        .build(&event_loop)
        .map_err(|e| PreviewError::WebviewFailed(format!("create window: {e}")))?;

    let mut builder = WebViewBuilder::new().with_url(url);
    if options.devtools {
        builder = builder.with_devtools(true);
    }

    // _webview 必须保活到事件循环结束，否则窗口立刻关闭。
    // 用 Option 以便在 reload 分支里取用 load_url。
    let webview = Some(
        builder
            .build(&window)
            .map_err(|e| PreviewError::WebviewFailed(format!("build webview: {e}")))?,
    );

    let url_owned = url.to_string();
    event_loop.run(move |event, _, control_flow| {
        // 用 WaitUntil 而不是 Wait：Wait 只在窗口事件时醒来，
        // 通道里的 reload 信号不是窗口事件，会被永远忽略。
        // WaitUntil 设 50ms 超时，定期醒来轮询通道，又不至于忙等烧 CPU。
        *control_flow = ControlFlow::WaitUntil(Instant::now() + RELOAD_POLL_INTERVAL);

        // 轮询 reload 通道：非阻塞地看一眼有没有信号。
        // try_recv 在空通道时返回 Err，正常情况不是错误。
        while let Ok(()) = reload_rx.try_recv() {
            if let Some(wv) = webview.as_ref() {
                // 重新加载同一个 URL。Markdown 场景下文件已被 watch 逻辑重写。
                if let Err(e) = wv.load_url(&url_owned) {
                    eprintln!("[od-preview] reload failed: {e}");
                }
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
