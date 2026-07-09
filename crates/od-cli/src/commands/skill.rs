//! `odl skill`：列出内置与 workspace 覆盖的 skills，或显示某个 skill 的正文。
//!
//! Spec: docs/specs/built-in-skills.md（`odl skill` 命令）

use crate::cli::SkillAction;
use crate::output::Reporter;
use od_core::workspace::{find_workspace_root, skills_dir};
use od_core::{discover, find, Result};
use serde_json::json;

/// 内置 skills 目录（开发期用 `CARGO_MANIFEST_DIR` 定位仓库根 `skills/`）。
/// `od-cli` 位于 `<repo>/crates/od-cli`，仓库根往上两级。
fn builtin_skills_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

pub fn run(action: Option<&SkillAction>, json: bool, reporter: &Reporter) -> Result<()> {
    let builtin = builtin_skills_dir();
    let workspace = std::env::current_dir()
        .ok()
        .and_then(|cwd| find_workspace_root(&cwd))
        .map(|root| skills_dir(&root));

    let skills = discover(&builtin, workspace.as_deref());

    match action {
        None => list(&skills, json),
        Some(SkillAction::Show { name, json }) => show(&skills, name, *json, reporter),
    }
}

fn list(skills: &[od_core::skill::Skill], json: bool) -> Result<()> {
    if json {
        let arr: Vec<_> = skills
            .iter()
            .map(|s| {
                json!({
                    "name": s.name(),
                    "mode": s.front.mode,
                    "description": s.front.description,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string(&arr).unwrap_or_else(|_| "[]".into())
        );
    } else {
        if skills.is_empty() {
            println!("no skills found");
            return Ok(());
        }
        for s in skills {
            println!("{} | {} | {}", s.name(), s.front.mode, s.front.description);
        }
    }
    Ok(())
}

/// `odl skill show <name>`：输出该 skill 的 SKILL.md 正文。
fn show(
    skills: &[od_core::skill::Skill],
    name: &str,
    json: bool,
    reporter: &Reporter,
) -> Result<()> {
    let Some(skill) = find(skills, name) else {
        return Err(od_core::error::OdError::SkillNotFound(name.to_string()));
    };
    if json {
        let payload = json!({
            "name": skill.name(),
            "mode": skill.front.mode,
            "description": skill.front.description,
            "body": skill.body(),
            "root": skill.root.display().to_string(),
        });
        println!("{}", serde_json::to_string(&payload).unwrap_or_else(|_| "{}".into()));
    } else {
        println!("{}", skill.body());
    }
    let _ = reporter; // 当前实现不依赖 reporter，保留以备 warning/error 复用。
    Ok(())
}