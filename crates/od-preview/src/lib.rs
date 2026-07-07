//! od-preview：本地预览边界（主文件检测 + WebView + 文件监听 + Markdown 渲染）。
//! M1 默认技术为 `wry` + 系统 WebView，外部浏览器为 fallback。
//!
//! Spec: docs/specs/preview.md

pub mod detect;
pub mod error_page;
pub mod fallback;
pub mod render;
pub mod watch;
pub mod webview;

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use thiserror::Error;

/// 预览参数（对应 `odl preview` flags）。
#[derive(Debug, Clone)]
pub struct PreviewOptions {
    pub artifact_root: PathBuf,
    pub external_browser: bool,
    pub watch: bool,
    pub devtools: bool,
}

impl PreviewOptions {
    pub fn new(artifact_root: impl Into<PathBuf>) -> Self {
        Self {
            artifact_root: artifact_root.into(),
            external_browser: false,
            watch: true,
            devtools: false,
        }
    }
}

/// 预览错误码。`code()` 与 preview spec 的错误表对应。
///
/// Spec: docs/specs/preview.md（错误页）
#[derive(Debug, Error)]
pub enum PreviewError {
    #[error("artifact not found: {0}")]
    ArtifactNotFound(PathBuf),
    #[error("primary file missing in {0}")]
    PrimaryFileMissing(PathBuf),
    #[error("render failed: {0}")]
    RenderFailed(String),
    #[error("webview failed: {0}")]
    WebviewFailed(String),
    #[error("watch failed: {0}")]
    WatchFailed(String),
    /// 外部浏览器 fallback 失败。M1 新增：spec 错误表未列，单独区分便于诊断。
    #[error("external browser fallback failed: {0}")]
    FallbackFailed(String),
}

impl PreviewError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ArtifactNotFound(_) => "artifact_not_found",
            Self::PrimaryFileMissing(_) => "primary_file_missing",
            Self::RenderFailed(_) => "render_failed",
            Self::WebviewFailed(_) => "webview_failed",
            Self::WatchFailed(_) => "watch_failed",
            Self::FallbackFailed(_) => "fallback_failed",
        }
    }
}

/// 把本地路径转成 WebView 能加载的 file:// URL。
///
/// 注意：Rust 在 Windows 上 `canonicalize()` 会返回 `\\?\C:\...` 这种
/// 扩展长度路径（verbatim 前缀）。这个前缀会让 file:// URL 变成非法的
/// `file://?/...`，WebView2 会拒绝。必须先剥掉 `\\?\`。
pub fn file_url(path: &Path) -> String {
    let mut s = path.display().to_string();
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

/// 预览入口：检测主文件 → 渲染（Markdown）→ 启动 watcher → 加载 WebView。
///
/// Spec: docs/specs/preview.md
pub fn preview(options: &PreviewOptions) -> Result<(), PreviewError> {
    let root: &Path = &options.artifact_root;

    if !root.exists() {
        return Err(PreviewError::ArtifactNotFound(root.to_path_buf()));
    }

    let (primary, kind) = detect::detect_primary(root)
        .ok_or_else(|| PreviewError::PrimaryFileMissing(root.join("index.html")))?;

    // canonicalize 成绝对路径：file_url 必须拿绝对路径才能生成合法 file:// URL。
    // 相对路径会变成 file:///.odl-demo/...，WebView2 解析不了（显示"未找到文件"）。
    // Windows 上 canonicalize 会加 \\?\ 前缀，file_url 内部已处理。
    let primary = primary
        .canonicalize()
        .map_err(|_e| PreviewError::PrimaryFileMissing(primary))?;

    // Markdown 要先渲染成 HTML 落盘，才能用 file:// 加载并支持 reload。
    // 临时文件放 .odl/（已在 .gitignore），watcher 要忽略它，避免自触发。
    let url = match kind {
        od_core::ArtifactKind::Markdown => {
            let html = render::markdown::render_markdown(&primary)?;
            let tmp = root.join(".odl").join("preview.html");
            std::fs::create_dir_all(tmp.parent().expect("tmp has parent"))
                .map_err(|e| PreviewError::RenderFailed(format!("create tmp dir: {e}")))?;
            std::fs::write(&tmp, &html)
                .map_err(|e| PreviewError::RenderFailed(format!("write tmp: {e}")))?;
            file_url(&tmp.canonicalize().map_err(|e| PreviewError::RenderFailed(e.to_string()))?)
        }
        _ => file_url(&primary),
    };

    // 外部浏览器分支：不弹原生窗口，交给系统浏览器，watcher 不保证刷新。
    if options.external_browser {
        return fallback::open_external(&primary);
    }

    // watch 回调不直接动 webview，只往通道塞信号；事件循环轮询通道再 reload。
    // 这样跨线程安全：notify 回调在别的线程，webview 只在主线程动。
    let (reload_tx, reload_rx) = mpsc::channel::<()>();
    // watcher 必须在主线程保活：notify 的事件派发依赖创建它的线程。
    // 用 Option 装着，和下面的 open_webview 在同一作用域，活到事件循环结束。
    let _watcher = if options.watch {
        Some(watch::watch(root, move || {
            let _ = reload_tx.send(());
        })?)
    } else {
        None
    };

    webview::open_webview(options, &url, reload_rx)
}
