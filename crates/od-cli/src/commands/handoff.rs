//! `odl handoff`：生成或刷新 handoff.md。
//!
//! Spec: docs/specs/cli.md, handoff.md

use crate::output::Reporter;
use od_core::artifact::{detect_kind, Artifact, ArtifactKind};
use od_core::handoff::{render as render_handoff, HandoffAgent, HandoffInput};
use od_core::manifest::load_artifact;
use od_core::{OdError, Result};
use std::fs;
use std::path::Path;

pub fn run(root: &Path, agent: &str, stdout: bool, reporter: &Reporter) -> Result<()> {
    // 解析目标 agent；未知值降级为 generic 并 warning。
    let agent_enum = match HandoffAgent::parse(agent) {
        Some(a) => a,
        None => {
            reporter.warning(&format!("unknown agent `{agent}`; using generic"));
            HandoffAgent::Generic
        }
    };

    // 读 artifact manifest 获取 title / brief / kind。
    let manifest_path = root.join("manifest.json");
    let (title, brief, kind) = if manifest_path.exists() {
        let m = load_artifact(&manifest_path)?;
        let kind = ArtifactKind::from_slug(&m.kind)?;
        (
            m.title.clone(),
            m.design.and_then(|d| d.visual_brief).unwrap_or_default(),
            kind,
        )
    } else {
        // manifest 缺失：从主文件推断 kind，title/brief 用默认。
        let primary = find_primary(root)
            .ok_or_else(|| OdError::PrimaryFileMissing(root.join("index.html")))?;
        let kind =
            detect_kind(&primary).ok_or_else(|| OdError::ArtifactKindUnknown(primary.clone()))?;
        ("Untitled".to_string(), String::new(), kind)
    };

    let artifact = Artifact::new(kind, root);
    let brief_ref: Option<&str> = if brief.is_empty() {
        None
    } else {
        Some(brief.as_str())
    };

    let content = render_handoff(&HandoffInput {
        artifact: &artifact,
        title: &title,
        brief: brief_ref,
        agent: agent_enum,
    });

    if stdout {
        println!("{content}");
    } else {
        fs::write(artifact.handoff_path(), content)?;
        reporter.warning("handoff.md regenerated");
    }
    Ok(())
}

/// 按 PRIMARY_FILE_ORDER 找已存在的主文件名。
fn find_primary(root: &Path) -> Option<String> {
    for name in od_core::artifact::PRIMARY_FILE_ORDER {
        if root.join(name).exists() {
            return Some(name.to_string());
        }
    }
    None
}
