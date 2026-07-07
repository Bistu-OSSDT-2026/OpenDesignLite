//! Markdown 渲染管线：comrak render → ammonia clean → minijinja wrap。签名占位。
//!
//! Spec: docs/specs/preview.md（Markdown 渲染流程）

use crate::PreviewError;
use std::path::Path;

/// 渲染 `doc.md` 为可被 WebView 加载的 HTML 字符串。
/// M1 接入 comrak / ammonia / minijinja，并引用 `assets/od-design.css`。
pub fn render_markdown(_doc: &Path) -> Result<String, PreviewError> {
    todo!("M1: comrak -> ammonia -> minijinja wrap; see docs/specs/preview.md")
}
