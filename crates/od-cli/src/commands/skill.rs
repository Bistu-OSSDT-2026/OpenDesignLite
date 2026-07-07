//! `odl skill`：列出可用内置/工作区 skill。
//!
//! Spec: docs/specs/built-in-skills.md

use od_core::skill::{self, Skill};
use std::path::{Path, PathBuf};

pub fn list(cwd: &Path) -> Vec<Skill> {
    let builtin = builtin_skills_dir();
    let workspace = workspace_skills_dir(cwd);
    skill::discover(&builtin, workspace.as_deref())
}

pub fn builtin_skills_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

pub fn workspace_skills_dir(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        if ancestor.join("manifest.json").exists() {
            return Some(ancestor.join("skills"));
        }
    }
    None
}
