//! 内核稳定错误类型。`code()` 返回的字符串是对外契约，被 CLI `--json` 与 MCP 复用。
//!
//! Spec: docs/specs/{artifact-workspace,cli,mcp}.md（错误码表）

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OdError {
    #[error("workspace not found at {0}")]
    WorkspaceNotFound(PathBuf),

    #[error("primary file {0} does not exist")]
    PrimaryFileMissing(PathBuf),

    #[error("manifest at {path} is invalid: {reason}")]
    ManifestInvalid { path: PathBuf, reason: String },

    #[error("unknown artifact kind `{0}`; use html, docs, or slides")]
    ArtifactKindUnknown(String),

    #[error("{0} already exists")]
    AlreadyExists(PathBuf),

    #[error("path escapes artifact root: {0}")]
    PathEscape(PathBuf),

    #[error("skill front matter is invalid: {0}")]
    SkillFrontMatterInvalid(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

impl OdError {
    /// 稳定错误码，用于 CLI `--json` 与 MCP 错误对象。变更需同步 specs。
    pub fn code(&self) -> &'static str {
        match self {
            Self::WorkspaceNotFound(_) => "workspace_not_found",
            Self::PrimaryFileMissing(_) => "primary_file_missing",
            Self::ManifestInvalid { .. } => "manifest_invalid",
            Self::ArtifactKindUnknown(_) => "artifact_kind_unknown",
            Self::AlreadyExists(_) => "already_exists",
            Self::PathEscape(_) => "path_escape",
            Self::SkillFrontMatterInvalid(_) => "skill_front_matter_invalid",
            Self::Io(_) => "io_error",
            Self::NotImplemented(_) => "not_implemented",
        }
    }
}

pub type Result<T> = std::result::Result<T, OdError>;
