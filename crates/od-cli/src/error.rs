//! CLI 层统一错误类型：能同时容纳内核错误（`OdError`）与预览错误（`PreviewError`）。
//!
//! 为什么需要它：`odl preview` 调 `od_preview::preview` 会产生 `PreviewError`，
//! 而 `init`/`new` 等命令产生 `OdError`。dispatch 需要一个统一出口。
//! 架构边界不允许让 od-core 依赖 od-preview，所以不能把 `PreviewError` 塞进 `OdError`，
//! 只能在 CLI 这一层做个适配器。
//!
//! `From` impl 让 `?` 自动转换：`od_preview::preview(opts)?;` 里 `?` 看到 `PreviewError`，
//! 查到 `From<PreviewError> for CliError`，自动转，无需手写 `map_err`。
//!
//! `code()` 委托给内部错误，保证 `artifact_not_found` / `webview_failed` 等
//! spec 契约码原样传到用户输出（`--json` 与 MCP 复用）。

use od_core::OdError;
use od_mcp::error::McpError;
use od_preview::PreviewError;

#[derive(Debug)]
pub enum CliError {
    /// 内核错误（init / new / handoff / export 命令产生）。
    Core(OdError),
    /// 预览错误（preview 命令产生）。
    Preview(PreviewError),
    /// MCP server 错误（mcp 命令产生）。
    Mcp(McpError),
}

impl CliError {
    /// 稳定错误码。委托给内部错误，不自己发明码，保留 spec 契约。
    pub fn code(&self) -> &'static str {
        match self {
            CliError::Core(e) => e.code(),
            CliError::Preview(e) => e.code(),
            CliError::Mcp(e) => e.code(),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Core(e) => std::fmt::Display::fmt(e, f),
            CliError::Preview(e) => std::fmt::Display::fmt(e, f),
            CliError::Mcp(e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::Core(e) => Some(e),
            CliError::Preview(e) => Some(e),
            CliError::Mcp(e) => Some(e),
        }
    }
}

impl From<OdError> for CliError {
    fn from(e: OdError) -> Self {
        CliError::Core(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Core(OdError::Io(e))
    }
}

impl From<PreviewError> for CliError {
    fn from(e: PreviewError) -> Self {
        CliError::Preview(e)
    }
}

impl From<McpError> for CliError {
    fn from(e: McpError) -> Self {
        CliError::Mcp(e)
    }
}

pub type Result<T> = std::result::Result<T, CliError>;
