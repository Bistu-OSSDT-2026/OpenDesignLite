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

    /// 每类 artifact 的默认 brief（对齐 built-in-skills spec）。
    pub fn default_for(kind: ArtifactKind) -> Self {
        match kind {
            ArtifactKind::Html | ArtifactKind::Markdown => Self::Editorial,
            ArtifactKind::Slides => Self::Studio,
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
}
