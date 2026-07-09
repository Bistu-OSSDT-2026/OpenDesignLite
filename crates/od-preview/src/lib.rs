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

use crate::detect::detect_primary;
use crate::render::markdown::{path_to_file_url, render_markdown};
use crate::webview::ReloadEvent;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
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

/// 预览入口：检测主文件 → 渲染（Markdown）→ 加载 WebView/外部浏览器 → 启动 watcher。
///
/// 默认走 `wry` 原生窗口（watcher 在后台线程，变更自动 reload）；
/// `--external-browser` 或 webview 初始化失败时 fallback 到系统浏览器。
pub fn preview(options: &PreviewOptions) -> Result<(), PreviewError> {
    let root = &options.artifact_root;
    if !root.exists() {
        return Err(PreviewError::ArtifactNotFound(root.clone()));
    }

    let (primary, kind) =
        detect_primary(root).ok_or_else(|| PreviewError::PrimaryFileMissing(root.clone()))?;

    // 算可加载路径：HTML/Slides 直接用主文件；Markdown 渲染为临时 HTML。
    let load_path: PathBuf = match kind {
        od_core::ArtifactKind::Markdown => {
            let html = render_markdown(&primary, root)?;
            let tmp = std::env::temp_dir().join(format!("od-preview-{}.html", std::process::id()));
            std::fs::write(&tmp, html)
                .map_err(|e| PreviewError::RenderFailed(format!("write temp: {e}")))?;
            tmp
        }
        od_core::ArtifactKind::Html | od_core::ArtifactKind::Slides => primary.clone(),
    };

    let url = path_to_file_url(&load_path);

    if options.external_browser {
        // 显式 fallback。
        eprintln!("info: opening in external browser");
        fallback::open_external(&load_path)?;
        run_watch_blocking(options, root, &primary, kind, &load_path);
        return Ok(());
    }

    // 默认：尝试原生 webview。
    match webview::open_webview(options, &url, url.clone(), |proxy| {
        if options.watch {
            spawn_watcher(
                root.clone(),
                primary.clone(),
                kind,
                load_path.clone(),
                proxy,
            );
        }
    }) {
        Ok(()) => Ok(()),
        Err(e) => {
            // webview 初始化失败 → fallback 到外部浏览器（spec 要求）。
            eprintln!(
                "warning: native webview unavailable ({}); falling back to external browser",
                e
            );
            fallback::open_external(&load_path)?;
            run_watch_blocking(options, root, &primary, kind, &load_path);
            Ok(())
        }
    }
}

/// 在后台线程启动 watcher，通过 proxy 通知主线程 reload。
fn spawn_watcher(
    root: PathBuf,
    primary: PathBuf,
    kind: od_core::ArtifactKind,
    load_path: PathBuf,
    proxy: tao::event_loop::EventLoopProxy<ReloadEvent>,
) {
    std::thread::spawn(move || {
        let watch_root = root.clone();
        let watch_primary = primary.clone();
        let watch_load = load_path.clone();
        let watch_kind = kind;
        let watch_root2 = root.clone();
        let proxy = Arc::new(Mutex::new(proxy));
        let proxy2 = Arc::clone(&proxy);
        let result = watch::watch(&watch_root, move || {
            eprintln!("info: file changed under {}", watch_root2.display());
            // Markdown 主文件变更时重新渲染临时 HTML。
            if matches!(watch_kind, od_core::ArtifactKind::Markdown) {
                if let Ok(html) = render_markdown(&watch_primary, &watch_root2) {
                    let _ = std::fs::write(&watch_load, html);
                }
            }
            // 通知 webview reload。
            let reload_url = path_to_file_url(&load_path);
            let _ = proxy2.lock().unwrap().send_event(ReloadEvent(reload_url));
        });
        if let Err(e) = result {
            eprintln!("warning: watch disabled: {}", e);
        }
        let _ = proxy; // 保持 proxy 活跃
    });
}

/// fallback 模式下的监听：watcher 阻塞当前线程（浏览器需手动刷新）。
fn run_watch_blocking(
    options: &PreviewOptions,
    root: &std::path::Path,
    primary: &std::path::Path,
    kind: od_core::ArtifactKind,
    load_path: &std::path::Path,
) {
    if !options.watch {
        return;
    }
    let watch_primary = primary.to_path_buf();
    let watch_load = load_path.to_path_buf();
    let watch_root = root.to_path_buf();
    let watch_kind = kind;
    let watch_root2 = watch_root.clone();
    let result = watch::watch(root, move || {
        eprintln!(
            "info: file changed under {} — reload your browser",
            watch_root.display()
        );
        if matches!(watch_kind, od_core::ArtifactKind::Markdown) {
            if let Ok(html) = render_markdown(&watch_primary, &watch_root2) {
                let _ = std::fs::write(&watch_load, html);
            }
        }
    });
    if let Err(e) = result {
        eprintln!("warning: watch disabled: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_options_new_defaults() {
        let opts = PreviewOptions::new("/tmp/x");
        assert!(!opts.external_browser);
        assert!(opts.watch);
        assert!(!opts.devtools);
    }

    #[test]
    fn preview_missing_dir_is_artifact_not_found() {
        let opts = PreviewOptions::new("/nonexistent-od-preview-test-xyz");
        let err = preview(&opts).unwrap_err();
        assert_eq!(err.code(), "artifact_not_found");
    }
}
