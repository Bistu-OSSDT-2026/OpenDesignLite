//! 外部浏览器 fallback。
//! 触发：用户传 `--external-browser`，或 WebView 初始化失败且平台可打开系统浏览器。
//!
//! Spec: docs/specs/preview.md（外部浏览器 fallback）

use crate::PreviewError;
use std::path::Path;

/// 用系统浏览器打开主文件或渲染后的临时 HTML。
pub fn open_external(path: &Path) -> Result<(), PreviewError> {
    open::that(path).map_err(|e| {
        PreviewError::WebviewFailed(format!(
            "external browser failed to open {}: {e}",
            path.display()
        ))
    })
}
