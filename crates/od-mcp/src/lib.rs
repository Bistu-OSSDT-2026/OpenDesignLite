//! od-mcp：MCP 工具面（M2）。默认 stdio transport + `rmcp`，异步限制在本 crate 内。
//! 所有工具必须调用 od-core 的同一套 artifact/workspace 规则，不重复实现路径/manifest/preview。
//! 本次仅模块划分：`rmcp` / `tokio` 未接入，只留签名。
//!
//! Spec: docs/specs/mcp.md

pub mod error;
pub mod tools;

pub const MCP_SERVER_NAME: &str = "open-design-lite";

/// M2 暴露的工具名（对齐 CLI 命令）。
///
/// Spec: docs/specs/mcp.md（Tools）
pub fn tool_names() -> &'static [&'static str] {
    &[
        "artifact_create",
        "artifact_preview",
        "artifact_handoff",
        "artifact_export",
    ]
}

/// 启动 stdio MCP server。M2 接入 rmcp + tokio。
pub fn serve_stdio() -> Result<(), error::McpError> {
    todo!("M2: rmcp stdio server; see docs/specs/mcp.md")
}
