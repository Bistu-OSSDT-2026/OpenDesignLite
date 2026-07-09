//! Default `od-design.css` implementation for generated artifacts.
//!
//! This is still a static, framework-free asset. Core owns it so every
//! consumer writes the same token and class contract to disk.

use super::brief::VisualBrief;

/// Return the default stylesheet for a visual brief.
pub fn css_for(brief: VisualBrief) -> String {
    let tokens = match brief {
        VisualBrief::Editorial => EDITORIAL_TOKENS,
        VisualBrief::Studio => STUDIO_TOKENS,
        VisualBrief::Workbench => WORKBENCH_TOKENS,
    };
    format!("{tokens}\n{BASE_CSS}")
}

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
}
