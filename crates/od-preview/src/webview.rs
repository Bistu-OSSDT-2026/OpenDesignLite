//! WebView 生命周期（wry + tao 事件循环）。
//! 安全规则：预览页面不得获得 shell / 文件系统 / 命令执行 IPC。
//!
//! Spec: docs/specs/preview.md（加载策略 / 安全规则）

use crate::{PreviewError, PreviewOptions};
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

/// 自定义事件：文件变更触发 reload（携带要重新加载的 URL）。
#[derive(Debug, Clone)]
pub struct ReloadEvent(pub String);

/// 打开 WebView 加载 `url`，阻塞当前线程直到窗口关闭。
///
/// `reload_url` 为 reload 时应重新加载的 URL。`on_proxy` 在事件循环启动前被调用，
/// 收到 `EventLoopProxy`；调用方应把它交给 watcher 线程，变更时 `proxy.send_event(ReloadEvent(url))`。
pub fn open_webview(
    options: &PreviewOptions,
    url: &str,
    reload_url: String,
    on_proxy: impl FnOnce(EventLoopProxy<ReloadEvent>),
) -> Result<(), PreviewError> {
    let event_loop: EventLoop<ReloadEvent> =
        EventLoopBuilder::<ReloadEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title(format!("odl preview — {}", options.artifact_root.display()))
        .build(&event_loop)
        .map_err(|e| PreviewError::WebviewFailed(format!("window: {e}")))?;

    let webview = WebViewBuilder::new()
        .with_devtools(options.devtools)
        .with_url(url)
        .with_ipc_handler(|_| {
            // 安全：不处理任何来自页面的 IPC 消息，不暴露 native bridge。
        })
        .build(&window)
        .map_err(|e| PreviewError::WebviewFailed(format!("webview init: {e}")))?;

    // 阻塞前把 proxy 交给调用方（让它派发给 watcher 线程）。
    on_proxy(proxy);

    let reload_for_loop = reload_url.clone();
    // event_loop.run 返回 `!`（窗口关闭时进程退出），后续代码不可达。
    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(ReloadEvent(_)) => {
                if let Err(e) = webview.load_url(&reload_for_loop) {
                    eprintln!("warning: reload failed: {e}");
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
