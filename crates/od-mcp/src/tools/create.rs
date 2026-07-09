//! `artifact_create` → 等价 `odl new`，转发到 od-core。
//!
//! 领域规则（kind 映射、artifact 路径、manifest schema、handoff 渲染、skill 发现/匹配）
//! 全部调用 `od-core::create`；本文件只做 MCP 错误码适配。
//!
//! Spec: docs/specs/mcp.md（artifact_create）

use crate::error::McpError;
use od_core::{create, Artifact, OdError};
use std::path::Path;

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
pub fn run(
    kind_slug: &str,
    dir: &Path,
    options: CreateOptions<'_>,
) -> Result<CreateResult, McpError> {
    let result = create::run(
        kind_slug,
        dir,
        create::CreateOptions {
            title: options.title,
            visual_brief: options.visual_brief,
            embed_css: false,
            overwrite: options.overwrite,
        },
    )
    .map_err(from_core_err)?;

    Ok(CreateResult {
        artifact: result.artifact,
        warnings: result.warnings,
    })
}

/// od-core 错误 → MCP 错误码。mcp.md 错误码表尚缺 `already_exists`/`io_error`：
/// 前者按“overwrite 参数与现状冲突”归 `invalid_args`，后者权宜归 `invalid_args`，待 spec 补齐。
fn from_core_err(err: OdError) -> McpError {
    match err {
        OdError::ArtifactKindUnknown(k) => {
            McpError::InvalidArgs(format!("unknown kind `{k}`; use html, docs, or slides"))
        }
        OdError::AlreadyExists(p) => McpError::InvalidArgs(format!(
            "{} already exists; pass overwrite to replace",
            p.display()
        )),
        OdError::Io(e) => McpError::InvalidArgs(format!("io error: {e}")),
        other => McpError::InvalidArgs(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use od_core::ArtifactKind;
    use std::fs;
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
