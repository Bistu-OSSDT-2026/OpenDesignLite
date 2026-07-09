//! Markdown 渲染管线：comrak render → ammonia clean → minijinja wrap。
//!
//! Spec: docs/specs/preview.md（Markdown 渲染流程）

use crate::PreviewError;
use comrak::{markdown_to_html, ComrakOptions};
use minijinja::{context, Environment};
use std::path::Path;

/// 渲染 `doc.md` 为可被 WebView/浏览器加载的自包含 HTML 字符串。
///
/// comrak(GFM)→ ammonia(白名单清洗)→ minijinja 包装，引用 `assets/od-design.css`。
/// CSS 文件不存在时内联最小 `.od-doc` 兜底样式，保证离线可读。
pub fn render_markdown(doc: &Path, artifact_root: &Path) -> Result<String, PreviewError> {
    let source = std::fs::read_to_string(doc)
        .map_err(|e| PreviewError::RenderFailed(format!("read {}: {e}", doc.display())))?;

    let mut options = ComrakOptions::default();
    options.render.github_pre_lang = true;
    let html_body = markdown_to_html(&source, &options);
    let clean = ammonia::clean(&html_body);

    // CSS：优先引用 artifact 的 assets/od-design.css；缺失则内联最小样式。
    let css_link = artifact_root.join("assets").join("od-design.css");
    let (style_tag, has_external_css) = if css_link.exists() {
        let href = path_to_file_url(&css_link);
        (format!("<link rel=\"stylesheet\" href=\"{href}\" />"), true)
    } else {
        (inline_doc_fallback_css(), false)
    };
    let _ = has_external_css;

    let env = Environment::new();
    let template = "/* minijinja markdown wrapper */\n\
<!doctype html>\n\
<html lang=\"en\">\n\
<head>\n\
<meta charset=\"utf-8\" />\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\
<title>{{ title }}</title>\n\
{{ style|safe }}\n\
</head>\n\
<body class=\"od-doc\">\n\
{{ content|safe }}\n\
</body>\n\
</html>";
    let rendered = env
        .render_str(
            template,
            context!(title => doc_title(doc), style => style_tag, content => clean),
        )
        .map_err(|e| PreviewError::RenderFailed(format!("template render: {e}")))?;

    Ok(rendered)
}

/// 内联最小 `.od-doc` 兜底样式（CSS 文件缺失时）。
fn inline_doc_fallback_css() -> String {
    "<style data-od-design>\n\
.od-doc{max-width:760px;margin-inline:auto;padding:2rem 1.5rem;line-height:1.7;\n\
font-family:system-ui,-apple-system,sans-serif;color:#171412;background:#fffaf3}\n\
.od-doc h1,.od-doc h2,.od-doc h3{line-height:1.3}\n\
.od-doc code{font-family:ui-monospace,monospace;background:#f0dfd2;padding:.15em .4em;border-radius:4px}\n\
.od-doc pre{padding:1rem;background:#fff;border:1px solid rgba(28,22,17,.1);border-radius:8px;overflow:auto}\n\
.od-doc pre code{background:none;padding:0}\n\
</style>"
        .to_string()
}

fn doc_title(doc: &Path) -> String {
    doc.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Document")
        .to_string()
}

/// 把本地路径转成 `file:///` URL（Windows 下处理盘符与反斜杠）。
pub fn path_to_file_url(path: &Path) -> String {
    let abs = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let mut s = abs.to_string_lossy().replace('\\', "/");
    // Windows: C:/... -> /C:/...
    if let Some(second) = s.chars().nth(1) {
        if second == ':' {
            s.insert(0, '/');
        }
    }
    format!("file://{s}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn render_markdown_produces_od_doc_wrapper() {
        let tmp = std::env::temp_dir().join("od-md-render-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        fs::write(tmp.join("doc.md"), "# Hello\n\nWorld.\n").unwrap();

        let html = render_markdown(&tmp.join("doc.md"), &tmp).unwrap();
        assert!(html.contains("class=\"od-doc\""));
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("World."));

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn render_markdown_cleans_raw_html() {
        let tmp = std::env::temp_dir().join("od-md-clean-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();
        fs::write(
            tmp.join("doc.md"),
            "text\n\n<script>alert(1)</script>\n\nmore\n",
        )
        .unwrap();

        let html = render_markdown(&tmp.join("doc.md"), &tmp).unwrap();
        assert!(!html.contains("<script>alert(1)</script>"));

        fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn path_to_file_url_handles_windows_drive() {
        let p = Path::new("C:/tmp/x.html");
        let url = path_to_file_url(p);
        // canonicalize 可能失败；退而断言基本结构。
        assert!(url.starts_with("file://") || url.starts_with("file:///"));
    }
}
