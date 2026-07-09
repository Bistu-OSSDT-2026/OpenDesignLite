//! `odl new`：创建 artifact（主文件 + handoff）。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md, handoff.md, built-in-skills.md

use od_core::{create, Artifact, Result};
use std::path::Path;

pub struct NewOptions<'a> {
    pub title: Option<&'a str>,
    pub brief: &'a str,
    pub embed_css: bool,
    pub force: bool,
}

pub struct NewResult {
    pub artifact: Artifact,
    pub warnings: Vec<String>,
}

pub fn run(kind_slug: &str, root: &Path, options: NewOptions<'_>) -> Result<NewResult> {
    let result = create::run(
        kind_slug,
        root,
        create::CreateOptions {
            title: options.title,
            visual_brief: options.brief,
            embed_css: options.embed_css,
            overwrite: options.force,
        },
    )?;
    let artifact = result.artifact;
    let warnings = result.warnings;
    Ok(NewResult { artifact, warnings })
}
