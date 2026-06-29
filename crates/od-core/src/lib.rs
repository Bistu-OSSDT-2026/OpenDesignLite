use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Html,
    Markdown,
    Slides,
}

impl ArtifactKind {
    pub fn from_slug(value: &str) -> Option<Self> {
        match value {
            "html" | "html-page" => Some(Self::Html),
            "docs" | "markdown" | "md" => Some(Self::Markdown),
            "slides" | "ppt" | "deck" => Some(Self::Slides),
            _ => None,
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
}

pub fn workspace_manifest_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("manifest.json")
}
