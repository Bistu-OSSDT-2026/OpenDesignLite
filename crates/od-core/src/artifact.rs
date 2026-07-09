//! Artifact 领域类型、kind↔主文件映射与主文件检测顺序。
//!
//! Spec: docs/specs/artifact-workspace.md

use crate::error::{OdError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Html,
    Markdown,
    Slides,
}

impl ArtifactKind {
    /// 用户输入 slug → kind。未知 slug 返回 `ArtifactKindUnknown`。
    pub fn from_slug(value: &str) -> Result<Self> {
        match value {
            "html" | "html-page" => Ok(Self::Html),
            "docs" | "markdown" | "md" => Ok(Self::Markdown),
            "slides" | "ppt" | "deck" => Ok(Self::Slides),
            other => Err(OdError::ArtifactKindUnknown(other.to_string())),
        }
    }

    /// 规范 slug（写入 manifest.kind）。
    pub fn slug(self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::Markdown => "docs",
            Self::Slides => "slides",
        }
    }

    pub fn primary_file(self) -> &'static str {
        match self {
            Self::Html => "index.html",
            Self::Markdown => "doc.md",
            Self::Slides => "slides.html",
        }
    }
}

/// 主文件检测顺序（集成契约）：index.html → slides.html → doc.md。
///
/// Spec: docs/specs/artifact-workspace.md（Kind 与主文件）
pub const PRIMARY_FILE_ORDER: [&str; 3] = ["index.html", "slides.html", "doc.md"];

/// 由主文件名反推 kind（与 `PRIMARY_FILE_ORDER` 对应）。
pub fn detect_kind(primary_file: &str) -> Option<ArtifactKind> {
    match primary_file {
        "index.html" => Some(ArtifactKind::Html),
        "slides.html" => Some(ArtifactKind::Slides),
        "doc.md" => Some(ArtifactKind::Markdown),
        _ => None,
    }
}

/// 一个可预览、可交接、可导出的产物目录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub kind: ArtifactKind,
    pub root: PathBuf,
}

impl Artifact {
    pub fn new(kind: ArtifactKind, root: impl Into<PathBuf>) -> Self {
        Self {
            kind,
            root: root.into(),
        }
    }

    pub fn primary_path(&self) -> PathBuf {
        self.root.join(self.kind.primary_file())
    }

    pub fn handoff_path(&self) -> PathBuf {
        self.root.join("handoff.md")
    }

    pub fn manifest_path(&self) -> PathBuf {
        self.root.join("manifest.json")
    }

    pub fn assets_dir(&self) -> PathBuf {
        self.root.join("assets")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_roundtrip() {
        assert_eq!(
            ArtifactKind::from_slug("md").unwrap(),
            ArtifactKind::Markdown
        );
        assert_eq!(
            ArtifactKind::from_slug("deck").unwrap(),
            ArtifactKind::Slides
        );
        assert!(ArtifactKind::from_slug("pdf").is_err());
    }

    #[test]
    fn primary_files_match_detection() {
        for kind in [
            ArtifactKind::Html,
            ArtifactKind::Markdown,
            ArtifactKind::Slides,
        ] {
            assert_eq!(detect_kind(kind.primary_file()), Some(kind));
        }
    }
}
