//! `odl init`：创建 workspace。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md

use od_core::manifest::{WorkspaceManifest, SCHEMA_VERSION};
use od_core::workspace::{artifacts_dir, skills_dir};
use od_core::{workspace_manifest_path, Result};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// M0 行为保留：创建 `artifacts/`、`skills/` 与最小 workspace manifest。
/// `name` / `force` 的完整语义在 M1 补齐（当前 name 未写入 manifest）。
pub fn run(root: &Path, _name: Option<&str>, _force: bool) -> Result<()> {
    fs::create_dir_all(artifacts_dir(root))?;
    fs::create_dir_all(skills_dir(root))?;

    let manifest = workspace_manifest_path(root);
    if !manifest.exists() {
        let manifest = WorkspaceManifest {
            schema_version: SCHEMA_VERSION,
            r#type: "workspace".to_string(),
            name: "Open Design Lite Workspace".to_string(),
            created_by: "odl".to_string(),
            created_at: created_at(),
        };
        fs::write(
            workspace_manifest_path(root),
            serde_json::to_string_pretty(&manifest).expect("workspace manifest serializes"),
        )?;
    }
    Ok(())
}

fn created_at() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}
