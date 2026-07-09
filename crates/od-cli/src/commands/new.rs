//! `odl new`：创建 artifact（主文件 + handoff）。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md, handoff.md, built-in-skills.md

use crate::commands::skill::{builtin_skills_dir, workspace_skills_dir};
use od_core::design::{css_for, guardrails, VisualBrief, KERNEL_VERSION, STYLESHEET_ASSET};
use od_core::manifest::{ArtifactManifest, DesignMeta, SCHEMA_VERSION};
use od_core::{handoff, skill, Artifact, ArtifactKind, OdError, Result};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let kind = ArtifactKind::from_slug(kind_slug)?;
    fs::create_dir_all(root)?;

    let artifact = Artifact::new(kind, root);
    let primary = artifact.primary_path();
    if primary.exists() && !options.force {
        return Err(OdError::AlreadyExists(primary));
    }

    let mut warnings = Vec::new();
    let template = template_content(kind, root, &mut warnings);
    let visual_brief = resolve_visual_brief(kind, options.brief, root);
    let stylesheet = if options.embed_css {
        None
    } else {
        fs::create_dir_all(artifact.assets_dir())?;
        fs::write(
            artifact.assets_dir().join("od-design.css"),
            css_for(visual_brief),
        )?;
        Some(STYLESHEET_ASSET.to_string())
    };

    let mut primary_content = template;
    if options.embed_css {
        primary_content = embed_design_css(&primary_content, &css_for(visual_brief));
    } else {
        primary_content = link_design_css(&primary_content);
    }

    fs::write(&primary, primary_content)?;

    let title = options
        .title
        .map(str::to_string)
        .or_else(|| {
            root.file_name()
                .and_then(|name| name.to_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "Open Design Lite Artifact".to_string());
    let manifest = ArtifactManifest {
        schema_version: SCHEMA_VERSION,
        r#type: "artifact".to_string(),
        kind: kind.slug().to_string(),
        title,
        primary_file: kind.primary_file().to_string(),
        created_by: "odl".to_string(),
        created_at: created_at(),
        design: Some(DesignMeta {
            kernel_version: KERNEL_VERSION,
            stylesheet,
            visual_brief: Some(visual_brief.as_str().to_string()),
        }),
    };
    fs::write(
        artifact.manifest_path(),
        serde_json::to_string_pretty(&manifest).expect("artifact manifest serializes"),
    )?;
    fs::write(
        artifact.handoff_path(),
        handoff::render(&artifact, handoff::HandoffAgent::Generic),
    )?;

    Ok(NewResult { artifact, warnings })
}

fn template_content(kind: ArtifactKind, root: &Path, warnings: &mut Vec<String>) -> String {
    let skills = skill::discover(&builtin_skills_dir(), workspace_skills_dir(root).as_deref());
    let Some(found) = skill::for_kind(&skills, kind) else {
        warnings.push(format!(
            "skill for kind `{}` was not found; using fallback starter",
            kind.slug()
        ));
        return starter_content(kind).to_string();
    };

    let Some(path) = found.template_path() else {
        warnings.push(format!(
            "skill `{}` has no usable template; using fallback starter",
            found.name()
        ));
        return starter_content(kind).to_string();
    };

    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(err) => {
            warnings.push(format!(
                "failed to read template `{}`: {err}; using fallback starter",
                path.display()
            ));
            starter_content(kind).to_string()
        }
    }
}

fn resolve_visual_brief(kind: ArtifactKind, brief: &str, root: &Path) -> VisualBrief {
    if let Some(parsed) = VisualBrief::parse(brief) {
        return parsed;
    }

    let skills = skill::discover(&builtin_skills_dir(), workspace_skills_dir(root).as_deref());
    skill::for_kind(&skills, kind)
        .and_then(|found| found.front.visual_brief.as_deref())
        .and_then(VisualBrief::parse)
        .unwrap_or_else(|| VisualBrief::default_for(kind))
}

fn starter_content(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Html => include_str!("../../../../templates/html-page/basic.html"),
        ArtifactKind::Markdown => "# Draft\n\nStart writing here.\n",
        ArtifactKind::Slides => include_str!("../../../../templates/slides/basic.html"),
    }
}

fn embed_design_css(content: &str, css: &str) -> String {
    if !content.contains("</head>") {
        return content.to_string();
    }
    let content = content.replace(
        &format!("  <link rel=\"stylesheet\" href=\"{STYLESHEET_ASSET}\" />\n"),
        "",
    );
    content.replace(
        "</head>",
        &format!(
            "  <style {}>\n{css}\n  </style>\n  </head>",
            guardrails::INLINE_STYLE_MARKER
        ),
    )
}

fn link_design_css(content: &str) -> String {
    if guardrails::references_stylesheet(content) || !content.contains("</head>") {
        return content.to_string();
    }
    content.replace(
        "</head>",
        &format!("  <link rel=\"stylesheet\" href=\"{STYLESHEET_ASSET}\" />\n  </head>"),
    )
}

fn created_at() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}
