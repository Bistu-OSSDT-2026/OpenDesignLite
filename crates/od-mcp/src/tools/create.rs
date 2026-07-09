//! `artifact_create` → 等价 `odl new`，转发到 od-core。
//!
//! 领域规则（kind 映射、artifact 路径、manifest schema、handoff 渲染、skill 发现/匹配）
//! 全部调用 od-core 已有接口；本文件只做集成层适配：定位 skills 目录、读取 template、
//! 写文件。产物布局与 `odl new` 一致；`CreateResult` 由 MCP handler 序列化为
//! docs/specs/mcp.md 的 `artifact` 输出对象。
//!
//! Spec: docs/specs/mcp.md（artifact_create）

use crate::error::McpError;
use od_core::design::{VisualBrief, KERNEL_VERSION, STYLESHEET_ASSET};
use od_core::manifest::{ArtifactManifest, DesignMeta, SCHEMA_VERSION};
use od_core::{handoff, skill, Artifact, ArtifactKind, OdError};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// `artifact_create` 输入（对齐 docs/specs/mcp.md）。
pub struct CreateOptions<'a> {
    pub title: Option<&'a str>,
    /// `editorial` | `studio` | `workbench`；无法解析时回退到 skill front matter 再到 kind 默认。
    pub visual_brief: &'a str,
    /// 目标已存在时是否覆盖（对应 CLI `--force`）。
    pub overwrite: bool,
}

/// `artifact_create` 输出。`artifact` 对齐 mcp.md 输出对象；`warnings` 与 `odl new` 一致。
#[derive(Debug)]
pub struct CreateResult {
    pub artifact: Artifact,
    pub warnings: Vec<String>,
}

/// 创建 artifact：主文件 + `assets/od-design.css` + `manifest.json` + `handoff.md`。
///
/// 对应 CLI `odl new`。MCP handler 负责把 `CreateResult` 序列化为 mcp.md 的 JSON 对象。
pub fn run(kind_slug: &str, dir: &Path, options: CreateOptions<'_>) -> Result<CreateResult, McpError> {
    let kind = ArtifactKind::from_slug(kind_slug).map_err(from_core_err)?;
    fs::create_dir_all(dir)?;

    let artifact = Artifact::new(kind, dir);
    let primary = artifact.primary_path();
    if primary.exists() && !options.overwrite {
        return Err(from_core_err(OdError::AlreadyExists(primary)));
    }

    let mut warnings = Vec::new();
    let template = template_content(kind, dir, &mut warnings);
    let visual_brief = resolve_visual_brief(kind, options.visual_brief, dir);

    fs::create_dir_all(artifact.assets_dir())?;
    fs::write(
        artifact.assets_dir().join("od-design.css"),
        design_css(visual_brief),
    )?;
    fs::write(&primary, template)?;

    let title = options
        .title
        .map(str::to_string)
        .or_else(|| {
            dir.file_name()
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
            stylesheet: Some(STYLESHEET_ASSET.to_string()),
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

    Ok(CreateResult { artifact, warnings })
}

/// od-core 错误 → MCP 错误码。mcp.md 错误码表尚缺 `already_exists`/`io_error`：
/// 前者按“overwrite 参数与现状冲突”归 `invalid_args`，后者权宜归 `invalid_args`，待 spec 补齐。
fn from_core_err(err: OdError) -> McpError {
    match err {
        OdError::ArtifactKindUnknown(k) => McpError::InvalidArgs(format!(
            "unknown kind `{k}`; use html, docs, or slides"
        )),
        OdError::AlreadyExists(p) => McpError::InvalidArgs(format!(
            "{} already exists; pass overwrite to replace",
            p.display()
        )),
        OdError::Io(e) => McpError::InvalidArgs(format!("io error: {e}")),
        other => McpError::InvalidArgs(other.to_string()),
    }
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

fn builtin_skills_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

fn workspace_skills_dir(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        if ancestor.join("manifest.json").exists() {
            return Some(ancestor.join("skills"));
        }
    }
    None
}

fn starter_content(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Html => include_str!("../../../../templates/html-page/basic.html"),
        ArtifactKind::Markdown => "# Draft\n\nStart writing here.\n",
        ArtifactKind::Slides => include_str!("../../../../templates/slides/basic.html"),
    }
}

fn design_css(brief: VisualBrief) -> &'static str {
    match brief {
        VisualBrief::Editorial => EDITORIAL_CSS,
        VisualBrief::Studio => STUDIO_CSS,
        VisualBrief::Workbench => WORKBENCH_CSS,
    }
}

fn created_at() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

const EDITORIAL_CSS: &str = r#":root {
  --od-bg-canvas: #f7f3ed;
  --od-bg-surface: #fffaf3;
  --od-text-primary: #171412;
  --od-text-secondary: #5f5750;
  --od-accent-solid: #8f5b3f;
  --od-space-6: 1.5rem;
  --od-radius-lg: 1rem;
  --od-font-sans: Arial, sans-serif;
}
body { background: var(--od-bg-canvas); color: var(--od-text-primary); font-family: var(--od-font-sans); }
a { color: var(--od-accent-solid); }
"#;

const STUDIO_CSS: &str = r#":root {
  --od-bg-canvas: #111318;
  --od-bg-surface: #1b1f29;
  --od-text-primary: #f4f0e8;
  --od-text-secondary: #bbb4a8;
  --od-accent-solid: #d8a24a;
  --od-space-6: 1.5rem;
  --od-radius-lg: 1rem;
  --od-font-sans: Arial, sans-serif;
}
body { background: var(--od-bg-canvas); color: var(--od-text-primary); font-family: var(--od-font-sans); }
a { color: var(--od-accent-solid); }
"#;

const WORKBENCH_CSS: &str = r#":root {
  --od-bg-canvas: #f4f7f8;
  --od-bg-surface: #ffffff;
  --od-text-primary: #142026;
  --od-text-secondary: #52616b;
  --od-accent-solid: #1b6f8f;
  --od-space-6: 1.5rem;
  --od-radius-lg: 1rem;
  --od-font-sans: Arial, sans-serif;
}
body { background: var(--od-bg-canvas); color: var(--od-text-primary); font-family: var(--od-font-sans); }
a { color: var(--od-accent-solid); }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn invalid_kind_returns_invalid_args() {
        let err = run(
            "pdf",
            Path::new("ignored"),
            CreateOptions {
                title: None,
                visual_brief: "editorial",
                overwrite: false,
            },
        )
        .unwrap_err();
        assert_eq!(err.code(), "invalid_args");
    }

    #[test]
    fn creates_artifact_files() {
        let root = temp_root("mcp-create");
        let result = run(
            "html",
            &root,
            CreateOptions {
                title: Some("Demo"),
                visual_brief: "editorial",
                overwrite: false,
            },
        )
        .unwrap();

        assert_eq!(result.artifact.kind, ArtifactKind::Html);
        assert!(result.artifact.primary_path().exists());
        assert!(result.artifact.manifest_path().exists());
        assert!(result.artifact.handoff_path().exists());
        assert!(result.artifact.assets_dir().join("od-design.css").exists());

        let _ = fs::remove_dir_all(root);
    }

    fn temp_root(prefix: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        std::env::temp_dir().join(format!("od-mcp-{prefix}-{millis}"))
    }
}
