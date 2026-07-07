//! 内置技能模型：`SKILL.md` front matter。技能是文件系统目录，不是硬编码逻辑。
//!
//! Spec: docs/specs/built-in-skills.md

use crate::artifact::ArtifactKind;
use crate::error::Result;

/// `SKILL.md` front matter（M1 只要求 name / mode / description，额外字段忽略）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontMatter {
    pub name: String,
    /// `html` | `docs` | `slides`。
    pub mode: String,
    pub description: String,
    pub template: Option<String>,
    pub visual_brief: Option<String>,
}

impl SkillFrontMatter {
    /// mode → artifact kind 映射。
    pub fn kind(&self) -> Option<ArtifactKind> {
        match self.mode.as_str() {
            "html" => Some(ArtifactKind::Html),
            "docs" => Some(ArtifactKind::Markdown),
            "slides" => Some(ArtifactKind::Slides),
            _ => None,
        }
    }

    /// 解析 `SKILL.md` 顶部 YAML front matter。M1 实现。
    pub fn parse(_source: &str) -> Result<Self> {
        todo!("M1: parse YAML front matter; see docs/specs/built-in-skills.md")
    }
}
