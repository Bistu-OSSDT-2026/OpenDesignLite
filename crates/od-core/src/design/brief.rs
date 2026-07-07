//! Visual brief：生成时的视觉方向约束。
//!
//! Spec: docs/specs/design-kernel.md（Visual Brief）

use crate::artifact::ArtifactKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualBrief {
    Editorial,
    Studio,
    Workbench,
}

impl VisualBrief {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "editorial" => Some(Self::Editorial),
            "studio" => Some(Self::Studio),
            "workbench" => Some(Self::Workbench),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Editorial => "editorial",
            Self::Studio => "studio",
            Self::Workbench => "workbench",
        }
    }

    /// 全部 brief（供 CLI/skills 列举，顺序稳定）。
    pub const ALL: [Self; 3] = [Self::Editorial, Self::Studio, Self::Workbench];

    /// 每类 artifact 的默认 brief（对齐 built-in-skills spec）。
    pub fn default_for(kind: ArtifactKind) -> Self {
        match kind {
            ArtifactKind::Html | ArtifactKind::Markdown => Self::Editorial,
            ArtifactKind::Slides => Self::Studio,
        }
    }

    /// 一句话视觉方向，让同一内核能切换题材气质（对齐 design-kernel spec 的 brief 表）。
    pub fn direction(self) -> &'static str {
        match self {
            Self::Editorial => "warm paper canvas, strong typography, restrained borders",
            Self::Studio => "generous whitespace, image-first, quiet surfaces",
            Self::Workbench => "clear hierarchy, compact controls, low-contrast background",
        }
    }

    /// 该 brief 推荐优先使用的 pattern class（来自 [`super::catalog::PATTERNS`]）。
    /// 是生成偏好而非强制模板，skills 可再自由组合 primitive。
    pub fn recommended_patterns(self) -> &'static [&'static str] {
        match self {
            Self::Editorial => &["od-doc", "od-hero", "od-artifact"],
            Self::Studio => &["od-hero", "od-slide", "od-artifact"],
            Self::Workbench => &["od-dashboard", "od-artifact"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brief_roundtrip() {
        for b in [
            VisualBrief::Editorial,
            VisualBrief::Studio,
            VisualBrief::Workbench,
        ] {
            assert_eq!(VisualBrief::parse(b.as_str()), Some(b));
        }
        assert_eq!(VisualBrief::parse("instrument"), None);
    }

    #[test]
    fn slides_default_is_studio() {
        assert_eq!(
            VisualBrief::default_for(ArtifactKind::Slides),
            VisualBrief::Studio
        );
    }

    #[test]
    fn recommended_patterns_are_defined_catalog_patterns() {
        for brief in VisualBrief::ALL {
            assert!(!brief.direction().is_empty());
            let patterns = brief.recommended_patterns();
            assert!(!patterns.is_empty());
            for pattern in patterns {
                assert!(
                    crate::design::catalog::PATTERNS.contains(pattern),
                    "{pattern} not in catalog"
                );
            }
        }
    }
}
