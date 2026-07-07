//! 工作区目录布局辅助。
//!
//! Spec: docs/specs/artifact-workspace.md（工作区布局）

use std::path::{Path, PathBuf};

pub fn workspace_manifest_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("manifest.json")
}

pub fn artifacts_dir(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("artifacts")
}

pub fn skills_dir(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("skills")
}
