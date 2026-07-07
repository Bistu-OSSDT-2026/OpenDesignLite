//! WebView 生命周期（wry + 事件循环）。签名占位。
//! 安全规则：预览页面不得获得 shell / 文件系统 / 命令执行 IPC。
//!
//! Spec: docs/specs/preview.md（加载策略 / 安全规则）

use crate::{PreviewError, PreviewOptions};

/// 打开 WebView 并加载 `url`（M1 为 `file://` 主文件或渲染后的临时 HTML）。
pub fn open_webview(_options: &PreviewOptions, _url: &str) -> Result<(), PreviewError> {
    todo!("M1: wry webview + event loop; see docs/specs/preview.md")
}
