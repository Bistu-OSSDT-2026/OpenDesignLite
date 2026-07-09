//! `artifact_handoff` → 等价 `odl handoff`，转发到 od-core。
//!
//! 仅做集成层适配：从目录探测主文件 → `Artifact` → `handoff::render`；
//! 按 `write` 决定落盘或只读返回 `content`。领域规则全部调用 od-core，
//! 不重复实现路径或渲染。行为与 `odl handoff` 对齐：
//! `write=true` 覆盖重写（M1 整体重写），`write=false` 只读——已有 handoff.md
//! 则读现有内容返回，无则渲染不落盘（遵守 handoff spec “不得无提示覆盖”）。
//!
//! Spec: docs/specs/mcp.md（artifact_handoff）, handoff.md

use crate::error::McpError;
use od_core::artifact::{detect_kind, PRIMARY_FILE_ORDER};
use od_core::handoff::{self, HandoffAgent};
use od_core::{Artifact, OdError};
use std::fs;
use std::path::Path;

/// `artifact_handoff` 输入（对齐 docs/specs/mcp.md）。
pub struct HandoffOptions<'a> {
    pub dir: &'a Path,
    /// `generic` | `opencode` | `claude-code` | `codex`；无法解析返回 `invalid_args`。
    pub agent: &'a str,
    /// `true` → 覆盖重写 handoff.md；`false` → 只读，不落盘。
    pub write: bool,
}

/// `artifact_handoff` 输出。对齐 mcp.md 的 `{path, content}`。
#[derive(Debug)]
pub struct HandoffResult {
    pub path: std::path::PathBuf,
    pub content: String,
}

/// 读取或生成 handoff.md。对应 CLI `odl handoff`。
pub fn run(options: HandoffOptions<'_>) -> Result<HandoffResult, McpError> {
    let agent = HandoffAgent::parse(options.agent).ok_or_else(|| {
        McpError::InvalidArgs(format!(
            "unknown agent `{}`; use generic, opencode, claude-code, or codex",
            options.agent
        ))
    })?;
    let artifact = artifact_from_root(options.dir).map_err(from_core_err)?;
    let path = artifact.handoff_path();

    let content = if options.write {
        let rendered = handoff::render(&artifact, agent);
        fs::write(&path, &rendered)?;
        rendered
    } else if path.exists() {
        // 已有 handoff.md：读现有内容返回，不覆盖（handoff spec 约束）。
        fs::read_to_string(&path)?
    } else {
        // 没有 handoff.md 且未要求 write：渲染后只返回内容，不落盘。
        handoff::render(&artifact, agent)
    };

    Ok(HandoffResult { path, content })
}

/// 从目录探测主文件 → `Artifact`（与 `odl handoff` 的 `artifact_from_root` 同口径）。
/// 仅调用 od-core 公开接口（`PRIMARY_FILE_ORDER` + `detect_kind` + `Artifact::new`）。
fn artifact_from_root(root: &Path) -> Result<Artifact, OdError> {
    for primary in PRIMARY_FILE_ORDER {
        let path = root.join(primary);
        if path.exists() {
            let kind = detect_kind(primary).expect("primary file order maps to kind");
            return Ok(Artifact::new(kind, root));
        }
    }
    Err(OdError::PrimaryFileMissing(root.join("index.html")))
}

/// od-core 错误 → MCP 错误码。mcp.md 错误码表尚缺 `io_error`/`already_exists`/
/// `primary_file_missing`：io 权宜归 `invalid_args`，primary 缺失归 `artifact_not_found`，
/// 其余待 spec 补齐。
fn from_core_err(err: OdError) -> McpError {
    match err {
        OdError::PrimaryFileMissing(p) => {
            McpError::ArtifactNotFound(format!("{}", p.display()))
        }
        OdError::Io(e) => McpError::InvalidArgs(format!("io error: {e}")),
        other => McpError::InvalidArgs(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    /// 未知 agent → `invalid_args`，不触碰内核错误码。
    #[test]
    fn unknown_agent_returns_invalid_args() {
        let root = temp_root("mcp-handoff-agent");
        let err = run(HandoffOptions {
            dir: &root,
            agent: "ghost",
            write: false,
        })
        .unwrap_err();
        assert_eq!(err.code(), "invalid_args");
    }

    /// 没 artifact 目录（无主文件）→ `artifact_not_found`。
    #[test]
    fn missing_artifact_returns_artifact_not_found() {
        let root = temp_root("mcp-handoff-missing");
        std::fs::create_dir_all(&root).unwrap();
        let err = run(HandoffOptions {
            dir: &root,
            agent: "opencode",
            write: false,
        })
        .unwrap_err();
        assert_eq!(err.code(), "artifact_not_found");

        let _ = std::fs::remove_dir_all(root);
    }

    /// `write=true`：渲染并落盘，返回内容与磁盘一致。
    #[test]
    fn write_persists_handoff() {
        let root = temp_root("mcp-handoff-write");
        std::fs::create_dir_all(&root).unwrap();
        // 造一个 html artifact 主文件
        std::fs::write(root.join("index.html"), "<!doctype html><p>demo</p>").unwrap();

        let result = run(HandoffOptions {
            dir: &root,
            agent: "generic",
            write: true,
        })
        .unwrap();
        assert_eq!(result.path, root.join("handoff.md"));
        assert!(result.path.exists(), "handoff.md should be written");
        let on_disk = std::fs::read_to_string(&result.path).unwrap();
        assert_eq!(result.content, on_disk);
        assert!(result.content.contains("# Handoff:"));

        let _ = std::fs::remove_dir_all(root);
    }

    /// `write=false` + 已有 handoff.md：读现有内容，不覆盖。
    #[test]
    fn read_returns_existing_without_overwrite() {
        let root = temp_root("mcp-handoff-read");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "<!doctype html><p>demo</p>").unwrap();
        let existing = "# Handoff: user-edited\n\ncustom notes";
        std::fs::write(root.join("handoff.md"), existing).unwrap();

        let result = run(HandoffOptions {
            dir: &root,
            agent: "opencode",
            write: false,
        })
        .unwrap();
        assert_eq!(result.content, existing, "read must return existing file verbatim");
        // 没被改写
        assert_eq!(std::fs::read_to_string(&result.path).unwrap(), existing);

        let _ = std::fs::remove_dir_all(root);
    }

    /// `write=false` + 无 handoff.md：渲染返回内容，但不落盘。
    #[test]
    fn read_without_existing_renders_unwritten() {
        let root = temp_root("mcp-handoff-norender");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("index.html"), "<!doctype html><p>demo</p>").unwrap();

        let result = run(HandoffOptions {
            dir: &root,
            agent: "codex",
            write: false,
        })
        .unwrap();
        assert!(result.content.contains("# Handoff:"));
        assert!(
            !result.path.exists(),
            "write=false must not persist when handoff.md absent"
        );

        let _ = std::fs::remove_dir_all(root);
    }

    fn temp_root(prefix: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        std::env::temp_dir().join(format!("od-mcp-{prefix}-{millis}"))
    }
}