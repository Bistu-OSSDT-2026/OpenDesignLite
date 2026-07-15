//! MCP 错误码映射（对外契约）。
//!
//! Spec: docs/specs/mcp.md（错误格式）

use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpError {
    #[error("invalid args: {0}")]
    InvalidArgs(String),
    #[error("artifact not found: {0}")]
    ArtifactNotFound(String),
    #[error("manifest invalid: {0}")]
    ManifestInvalid(String),
    #[error("preview unavailable: {0}")]
    PreviewUnavailable(String),
    /// 预览子进程 spawn 成功但短时间内退出（mcp.md：不得假装成功）。
    #[error("preview crashed: {0}")]
    PreviewCrashed(String),
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    #[error("format unsupported: {0}")]
    FormatUnsupported(String),
    #[error("export failed: {0}")]
    ExportFailed(String),
    #[error("pdf backend missing: {0}")]
    PdfBackendMissing(String),
    #[error("resource missing: {0}")]
    ResourceMissing(String),
}

impl McpError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidArgs(_) => "invalid_args",
            Self::ArtifactNotFound(_) => "artifact_not_found",
            Self::ManifestInvalid(_) => "manifest_invalid",
            Self::PreviewUnavailable(_) => "preview_unavailable",
            Self::PreviewCrashed(_) => "preview_crashed",
            Self::NotImplemented(_) => "not_implemented",
            Self::FormatUnsupported(_) => "format_unsupported",
            Self::ExportFailed(_) => "export_failed",
            Self::PdfBackendMissing(_) => "pdf_backend_missing",
            Self::ResourceMissing(_) => "resource_missing",
        }
    }
}

/// 与各工具 `from_core_err` 里 `OdError::Io` 同口径：mcp.md 错误码表尚缺
/// `io_error`，暂归 `invalid_args`，待 spec 补齐。
impl From<std::io::Error> for McpError {
    fn from(err: std::io::Error) -> Self {
        Self::InvalidArgs(format!("io error: {err}"))
    }
}
