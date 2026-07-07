//! `odl handoff`：生成或刷新 handoff.md（M1）。
//!
//! Spec: docs/specs/cli.md, handoff.md

use od_core::artifact::detect_kind;
use od_core::handoff::{self, HandoffAgent};
use od_core::{Artifact, OdError, Result};
use std::fs;
use std::path::Path;

pub fn run(root: &Path, agent: &str, stdout: bool) -> Result<Option<String>> {
    let agent =
        HandoffAgent::parse(agent).ok_or_else(|| OdError::ArtifactKindUnknown(agent.into()))?;
    let artifact = artifact_from_root(root)?;
    let rendered = handoff::render(&artifact, agent);

    if stdout {
        Ok(Some(rendered))
    } else {
        fs::write(artifact.handoff_path(), rendered)?;
        Ok(None)
    }
}

fn artifact_from_root(root: &Path) -> Result<Artifact> {
    for primary in od_core::artifact::PRIMARY_FILE_ORDER {
        let path = root.join(primary);
        if path.exists() {
            let kind = detect_kind(primary).expect("primary file order maps to kind");
            return Ok(Artifact::new(kind, root));
        }
    }
    Err(OdError::PrimaryFileMissing(root.join("index.html")))
}
