//! handoff.md 章节模型与渲染。
//!
//! Spec: docs/specs/handoff.md

use crate::artifact::{Artifact, ArtifactKind};
use crate::design::{VisualBrief, STYLESHEET_ASSET};

/// handoff.md 标准章节顺序（集成契约）。
pub const SECTIONS: [&str; 7] = [
    "Intent",
    "Artifact",
    "Files",
    "Design Notes",
    "How To Preview",
    "Next Steps",
    "Agent Notes",
];

/// 目标 Agent，影响 `Agent Notes` 的附注文案（不改变文件布局规则）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffAgent {
    Generic,
    OpenCode,
    ClaudeCode,
    Codex,
}

impl HandoffAgent {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "generic" => Some(Self::Generic),
            "opencode" => Some(Self::OpenCode),
            "claude-code" => Some(Self::ClaudeCode),
            "codex" => Some(Self::Codex),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::OpenCode => "opencode",
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
        }
    }
}

/// 渲染 handoff 所需的上下文（纯数据，便于单测）。
#[derive(Debug, Clone)]
pub struct HandoffInput<'a> {
    pub artifact: &'a Artifact,
    pub title: &'a str,
    pub brief: Option<&'a str>,
    pub agent: HandoffAgent,
}

/// 渲染完整 handoff.md（7 章标准结构，纯函数）。
pub fn render(input: &HandoffInput) -> String {
    let artifact = input.artifact;
    let kind = artifact.kind;
    let primary = kind.primary_file();
    let dir = artifact
        .root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");

    let mut out = String::new();
    out.push_str(&format!("# Handoff — {}\n\n", input.title));

    // Intent
    out.push_str("## Intent\n\n");
    out.push_str(&format!(
        "This artifact is a `{}` previewable with `odl preview`. Replace this section with the \
        user's goal, constraints, and the problem the artifact solves.\n\n",
        kind.slug()
    ));

    // Artifact
    out.push_str("## Artifact\n\n");
    let brief_line = input
        .brief
        .unwrap_or_else(|| VisualBrief::default_for(kind).as_str());
    out.push_str(&format!(
        "- kind: `{}`\n- primary file: `{}`\n- visual brief: `{}`\n- target agent: `{}`\n\n",
        kind.slug(),
        primary,
        brief_line,
        input.agent.as_str(),
    ));

    // Files
    out.push_str("## Files\n\n");
    out.push_str(&format!("- `{primary}`\n"));
    out.push_str("- `manifest.json`\n");
    if matches!(kind, ArtifactKind::Html | ArtifactKind::Slides) {
        out.push_str(&format!("- `{STYLESHEET_ASSET}`\n"));
    }
    out.push_str("- `handoff.md`\n\n");

    // Design Notes
    out.push_str("## Design Notes\n\n");
    out.push_str("Design constraints for the next agent:\n\n");
    out.push_str(&format!(
        "- Use `{STYLESHEET_ASSET}` (linked) or inline `<style data-od-design>` for styling.\n"
    ));
    out.push_str("- Use `--od-*` design tokens; do not hardcode a parallel token system.\n");
    out.push_str("- Do not introduce a dev server, Tailwind, React, shadcn/ui, or a CDN UI kit.\n");
    out.push_str("- Keep the artifact previewable by `odl preview` with no build step.\n");
    if matches!(kind, ArtifactKind::Slides) {
        out.push_str(
            "- Slides must keep a 16:9 frame, not overflow, and support keyboard navigation.\n",
        );
    }
    if matches!(kind, ArtifactKind::Markdown) {
        out.push_str("- Preserve technical facts; do not rewrite meaning when polishing prose.\n");
    }
    out.push('\n');

    // How To Preview
    out.push_str("## How To Preview\n\n");
    out.push_str(&format!("```\nodl preview {dir}\n```\n\n",));
    if matches!(kind, ArtifactKind::Markdown) {
        out.push_str(
            "Markdown is rendered to HTML by the preview shell; edits reload the preview.\n\n",
        );
    } else {
        out.push_str("Edits to the primary file or assets reload the preview automatically.\n\n");
    }

    // Next Steps
    out.push_str("## Next Steps\n\n");
    out.push_str("- [ ] Confirm the artifact matches the intent above.\n");
    out.push_str("- [ ] Fill in content / refine layout.\n");
    out.push_str("- [ ] Verify it opens with `odl preview`.\n\n");

    // Agent Notes
    out.push_str("## Agent Notes\n\n");
    out.push_str(match input.agent {
        HandoffAgent::Generic => {
            "Hand this file to any coding agent. Keep edits inspectable and handoff-friendly.\n"
        }
        HandoffAgent::OpenCode => {
            "For OpenCode: read the Files section, edit files in place, and re-run `odl preview`.\n"
        }
        HandoffAgent::ClaudeCode => {
            "For Claude Code: treat the Files list as the working set; preserve manifest.json fields.\n"
        }
        HandoffAgent::Codex => {
            "For Codex: keep changes minimal and boring; do not add build tooling.\n"
        }
    });

    out
}

/// M0 兼容：最小占位 handoff。M1 起改用 `render`。
#[deprecated(note = "use `render` with `HandoffInput` instead")]
pub fn minimal_stub() -> &'static str {
    "# Handoff\n\nDescribe intent, constraints, and next agent steps here.\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn render_contains_all_sections() {
        let artifact = Artifact::new(ArtifactKind::Html, Path::new("site"));
        let out = render(&HandoffInput {
            artifact: &artifact,
            title: "Demo",
            brief: Some("editorial"),
            agent: HandoffAgent::Generic,
        });
        for section in SECTIONS {
            assert!(
                out.contains(&format!("## {section}")),
                "missing section: {section}"
            );
        }
    }

    #[test]
    fn render_contains_preview_command_and_design_constraints() {
        let artifact = Artifact::new(ArtifactKind::Html, Path::new("site"));
        let out = render(&HandoffInput {
            artifact: &artifact,
            title: "Demo",
            brief: None,
            agent: HandoffAgent::Generic,
        });
        assert!(out.contains("odl preview"));
        assert!(out.contains("--od-*"));
        assert!(out.contains("Tailwind"));
        assert!(out.contains(STYLESHEET_ASSET));
    }

    #[test]
    fn render_slides_includes_16_9_and_keyboard() {
        let artifact = Artifact::new(ArtifactKind::Slides, Path::new("deck"));
        let out = render(&HandoffInput {
            artifact: &artifact,
            title: "Deck",
            brief: Some("studio"),
            agent: HandoffAgent::Generic,
        });
        assert!(out.contains("16:9"));
        assert!(out.contains("keyboard"));
    }

    #[test]
    fn render_docs_preserve_facts() {
        let artifact = Artifact::new(ArtifactKind::Markdown, Path::new("doc"));
        let out = render(&HandoffInput {
            artifact: &artifact,
            title: "Doc",
            brief: Some("editorial"),
            agent: HandoffAgent::Codex,
        });
        assert!(out.contains("Preserve technical facts"));
    }

    #[test]
    fn agent_notes_vary_by_agent() {
        let artifact = Artifact::new(ArtifactKind::Html, Path::new("site"));
        let generic = render(&HandoffInput {
            artifact: &artifact,
            title: "t",
            brief: None,
            agent: HandoffAgent::Generic,
        });
        let codex = render(&HandoffInput {
            artifact: &artifact,
            title: "t",
            brief: None,
            agent: HandoffAgent::Codex,
        });
        assert!(generic.contains("any coding agent"));
        assert!(codex.contains("Codex"));
    }
}
