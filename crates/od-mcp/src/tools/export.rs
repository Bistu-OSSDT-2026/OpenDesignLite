//! `artifact_export` → 等价 `odl export`。M4 前返回 `not_implemented`。
//!
//! Spec: docs/specs/mcp.md（Tools）, export.md

use crate::error::McpError;

pub fn run() -> Result<(), McpError> {
    Err(McpError::NotImplemented("artifact_export (M4)"))
}
