//! Design kernel 语义层（非组件库）。只提供 `--od-*` token、visual brief 与命名常量。
//! 不引入 React / Tailwind / Radix / shadcn/ui / Web Components runtime。
//!
//! Spec: docs/specs/design-kernel.md

pub mod brief;
pub mod catalog;
pub mod guardrails;
pub mod stylesheet;
pub mod tokens;

pub use brief::VisualBrief;
pub use catalog::{ClassKind, PATTERNS, PRIMITIVES, RECIPES};
pub use stylesheet::css_for;

/// design kernel 版本，写入 artifact manifest 的 `design.kernelVersion`。
pub const KERNEL_VERSION: u32 = 1;

/// 默认样式资产（相对 artifact root）。
pub const STYLESHEET_ASSET: &str = "assets/od-design.css";

/// 可选 token JSON 资产。
pub const TOKENS_ASSET: &str = "design-tokens.json";
