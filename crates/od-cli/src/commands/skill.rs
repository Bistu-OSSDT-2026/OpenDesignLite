//! `odl skill`：列出可用内置/工作区 skill，或输出某个 skill 的正文。
//!
//! Spec: docs/specs/built-in-skills.md

use crate::cli::SkillAction;
use od_core::skill::{self, Skill};
use od_core::{OdError, Result};
use std::path::{Path, PathBuf};

pub fn list(cwd: &Path) -> Vec<Skill> {
    let builtin = builtin_skills_dir();
    let workspace = workspace_skills_dir(cwd);
    skill::discover(&builtin, workspace.as_deref())
}

pub fn show(cwd: &Path, name: &str) -> Result<Skill> {
    let skills = list(cwd);
    skill::find(&skills, name)
        .cloned()
        .ok_or_else(|| OdError::SkillNotFound(name.to_string()))
}

pub fn wants_json(global_json: bool, local_json: bool, action: Option<&SkillAction>) -> bool {
    global_json || local_json || matches!(action, Some(SkillAction::Show { json: true, .. }))
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
