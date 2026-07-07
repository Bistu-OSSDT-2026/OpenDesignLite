//! `odl init`：创建 workspace。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md

use od_core::workspace::{artifacts_dir, skills_dir};
use od_core::{workspace_manifest_path, Result};
use std::fs;
use std::path::Path;

/// M0 行为保留：创建 `artifacts/`、`skills/` 与最小 workspace manifest。
/// `name` / `force` 的完整语义在 M1 补齐（当前 name 未写入 manifest）。
pub fn run(root: &Path, _name: Option<&str>, _force: bool) -> Result<()> {
    fs::create_dir_all(artifacts_dir(root))?;
    fs::create_dir_all(skills_dir(root))?;

    let manifest = workspace_manifest_path(root);
    if !manifest.exists() {
        fs::write(
            &manifest,
            "{\n  \"schemaVersion\": 1,\n  \"type\": \"workspace\",\n  \"name\": \"Open Design Lite Workspace\"\n}\n",
        )?;
    }
    Ok(())
}
