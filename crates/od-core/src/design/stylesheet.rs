//! Default `od-design.css` implementation for generated artifacts.
//!
//! This is still a static, framework-free asset. Core owns it so every
//! consumer writes the same token and class contract to disk.

use super::brief::VisualBrief;
use crate::ArtifactKind;

/// Return the default stylesheet for a visual brief.
pub fn css_for(brief: VisualBrief) -> String {
    let tokens = match brief {
        VisualBrief::Editorial => EDITORIAL_TOKENS,
        VisualBrief::Studio => STUDIO_TOKENS,
        VisualBrief::Workbench => WORKBENCH_TOKENS,
    };
    format!("{tokens}\n{BASE_CSS}")
}

/// Kind 感知版本：slides 追加 16:9 固定纸张打印规则，其余 kind 与
/// `css_for` 一致（`@page` 是全局规则，混进共享 CSS 会波及 html/docs
/// 的 PDF 纸张，因此只对 slides 产物发出）。
///
/// Spec: docs/specs/export.md（Slides PDF 规则）
pub fn css_for_kind(brief: VisualBrief, kind: ArtifactKind) -> String {
    match kind {
        ArtifactKind::Slides => format!("{}\n{SLIDES_PRINT_CSS}", css_for(brief)),
        _ => css_for(brief),
    }
}

/// Slides 打印规则：每页精确 13.333in × 7.5in（16:9）。
///
/// 纸张尺寸唯一可靠的控制方式是 CSS `@page`——Chrome/Edge 的 headless CLI
/// 没有稳定的纸张尺寸开关（`--print-to-pdf-page-size` 不存在，那是
/// DevTools Protocol 的参数）。`.od-slide` 打印时锁定整页尺寸并
/// `overflow: hidden`，杜绝溢出触发 Blink 自动分页；末页取消 break
/// 避免尾部空白页。规则不入 @layer：未分层样式天然胜过分层样式，
/// 保证覆盖 `od.patterns` 里的 `.od-slide { min-height: 100vh }`。
const SLIDES_PRINT_CSS: &str = r#"/* Slides print: fixed 16:9 pages (docs/specs/export.md). */
@page { size: 13.333in 7.5in; margin: 0; }
@media print {
  .od-slide {
    min-height: unset;
    width: 13.333in;
    height: 7.5in;
    margin: 0;
    overflow: hidden;
    break-after: page;
    page-break-after: always;
  }
  .od-slide:last-of-type { break-after: auto; page-break-after: auto; }
}
"#;

const BASE_CSS: &str = r#"@layer od.reset, od.tokens, od.base, od.primitives, od.recipes, od.patterns, od.utilities;

@layer od.reset {
  *, *::before, *::after { box-sizing: border-box; }
  html { color-scheme: light; }
  body { margin: 0; min-height: 100vh; }
  img, svg, video { max-width: 100%; height: auto; }
}

@layer od.base {
  body {
    background: var(--od-bg-canvas);
    color: var(--od-text-primary);
    font-family: var(--od-font-sans);
    line-height: 1.6;
  }
  a { color: var(--od-accent-solid); }
  h1, h2, h3 { line-height: 1.08; letter-spacing: -0.03em; }
}

@layer od.primitives {
  .od-container { width: min(100% - 48px, 960px); margin-inline: auto; }
  .od-stack { display: grid; gap: var(--od-space-6); }
  .od-inline, .od-cluster { display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; }
  .od-grid { display: grid; gap: var(--od-space-6); grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); }
  .od-section { padding-block: clamp(3rem, 8vw, 7rem); }
  .od-split { display: grid; gap: var(--od-space-6); grid-template-columns: minmax(0, 1.1fr) minmax(240px, 0.9fr); }
  .od-frame { border: 1px solid color-mix(in srgb, var(--od-text-primary), transparent 86%); border-radius: var(--od-radius-lg); overflow: hidden; }
}

@layer od.recipes {
  .od-button { display: inline-flex; align-items: center; justify-content: center; min-height: 2.75rem; padding: 0 1rem; border-radius: 999px; background: var(--od-accent-solid); color: var(--od-accent-contrast); text-decoration: none; font-weight: 700; }
  .od-card { padding: var(--od-space-6); border-radius: var(--od-radius-lg); background: var(--od-bg-surface); box-shadow: 0 18px 50px color-mix(in srgb, var(--od-text-primary), transparent 90%); }
  .od-input { width: 100%; border: 1px solid color-mix(in srgb, var(--od-text-primary), transparent 80%); border-radius: 0.75rem; padding: 0.75rem 0.875rem; background: var(--od-bg-surface); color: var(--od-text-primary); }
  .od-badge { display: inline-flex; border-radius: 999px; padding: 0.25rem 0.65rem; background: color-mix(in srgb, var(--od-accent-solid), transparent 86%); color: var(--od-accent-solid); font-size: 0.8rem; font-weight: 700; }
  .od-table { width: 100%; border-collapse: collapse; }
  .od-table th, .od-table td { padding: 0.75rem; border-bottom: 1px solid color-mix(in srgb, var(--od-text-primary), transparent 86%); text-align: left; }
  .od-empty { padding: var(--od-space-6); border: 1px dashed color-mix(in srgb, var(--od-text-secondary), transparent 60%); border-radius: var(--od-radius-lg); color: var(--od-text-secondary); }
}

@layer od.patterns {
  .od-artifact { min-height: 100vh; }
  .od-doc { max-width: 760px; }
  .od-slide { min-height: 100vh; display: grid; place-items: center; padding: clamp(2rem, 7vw, 5rem); }
  .od-hero { padding-block: clamp(4rem, 10vw, 8rem); }
  .od-dashboard { display: grid; gap: var(--od-space-6); grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); }
}

@layer od.utilities {
  .od-muted { color: var(--od-text-secondary); }
}

@media (max-width: 720px) {
  .od-split { grid-template-columns: 1fr; }
  .od-container { width: min(100% - 32px, 960px); }
}
"#;

const EDITORIAL_TOKENS: &str = r#"@layer od.tokens {
  :root {
    --od-bg-canvas: #f7f3ed;
    --od-bg-surface: #fffaf3;
    --od-text-primary: #171412;
    --od-text-secondary: #5f5750;
    --od-accent-solid: #8f5b3f;
    --od-accent-contrast: #fffaf3;
    --od-space-6: 1.5rem;
    --od-radius-lg: 1rem;
    --od-font-sans: Arial, sans-serif;
  }
}
"#;

const STUDIO_TOKENS: &str = r#"@layer od.tokens {
  :root {
    --od-bg-canvas: #111318;
    --od-bg-surface: #1b1f29;
    --od-text-primary: #f4f0e8;
    --od-text-secondary: #bbb4a8;
    --od-accent-solid: #d8a24a;
    --od-accent-contrast: #17120a;
    --od-space-6: 1.5rem;
    --od-radius-lg: 1rem;
    --od-font-sans: Arial, sans-serif;
  }
}
"#;

const WORKBENCH_TOKENS: &str = r#"@layer od.tokens {
  :root {
    --od-bg-canvas: #f4f7f8;
    --od-bg-surface: #ffffff;
    --od-text-primary: #142026;
    --od-text-secondary: #52616b;
    --od-accent-solid: #1b6f8f;
    --od-accent-contrast: #ffffff;
    --od-space-6: 1.5rem;
    --od-radius-lg: 1rem;
    --od-font-sans: Arial, sans-serif;
  }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::catalog;
    use crate::design::tokens::{CSS_LAYERS, TOKEN_PREFIX};

    #[test]
    fn stylesheet_consumes_core_contract() {
        let css = css_for(VisualBrief::Editorial);

        assert!(css.contains(TOKEN_PREFIX));
        for layer in CSS_LAYERS {
            assert!(css.contains(layer), "missing layer: {layer}");
        }
        for class in catalog::all_classes() {
            assert!(css.contains(&format!(".{class}")), "missing class: {class}");
        }
    }

    /// Slides PDF 16:9 契约（export.md）：slides 带 `@page` 固定纸张与分页
    /// 规则；html/docs 不带（`@page` 会波及它们的 PDF 纸张）。
    #[test]
    fn slides_css_carries_print_page_rules() {
        let slides = css_for_kind(VisualBrief::Studio, ArtifactKind::Slides);
        assert!(slides.contains("@page { size: 13.333in 7.5in; margin: 0; }"));
        assert!(slides.contains("break-after: page"));
        assert!(slides.contains(".od-slide:last-of-type"));

        for kind in [ArtifactKind::Html, ArtifactKind::Markdown] {
            let css = css_for_kind(VisualBrief::Studio, kind);
            assert!(!css.contains("@page"), "{kind:?} must not carry @page");
        }
    }
}
