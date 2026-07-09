//! `odl new`：创建 artifact（主文件 + manifest.json + assets/od-design.css + handoff.md）。
//!
//! Spec: docs/specs/cli.md, artifact-workspace.md, built-in-skills.md, handoff.md, design-kernel.md

use crate::output::Reporter;
use od_core::design::{VisualBrief, STYLESHEET_ASSET};
use od_core::handoff::{render as render_handoff, HandoffAgent, HandoffInput};
use od_core::manifest::{save_artifact, ArtifactManifest};
use od_core::skill::{discover, for_kind};
use od_core::workspace::{find_workspace_root, skills_dir};
use od_core::{Artifact, ArtifactKind, OdError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// 内置 skills 目录（开发期用 `CARGO_MANIFEST_DIR` 定位仓库根 `skills/`）。
fn builtin_skills_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

/// 仓库根的 `templates/od-design.css`（编译期嵌入，写进 artifact assets）。
const DESIGN_CSS: &str = include_str!("../../../../templates/od-design.css");

pub fn run(
    kind_slug: &str,
    root: &Path,
    title: Option<&str>,
    brief: Option<&str>,
    embed_css: bool,
    force: bool,
    reporter: &Reporter,
) -> Result<Artifact> {
    let kind = ArtifactKind::from_slug(kind_slug)?;
    let artifact = Artifact::new(kind, root);

    // 已存在检查。
    let primary = artifact.primary_path();
    if primary.exists() && !force {
        return Err(OdError::AlreadyExists(primary));
    }
    fs::create_dir_all(root)?;

    // 发现 skills：内置 + workspace 覆盖。
    let builtin = builtin_skills_dir();
    let workspace_skills = std::env::current_dir()
        .ok()
        .and_then(|cwd| find_workspace_root(&cwd))
        .map(|ws_root| skills_dir(&ws_root));
    let skills = discover(&builtin, workspace_skills.as_deref());
    let skill = for_kind(&skills, kind);

    // 模板内容：skill 模板优先，缺失则 fallback 到内置 starter。
    let (template_content, used_skill) = match skill.and_then(|s| s.template_path()) {
        Some(path) => match fs::read_to_string(&path) {
            Ok(content) => (content, true),
            Err(_) => (fallback_starter(kind), false),
        },
        None => (fallback_starter(kind), false),
    };
    if !used_skill {
        reporter.warning("skill template not found; using built-in starter");
    }

    // brief 解析：--brief > skill.visual_brief > default_for(kind)。
    let brief_str = brief
        .map(|b| b.to_string())
        .or_else(|| skill.and_then(|s| s.front.visual_brief.clone()))
        .unwrap_or_else(|| VisualBrief::default_for(kind).as_str().to_string());
    // 校验 brief 合法性（未知值降级为 default，并 warning）。
    let brief_str = match VisualBrief::parse(&brief_str) {
        Some(_) => brief_str,
        None => {
            reporter.warning(&format!(
                "unknown brief `{brief_str}`; falling back to default"
            ));
            VisualBrief::default_for(kind).as_str().to_string()
        }
    };

    let title = title.unwrap_or("Untitled");

    // 写主文件（embed_css 时把 CSS 内联进 <style data-od-design>，替换 <link>）。
    let main_content = if embed_css {
        inline_css(&template_content, DESIGN_CSS)
    } else {
        template_content
    };
    fs::write(&primary, main_content)?;

    // 写 assets/od-design.css（embed_css 时跳过外部文件，已内联）。
    if !embed_css && matches!(kind, ArtifactKind::Html | ArtifactKind::Slides) {
        let assets = artifact.assets_dir();
        fs::create_dir_all(&assets)?;
        fs::write(assets.join("od-design.css"), DESIGN_CSS)?;
    }

    // 写 manifest.json。
    let manifest = ArtifactManifest::new(kind.slug(), title, kind.primary_file(), Some(&brief_str));
    save_artifact(&manifest, &artifact.manifest_path())?;

    // 写 handoff.md（整体生成；M2 才做 section-level 更新）。
    let handoff_input = HandoffInput {
        artifact: &artifact,
        title,
        brief: Some(&brief_str),
        agent: HandoffAgent::Generic,
    };
    let handoff_content = render_handoff(&handoff_input);
    let handoff_path = artifact.handoff_path();
    if handoff_path.exists() && !force {
        reporter.warning("handoff.md already exists; use --force to regenerate");
    } else {
        fs::write(&handoff_path, handoff_content)?;
    }

    Ok(artifact)
}

/// M0 兼容 fallback starter（skill 模板缺失时使用）。
fn fallback_starter(kind: ArtifactKind) -> String {
    match kind {
        ArtifactKind::Html => include_str!("../../../../skills/html-page/templates/basic.html").to_string(),
        ArtifactKind::Markdown => {
            "# Title\n\nReplace this with your document title.\n\n## Section\n\nStart writing here.\n"
                .to_string()
        }
        ArtifactKind::Slides => include_str!("../../../../skills/slides-html/templates/basic.html").to_string(),
    }
}

/// 把模板里的 `<link rel="stylesheet" href="assets/od-design.css">` 替换为
/// `<style data-od-design>...</style>`（内联模式）。
fn inline_css(template: &str, css: &str) -> String {
    let link = format!("<link rel=\"stylesheet\" href=\"{STYLESHEET_ASSET}\" />");
    let style = format!("<style data-od-design>\n{css}\n</style>");
    if template.contains(&link) {
        template.replace(&link, &style)
    } else {
        // 兼容无自闭合斜杠的写法。
        let link_alt = format!("<link rel=\"stylesheet\" href=\"{STYLESHEET_ASSET}\">");
        template.replace(&link_alt, &style)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_css_replaces_link() {
        let tpl =
            "<html><head><link rel=\"stylesheet\" href=\"assets/od-design.css\" /></head></html>";
        let out = inline_css(tpl, "/* css */");
        assert!(out.contains("<style data-od-design>"));
        assert!(!out.contains("<link rel=\"stylesheet\" href=\"assets/od-design.css"));
    }

    #[test]
    fn inline_css_handles_no_self_closing() {
        let tpl =
            "<html><head><link rel=\"stylesheet\" href=\"assets/od-design.css\"></head></html>";
        let out = inline_css(tpl, "/* css */");
        assert!(out.contains("<style data-od-design>"));
    }
}
