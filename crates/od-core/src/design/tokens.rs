//! `--od-*` token 命名约定与 `od-design.css` 的 @layer 顺序（契约，非实现）。
//! 实际 CSS 由 templates/ 手写，本文件只固定命名规则供校验与文档引用。
//!
//! Spec: docs/specs/design-kernel.md（CSS 变量 / CSS 分层）

/// 对外 CSS 变量前缀。任何进入 core 的 token 必须以此为前缀。
pub const TOKEN_PREFIX: &str = "--od-";

/// `od-design.css` 的 @layer 顺序（集成契约）。
pub const CSS_LAYERS: [&str; 7] = [
    "od.reset",
    "od.tokens",
    "od.base",
    "od.primitives",
    "od.recipes",
    "od.patterns",
    "od.utilities",
];
