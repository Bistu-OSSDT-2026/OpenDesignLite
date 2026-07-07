//! `odl new`：创建 artifact（主文件 + handoff）。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md, handoff.md

use od_core::{handoff, Artifact, ArtifactKind, OdError, Result};
use std::fs;
use std::path::Path;

/// M0 行为保留：写主文件 + 最小 handoff。
/// M1 补齐：manifest.json、assets/od-design.css、完整 handoff 渲染、`--title/--brief/--embed-css`。
pub fn run(kind_slug: &str, root: &Path) -> Result<Artifact> {
    let kind = ArtifactKind::from_slug(kind_slug)?;
    fs::create_dir_all(root)?;

    let artifact = Artifact::new(kind, root);
    let primary = artifact.primary_path();
    if primary.exists() {
        return Err(OdError::AlreadyExists(primary));
    }

    fs::write(&primary, starter_content(kind))?;
    fs::write(artifact.handoff_path(), handoff::minimal_stub())?;
    Ok(artifact)
}

fn starter_content(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Html => include_str!("../../../../templates/html-page/basic.html"),
        ArtifactKind::Markdown => "# Draft\n\nStart writing here.\n",
        ArtifactKind::Slides => include_str!("../../../../templates/slides/basic.html"),
    }
}
