//! 外部浏览器 fallback。
//! 触发：用户传 `--external-browser`，或 WebView 初始化失败且平台可打开系统浏览器。
//!
//! Spec: docs/specs/preview.md（外部浏览器 fallback）

use crate::PreviewError;
use std::path::Path;

/// 用系统默认浏览器打开主文件或渲染后的临时 HTML。
/// 注意：fallback 路径下 watcher 不保证自动刷新。
pub fn open_external(path: &Path) -> Result<(), PreviewError> {
    open::that(path).map_err(|e| PreviewError::FallbackFailed(format!("open: {e}")))
}
