//! od-mcp：MCP 工具面（M2）。默认 stdio transport + `rmcp`，异步限制在本 crate 内。
//! 所有工具必须调用 od-core 的同一套 artifact/workspace 规则，不重复实现路径/manifest/preview。
//! 本次：工具 `run()` 实现 + JSON DTO（serde/schemars）已就绪；`rmcp` / `tokio`
//! server 外壳由后续集成接入。DTO 与 schema 调度见 `tool_input_schema` /
//! `tool_output_schema`，供 server handler 直接取用。
//!
//! Spec: docs/specs/mcp.md

pub mod error;
pub mod tools;

use schemars::schema_for;
use tools::dto::{
    CreateRequest, CreateResponse, ExportRequest, ExportResponse, HandoffRequest,
    HandoffResponse, PreviewRequest, PreviewResponse,
};

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

/// 按工具名返回其入参的 JSON Schema（schemars 生成）。
/// 未知工具名返回 `None`，供 server 在 tools/list 里跳过。
///
/// Spec: docs/specs/mcp.md（各 tool 输入）
pub fn tool_input_schema(name: &str) -> Option<serde_json::Value> {
    Some(match name {
        "artifact_create" => serde_json::to_value(schema_for!(CreateRequest)).ok()?,
        "artifact_preview" => serde_json::to_value(schema_for!(PreviewRequest)).ok()?,
        "artifact_handoff" => serde_json::to_value(schema_for!(HandoffRequest)).ok()?,
        "artifact_export" => serde_json::to_value(schema_for!(ExportRequest)).ok()?,
        _ => return None,
    })
}

/// 按工具名返回其出参的 JSON Schema。
///
/// Spec: docs/specs/mcp.md（各 tool 输出）
pub fn tool_output_schema(name: &str) -> Option<serde_json::Value> {
    Some(match name {
        "artifact_create" => serde_json::to_value(schema_for!(CreateResponse)).ok()?,
        "artifact_preview" => serde_json::to_value(schema_for!(PreviewResponse)).ok()?,
        "artifact_handoff" => serde_json::to_value(schema_for!(HandoffResponse)).ok()?,
        "artifact_export" => serde_json::to_value(schema_for!(ExportResponse)).ok()?,
        _ => return None,
    })
}

/// 启动 stdio MCP server。M2 接入 rmcp + tokio（由组长协调接入）。
pub fn serve_stdio() -> Result<(), error::McpError> {
    todo!("M2: rmcp stdio server; see docs/specs/mcp.md")
}
