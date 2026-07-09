//! 最小预览窗口 demo：用 wry + tao 弹一个原生窗口，加载本地 HTML 文件。
//!
//! 跑法（在仓库根目录，经 scripts/build.ps1 进入 MSVC 环境）：
//!   powershell -File scripts\build.ps1 run -p od-preview --example hello_window
//!
//! 默认加载 templates/html-page/basic.html；也可传一个路径：
//!   -- <到某个 index.html 的绝对路径>
//!
//! 这是「第 1 步」探针：先确认 WebView2 在本机能弹窗、能显示 file:// 页面，
//! 再去填 od-preview::preview() 那些 todo!()。

use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
    // 解析要加载的文件：命令行第一个非 flag 参数，否则用项目自带模板。
    let target: PathBuf = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .map(PathBuf::from)
        .unwrap_or_else(default_template);

    if !target.exists() {
        eprintln!("target file not found: {}", target.display());
        std::process::exit(2);
    }

    // file:// URL：Windows 路径要转成正斜杠并加上前缀。
    let url = file_url(&target);
    println!("[hello_window] loading {}", url);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Open Design Lite · preview demo")
        .build(&event_loop)
        .expect("failed to create window");

    // Windows 上 WebViewBuilder::build 接收 &window。
    // _webview 必须保活到事件循环结束，否则窗口立刻关闭。
    let _webview = WebViewBuilder::new().with_url(&url).build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}

/// 默认加载仓库内的模板，方便不传参数也能看到东西。
fn default_template() -> PathBuf {
    // examples/ 在 crates/od-preview/examples/，模板在仓库根 templates/。
    // 用 CARGO_MANIFEST_DIR 拿到 od-preview 的目录，再往上一级找仓库根。
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest)
        .join("../../templates/html-page/basic.html")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("templates/html-page/basic.html"))
}

/// 把本地路径转成 WebView 能加载的 file:// URL。
///
/// 注意：Rust 在 Windows 上 `canonicalize()` 会返回 `\\?\C:\...` 这种
/// 扩展长度路径（verbatim 前缀）。这个前缀会让 file:// URL 变成非法的
/// `file://?/...`，WebView2 会拒绝。必须先剥掉 `\\?\`。
fn file_url(path: &PathBuf) -> String {
    let mut s = path.display().to_string();
    // 剥掉 verbatim 前缀 \\?\ （大小写不敏感）
    let lower = s.to_ascii_lowercase();
    if lower.starts_with(r"\\?\") {
        s = s[r"\\?\".len()..].to_string();
    }
    let s = s.replace('\\', "/");
    if s.starts_with("//") {
        format!("file:{}", s)
    } else if s.starts_with('/') {
        format!("file://{}", s)
    } else {
        // Windows 的 C:/... 形式
        format!("file:///{}", s)
    }
}
