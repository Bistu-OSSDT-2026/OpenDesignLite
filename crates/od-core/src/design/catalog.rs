//! Design language 目录：layout primitive、component recipe、pattern 的 `od-*` class 名。
//!
//! 这是 framework-agnostic 语义契约（研究结论：recipe 名、pattern 名属于 core）。
//! `od-design.css`、templates、skills、preview 与 smoke check 必须以本文件为单一来源，
//! 不得各自硬编码字符串。本文件只固定命名，不含 CSS 实现。
//!
//! Spec: docs/specs/design-kernel.md（Layout Primitive / Component Recipe / Pattern）

/// 所有对外 class 的公共前缀（区别于 CSS 变量前缀 [`super::tokens::TOKEN_PREFIX`]）。
pub const CLASS_PREFIX: &str = "od-";

/// Layout primitive class（只表达结构，不含品牌色或业务语义）。
pub const PRIMITIVES: [&str; 8] = [
    "od-container",
    "od-stack",
    "od-inline",
    "od-cluster",
    "od-grid",
    "od-section",
    "od-split",
    "od-frame",
];

/// Component recipe class（产品质感，使用 semantic token）。
pub const RECIPES: [&str; 6] = [
    "od-button",
    "od-card",
    "od-input",
    "od-badge",
    "od-table",
    "od-empty",
];

/// Pattern class（页面组织约束，非固定模板）。
pub const PATTERNS: [&str; 5] = [
    "od-artifact",
    "od-doc",
    "od-slide",
    "od-hero",
    "od-dashboard",
];

/// design language 中一个 class 的类别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassKind {
    Primitive,
    Recipe,
    Pattern,
}

/// 是否为 design kernel 命名空间下的 class（前缀检查，不校验是否已定义）。
pub fn is_od_class(class: &str) -> bool {
    class.starts_with(CLASS_PREFIX)
}

/// 查一个 class 属于哪个类别；未定义返回 `None`。
pub fn classify(class: &str) -> Option<ClassKind> {
    if PRIMITIVES.contains(&class) {
        Some(ClassKind::Primitive)
    } else if RECIPES.contains(&class) {
        Some(ClassKind::Recipe)
    } else if PATTERNS.contains(&class) {
        Some(ClassKind::Pattern)
    } else {
        None
    }
}

/// 是否为已定义的 design language class（primitive / recipe / pattern）。
pub fn is_known_class(class: &str) -> bool {
    classify(class).is_some()
}

/// 遍历全部已定义 class（primitive → recipe → pattern 顺序）。
pub fn all_classes() -> impl Iterator<Item = &'static str> {
    PRIMITIVES
        .iter()
        .chain(RECIPES.iter())
        .chain(PATTERNS.iter())
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_classes_share_prefix_and_are_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for class in all_classes() {
            assert!(is_od_class(class), "{class} must carry od- prefix");
            assert!(seen.insert(class), "duplicate class: {class}");
        }
        assert_eq!(seen.len(), PRIMITIVES.len() + RECIPES.len() + PATTERNS.len());
    }

    #[test]
    fn classify_matches_group() {
        assert_eq!(classify("od-stack"), Some(ClassKind::Primitive));
        assert_eq!(classify("od-button"), Some(ClassKind::Recipe));
        assert_eq!(classify("od-dashboard"), Some(ClassKind::Pattern));
        assert_eq!(classify("od-unknown"), None);
        assert!(!is_known_class("od-unknown"));
    }

    #[test]
    fn is_od_class_is_prefix_only() {
        assert!(is_od_class("od-anything"));
        assert!(!is_od_class("button"));
        assert!(!is_known_class("od-anything"));
    }
}
