//! Markdown 渲染管线：comrak render → ammonia clean → 包 HTML 外壳。
//!
//! M1 对齐 od-core design kernel：inline `css_for(VisualBrief)` 替代硬编码样式，
//! body 用 `.od-container.od-doc` 语义类包裹，让 Markdown 预览与 HTML/Slides 产物
//! 共用同一套 `--od-*` token + `@layer` 分层。
//!
//! Spec: docs/specs/preview.md（Markdown 渲染流程）

use crate::PreviewError;
use od_core::design::{css_for, VisualBrief};
use std::fs;
use std::path::Path;

/// 渲染 `doc.md` 为可被 WebView 加载的 HTML 字符串。
///
/// 流程：读源 → comrak 转 HTML 片段 → ammonia 清洗危险标签 → 包页面外壳。
/// `brief` 决定内联的 design kernel 样式表(Editorial/Studio/Workbench)。
/// 安全：Markdown 里写的原始 HTML（含 `<script>`）会被 ammonia 删掉。
pub fn render_markdown(doc: &Path, brief: VisualBrief) -> Result<String, PreviewError> {
    let md = fs::read_to_string(doc)
        .map_err(|e| PreviewError::RenderFailed(format!("read {}: {e}", doc.display())))?;

    let fragment = comrak::markdown_to_html(&md, &comrak::Options::default());
    let safe = ammonia::clean(&fragment);
    let design_css = css_for(brief);

    // 外壳：inline design kernel CSS，自包含、可离线打开。
    // `.od-container` 提供统一最大宽度与居中，`.od-doc` 为 Markdown 专属阅读列宽。
    let html = format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>Preview</title>\n  <style>\n{design_css}\n  </style>\n</head>\n\
         <body>\n  <div class=\"od-container od-doc\">\n{safe}\n  </div>\n</body>\n</html>"
    );

    Ok(html)
}
