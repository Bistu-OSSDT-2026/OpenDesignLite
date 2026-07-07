//! `artifact_preview` → 等价 `odl preview`。无 GUI 环境应返回 `preview_unavailable`。
//!
//! Spec: docs/specs/mcp.md（artifact_preview）

use crate::error::McpError;

pub fn run() -> Result<(), McpError> {
    todo!("M2: open preview via od-preview; see docs/specs/mcp.md")
}
