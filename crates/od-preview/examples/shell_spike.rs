//! Spike：验证 wry 0.48 custom protocol + 壳页面 iframe 方案（Phase 3 前置）。
//!
//! 验证三件事（docs/specs/preview.md 加载策略）：
//! 1. `with_custom_protocol` 的闭包签名与注册；
//! 2. 壳页面里的 iframe 同 scheme 请求是否被同一 handler 拦截；
//! 3. iframe 文档内相对路径资源（assets/...）能否正确解析。
//!
//! 自验证：页面 JS 把结果 fetch 到 `/probe/<msg>`，handler 写入
//! `%TEMP%/odl-shell-spike.log`；窗口 6 秒后自动退出。跑完看日志即知结论：
//! `cargo run -p od-preview --example shell_spike`

use std::borrow::Cow;
use std::io::Write;
use std::time::{Duration, Instant};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::http::{header::CONTENT_TYPE, Request, Response, StatusCode};
use wry::WebViewBuilder;

const SHELL_HTML: &str = r#"<!doctype html><html><head><meta charset="utf-8"><style>
  body { margin: 0; background: #111; color: #eee; font-family: system-ui; }
  header { height: 40px; display: flex; align-items: center; padding: 0 12px; }
  iframe { display: block; width: 640px; height: 360px; border: 0; margin: 0 auto; background: #fff; }
</style></head><body>
<header>shell chrome</header>
<iframe id="stage" src="/artifact/index.html"></iframe>
<script>
  document.getElementById('stage').addEventListener('load', () => fetch('/probe/iframe-loaded'));
</script>
</body></html>"#;

const ARTIFACT_HTML: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<link rel="stylesheet" href="assets/probe.css">
</head><body><h1>artifact</h1>
<script>
  fetch('assets/probe.css')
    .then(r => r.ok ? fetch('/probe/relative-asset-ok') : fetch('/probe/relative-asset-status-' + r.status))
    .catch(() => fetch('/probe/relative-asset-failed'));
</script>
</body></html>"#;

const ARTIFACT_CSS: &str = "body { background: #fafafa; }";

fn spike_log() -> std::path::PathBuf {
    std::env::temp_dir().join("odl-shell-spike.log")
}

fn log_probe(msg: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(spike_log())
    {
        let _ = writeln!(f, "{msg}");
    }
}

/// 从两种 URL 形式里取出 path：
/// - macOS/Linux: `odl-shell://shell/artifact/index.html`
/// - Windows:     `http://odl-shell.shell/artifact/index.html`
fn request_path(uri: &str) -> String {
    let after_scheme = uri.split("://").nth(1).unwrap_or(uri);
    match after_scheme.find('/') {
        Some(idx) => after_scheme[idx..].to_string(),
        None => "/".to_string(),
    }
}

fn respond(body: &'static str, mime: &str, status: StatusCode) -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, mime)
        .body(Cow::Borrowed(body.as_bytes()))
        .expect("static response builds")
}

fn main() -> wry::Result<()> {
    let _ = std::fs::remove_file(spike_log());
    log_probe("spike-start");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("odl shell spike")
        .build(&event_loop)
        .expect("window");

    #[cfg(target_os = "windows")]
    let entry = "http://odl-shell.shell/index.html";
    #[cfg(not(target_os = "windows"))]
    let entry = "odl-shell://shell/index.html";

    let webview = WebViewBuilder::new()
        .with_custom_protocol("odl-shell".to_string(), |_id, request: Request<Vec<u8>>| {
            let path = request_path(&request.uri().to_string());
            log_probe(&format!("request {path}"));
            match path.as_str() {
                "/index.html" => respond(SHELL_HTML, "text/html", StatusCode::OK),
                "/artifact/index.html" => respond(ARTIFACT_HTML, "text/html", StatusCode::OK),
                "/artifact/assets/probe.css" => respond(ARTIFACT_CSS, "text/css", StatusCode::OK),
                p if p.starts_with("/probe/") => {
                    log_probe(&format!("probe {}", &p["/probe/".len()..]));
                    respond("", "text/plain", StatusCode::NO_CONTENT)
                }
                _ => respond("not found", "text/plain", StatusCode::NOT_FOUND),
            }
        })
        .with_url(entry)
        .build(&window)?;
    let _webview = webview;

    let deadline = Instant::now() + Duration::from_secs(6);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(100));
        if Instant::now() >= deadline {
            log_probe("spike-timeout-exit");
            *control_flow = ControlFlow::Exit;
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
