//! 外部浏览器 fallback。签名占位。
//! 触发：用户传 `--external-browser`，或 WebView 初始化失败且平台可打开系统浏览器。
//!
//! Spec: docs/specs/preview.md（外部浏览器 fallback）

use crate::PreviewError;
use std::path::Path;

/// 用系统浏览器打开主文件或渲染后的临时 HTML。M1 接入 `open` crate。
pub fn open_external(_path: &Path) -> Result<(), PreviewError> {
    todo!("M1: open crate or platform call; see docs/specs/preview.md")
}
