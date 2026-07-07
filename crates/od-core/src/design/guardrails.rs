//! 生成守门契约：反 AI slop 的禁止默认，以及「产物是否用了 design kernel」的质量标记。
//!
//! 研究结论把「好看」工程化为可校验规则，而不是散落在各处的提示词。skills、handoff、
//! preview smoke check 必须复用本文件，保证禁止项与验收标记只有一个来源。
//!
//! Spec: docs/specs/design-kernel.md、docs/research/design-core-engineering-choice.md（质量门槛 / 反模式）

use super::catalog;
use super::tokens::TOKEN_PREFIX;
use super::STYLESHEET_ASSET;

/// 内联样式必须携带的标记属性，配合外部 `assets/od-design.css` 二选一。
pub const INLINE_STYLE_MARKER: &str = "data-od-design";

/// 默认路径禁止出现的 AI slop（人类可读，供 skills / handoff 直接列出）。
pub const FORBIDDEN_DEFAULTS: [&str; 8] = [
    "blue-to-purple gradient backgrounds",
    "glassmorphism / frosted blur as the primary surface",
    "decorative or randomly picked icons",
    "remote web fonts as the default typeface",
    "CDN UI kits (Bootstrap, Tailwind Play CDN, shadcn/ui, Material) as the visual base",
    "per-section invented colors instead of the shared token scale",
    "React / Vue runtime just to render a static artifact",
    "remote scripts or images required to open the file",
];

/// 判定 artifact 主文件是否接入了 design kernel 样式来源（外链或内联标记）。
///
/// 用于 preview / smoke check 的「引用 assets/od-design.css 或内联 data-od-design」自动门槛。
pub fn references_stylesheet(source: &str) -> bool {
    source.contains(STYLESHEET_ASSET) || source.contains(INLINE_STYLE_MARKER)
}

/// 判定源码是否使用了 design language（`--od-*` token 或任一 `od-*` primitive/recipe/pattern）。
pub fn uses_design_language(source: &str) -> bool {
    source.contains(TOKEN_PREFIX) || catalog::all_classes().any(|class| source.contains(class))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stylesheet_reference_accepts_external_or_inline() {
        assert!(references_stylesheet(
            "<link rel=\"stylesheet\" href=\"assets/od-design.css\" />"
        ));
        assert!(references_stylesheet("<style data-od-design>/* … */</style>"));
        assert!(!references_stylesheet("<style>body{}</style>"));
    }

    #[test]
    fn design_language_detects_token_or_class() {
        assert!(uses_design_language("color: var(--od-text-primary);"));
        assert!(uses_design_language("<main class=\"od-artifact\">"));
        assert!(!uses_design_language("<main class=\"wrapper\">"));
    }

    #[test]
    fn forbidden_defaults_are_non_empty() {
        assert!(FORBIDDEN_DEFAULTS.iter().all(|item| !item.is_empty()));
    }
}
