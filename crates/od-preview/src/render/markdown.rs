//! Markdown 渲染管线：comrak render → ammonia clean → 包 HTML 外壳。
//!
//! M1 第一版用 `format!` 包外壳（模板太简单），后续可换成 minijinja 加载
//! `markdown-preview.html` 模板并引用 `assets/od-design.css`。
//!
//! Spec: docs/specs/preview.md（Markdown 渲染流程）

use crate::PreviewError;
use std::fs;
use std::path::Path;

/// 渲染 `doc.md` 为可被 WebView 加载的 HTML 字符串。
///
/// 流程：读源 → comrak 转 HTML 片段 → ammonia 清洗危险标签 → 包页面外壳。
/// 安全：Markdown 里写的原始 HTML（含 `<script>`）会被 ammonia 删掉。
pub fn render_markdown(doc: &Path) -> Result<String, PreviewError> {
    let md = fs::read_to_string(doc)
        .map_err(|e| PreviewError::RenderFailed(format!("read {}: {e}", doc.display())))?;

    let fragment = comrak::markdown_to_html(&md, &comrak::Options::default());
    let safe = ammonia::clean(&fragment);

    // 外壳：自包含，M1 暂用内联样式，后续接 od-design.css。
    let html = format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>Preview</title>\n  <style>\n    body{{max-width:880px;margin:0 auto;padding:64px 24px;font:16px/1.6 system-ui,sans-serif;color:#171717}}\n  </style>\n</head>\n\
         <body>\n{safe}\n</body>\n</html>"
    );

    Ok(html)
}
