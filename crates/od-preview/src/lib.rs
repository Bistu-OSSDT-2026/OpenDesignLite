//! od-preview：本地预览边界（主文件检测 + WebView + 文件监听 + Markdown 渲染）。
//! M1 默认技术为 `wry` + 系统 WebView，外部浏览器为 fallback。
//! 本次仅模块划分：重型依赖（wry / comrak / ammonia / notify）未接入，只留签名。
//!
//! Spec: docs/specs/preview.md

pub mod detect;
pub mod error_page;
pub mod fallback;
pub mod render;
pub mod watch;
pub mod webview;

use std::path::PathBuf;
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
}

impl PreviewError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ArtifactNotFound(_) => "artifact_not_found",
            Self::PrimaryFileMissing(_) => "primary_file_missing",
            Self::RenderFailed(_) => "render_failed",
            Self::WebviewFailed(_) => "webview_failed",
            Self::WatchFailed(_) => "watch_failed",
        }
    }
}

/// 预览入口：检测主文件 → 渲染（Markdown）→ 加载 WebView → 启动 watcher。M1 实现。
pub fn preview(_options: &PreviewOptions) -> Result<(), PreviewError> {
    todo!("M1: wire detect -> render -> webview -> watch; see docs/specs/preview.md")
}
