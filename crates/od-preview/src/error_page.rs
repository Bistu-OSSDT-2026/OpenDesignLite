//! 内联错误页（自包含，不依赖 artifact assets）。
//!
//! Spec: docs/specs/preview.md（错误页）

use crate::PreviewError;

/// 渲染自包含错误 HTML，含用户可执行的下一步。
pub fn render_error_page(err: &PreviewError, artifact_path: &str) -> String {
    let hint = next_step(err);
    format!(
        "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
         <title>Preview error</title>\
         <body style=\"font-family:system-ui,-apple-system,sans-serif;padding:2rem;color:#171412;background:#fffaf3\">\
         <h1 style=\"margin-top:0\">Preview error</h1>\
         <p><strong>code:</strong> {}</p>\
         <p>{}</p>\
         <p><strong>artifact:</strong> {}</p>\
         <p style=\"margin-top:1.5rem;padding:1rem;background:#f0dfd2;border-radius:8px\"><strong>Next step:</strong> {}</p>\
         </body></html>",
        err.code(),
        err,
        artifact_path,
        hint,
    )
}

/// 按错误码给出用户可执行的下一步建议。
fn next_step(err: &PreviewError) -> &'static str {
    match err {
        PreviewError::ArtifactNotFound(_) => {
            "Check the directory path, or run `odl init <dir>` to create a workspace first."
        }
        PreviewError::PrimaryFileMissing(_) => {
            "Run `odl new <kind> .` to create the primary file (e.g. `odl new html .`)."
        }
        PreviewError::RenderFailed(_) => {
            "Check the Markdown source for syntax issues; the preview shell will retry on save."
        }
        PreviewError::WebviewFailed(_) => {
            "Try `odl preview --external-browser` to fall back to the system browser."
        }
        PreviewError::WatchFailed(_) => {
            "Preview continues without auto-refresh; save and re-run `odl preview` to pick up edits."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn error_page_contains_code_and_next_step() {
        let err = PreviewError::PrimaryFileMissing(PathBuf::from("/tmp/x/index.html"));
        let html = render_error_page(&err, "/tmp/x");
        assert!(html.contains("primary_file_missing"));
        assert!(html.contains("odl new"));
        assert!(html.contains("Next step"));
    }
}
