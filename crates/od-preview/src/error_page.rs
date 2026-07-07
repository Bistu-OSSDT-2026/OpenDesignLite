//! 内联错误页（自包含，不依赖 artifact assets）。
//!
//! Spec: docs/specs/preview.md（错误页）

use crate::PreviewError;

/// 渲染自包含错误 HTML。M1 会补齐「用户可执行的下一步」等内容。
pub fn render_error_page(err: &PreviewError, artifact_path: &str) -> String {
    format!(
        "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\">\
         <title>Preview error</title>\
         <body style=\"font-family:system-ui;padding:2rem\">\
         <h1>Preview error</h1>\
         <p><strong>code:</strong> {}</p>\
         <p>{}</p>\
         <p><strong>artifact:</strong> {}</p>\
         </body></html>",
        err.code(),
        err,
        artifact_path
    )
}
