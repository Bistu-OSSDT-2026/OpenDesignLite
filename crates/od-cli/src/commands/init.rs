//! `odl init`：创建 workspace（artifacts/ + skills/ + workspace manifest）。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md

use od_core::manifest::{save_workspace, WorkspaceManifest};
use od_core::workspace::{artifacts_dir, skills_dir, workspace_manifest_path};
use od_core::{OdError, Result};
use std::fs;
use std::path::Path;

const DEFAULT_WORKSPACE_NAME: &str = "Open Design Lite Workspace";

pub fn run(root: &Path, name: Option<&str>, force: bool) -> Result<()> {
    fs::create_dir_all(root)?;
    fs::create_dir_all(artifacts_dir(root))?;
    fs::create_dir_all(skills_dir(root))?;

    let manifest_path = workspace_manifest_path(root);
    if manifest_path.exists() && !force {
        return Err(OdError::AlreadyExists(manifest_path));
    }

    let manifest = WorkspaceManifest::new(name.unwrap_or(DEFAULT_WORKSPACE_NAME));
    save_workspace(&manifest, &manifest_path)?;
    Ok(())
}
