//! od-mcp：MCP 工具面（M2）。默认 stdio transport + `rmcp`，异步限制在本 crate 内。
//! 所有工具必须调用 od-core 的同一套 artifact/workspace 规则，不重复实现路径/manifest/preview。
//! DTO 与 schema 调度见 `tool_input_schema` / `tool_output_schema`；`serve_stdio()`
//! 提供一个轻量 JSON-RPC stdio MCP server。
//!
//! Spec: docs/specs/mcp.md

pub mod error;
pub mod tools;

use schemars::schema_for;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tools::dto::{
    CreateRequest, CreateResponse, ExportRequest, ExportResponse, HandoffRequest, HandoffResponse,
    PreviewRequest, PreviewResponse,
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
    // Prefer compact hand-written schemas for tools/list: Cursor's tool discovery
    // is happier without schemars `$ref` / `definitions` / `oneOf` graphs.
    Some(match name {
        "artifact_create" => json!({
            "type": "object",
            "properties": {
                "kind": {"type": "string", "description": "html | docs | slides"},
                "dir": {"type": "string", "description": "artifact root directory"},
                "title": {"type": "string"},
                "visualBrief": {"type": "string", "description": "editorial | studio | workbench"},
                "overwrite": {"type": "boolean", "default": false}
            },
            "required": ["kind", "dir"],
            "additionalProperties": false
        }),
        "artifact_preview" => json!({
            "type": "object",
            "properties": {
                "dir": {"type": "string"},
                "externalBrowser": {"type": "boolean", "default": false},
                "watch": {"type": "boolean", "default": true}
            },
            "required": ["dir"],
            "additionalProperties": false
        }),
        "artifact_handoff" => json!({
            "type": "object",
            "properties": {
                "dir": {"type": "string"},
                "agent": {"type": "string", "default": "generic"},
                "write": {"type": "boolean", "default": false}
            },
            "required": ["dir"],
            "additionalProperties": false
        }),
        "artifact_export" => json!({
            "type": "object",
            "properties": {
                "dir": {"type": "string"},
                "format": {"type": "string", "enum": ["html", "md", "zip", "pdf"]},
                "out": {"type": "string"}
            },
            "required": ["dir", "format"],
            "additionalProperties": false
        }),
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

/// 启动 stdio MCP server。
pub fn serve_stdio() -> Result<(), error::McpError> {
    // Keep stderr quiet: Cursor surfaces any stderr line as an MCP error.
    let stdin = io::stdin();
    let stdout = io::stdout();
    serve_stdio_with(stdin.lock(), stdout.lock())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Framing {
    /// LSP-style `Content-Length` headers.
    ContentLength,
    /// Newline-delimited JSON (no headers). Cursor may speak this on stdio.
    Ndjson,
}

struct Incoming {
    framing: Framing,
    body: Vec<u8>,
}

fn serve_stdio_with<R: BufRead, W: Write>(
    mut reader: R,
    mut writer: W,
) -> Result<(), error::McpError> {
    while let Some(message) = read_message(&mut reader)? {
        debug_log(&format!(
            "recv {:?} {} bytes: {}",
            message.framing,
            message.body.len(),
            String::from_utf8_lossy(&message.body)
        ));
        let request: Value = serde_json::from_slice(&message.body).map_err(|err| {
            error::McpError::InvalidArgs(format!("invalid json-rpc message: {err}"))
        })?;
        if let Some(response) = handle_json_rpc(request) {
            debug_log(&format!("send {:?}: {response}", message.framing));
            write_message(&mut writer, message.framing, &response)?;
        } else {
            debug_log("no response (notification)");
        }
    }
    debug_log("stdin closed");
    Ok(())
}

fn debug_log(msg: &str) {
    use std::fs::OpenOptions;
    use std::io::Write as _;
    let path = std::env::temp_dir().join("odl-mcp-debug.log");
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(
            f,
            "{} {msg}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        );
    }
}

fn read_message<R: BufRead>(reader: &mut R) -> Result<Option<Incoming>, error::McpError> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            if content_length.is_none() {
                continue;
            }
            break;
        }

        // Some clients speak newline-delimited JSON with no headers.
        let looks_like_json = trimmed.starts_with('{') || trimmed.starts_with('[');
        if looks_like_json && content_length.is_none() {
            return Ok(Some(Incoming {
                framing: Framing::Ndjson,
                body: trimmed.as_bytes().to_vec(),
            }));
        }

        // MCP headers are case-insensitive; Cursor may send `content-length:`.
        if let Some(value) = content_length_value(trimmed) {
            let parsed = value.parse::<usize>().map_err(|err| {
                error::McpError::InvalidArgs(format!("invalid Content-Length: {err}"))
            })?;
            content_length = Some(parsed);
        }
    }

    let length = content_length
        .ok_or_else(|| error::McpError::InvalidArgs("missing Content-Length".to_string()))?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(Some(Incoming {
        framing: Framing::ContentLength,
        body,
    }))
}

fn content_length_value(header_line: &str) -> Option<&str> {
    let (name, value) = header_line.split_once(':')?;
    if name.eq_ignore_ascii_case("Content-Length") {
        Some(value.trim())
    } else {
        None
    }
}

fn write_message<W: Write>(
    writer: &mut W,
    framing: Framing,
    value: &Value,
) -> Result<(), error::McpError> {
    let body = serde_json::to_vec(value)
        .map_err(|err| error::McpError::InvalidArgs(format!("serialize response failed: {err}")))?;
    match framing {
        Framing::ContentLength => {
            write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
            writer.write_all(&body)?;
        }
        Framing::Ndjson => {
            writer.write_all(&body)?;
            writer.write_all(b"\n")?;
        }
    }
    writer.flush()?;
    Ok(())
}

fn handle_json_rpc(request: Value) -> Option<Value> {
    let id = request.get("id").cloned();
    let method = request.get("method").and_then(Value::as_str).unwrap_or("");
    if id.is_none() {
        return None;
    }
    let id = id.expect("checked id");

    match dispatch_method(
        method,
        request.get("params").cloned().unwrap_or(Value::Null),
    ) {
        Ok(result) => Some(json!({"jsonrpc": "2.0", "id": id, "result": result})),
        Err(err) => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": json_rpc_error_code(&err),
                "message": err.to_string(),
                "data": {"code": err.code()}
            }
        })),
    }
}

fn dispatch_method(method: &str, params: Value) -> Result<Value, error::McpError> {
    match method {
        "initialize" => {
            // Echo a protocol version the client understands when possible.
            // Cursor may send newer versions; we still speak the 2024-11-05 tool surface.
            let requested = params
                .get("protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or("2024-11-05");
            let protocol_version = match requested {
                "2024-11-05" | "2025-03-26" | "2025-06-18" => requested,
                _ => "2024-11-05",
            };
            Ok(json!({
                "protocolVersion": protocol_version,
                "serverInfo": {
                    "name": MCP_SERVER_NAME,
                    "version": env!("CARGO_PKG_VERSION")
                },
                "capabilities": {
                    "tools": {}
                }
            }))
        }
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({
            "tools": tool_names()
                .iter()
                .map(|name| json!({
                    "name": name,
                    "description": tool_description(name),
                    "inputSchema": tool_input_schema(name).unwrap_or_else(|| json!({"type": "object"}))
                }))
                .collect::<Vec<_>>()
        })),
        "tools/call" => call_tool(params),
        // Cursor probes these after initialize; empty lists keep handshake healthy.
        "resources/list" => Ok(json!({"resources": []})),
        "prompts/list" => Ok(json!({"prompts": []})),
        _ => Err(error::McpError::InvalidArgs(format!(
            "unknown method `{method}`"
        ))),
    }
}

fn call_tool(params: Value) -> Result<Value, error::McpError> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| error::McpError::InvalidArgs("tools/call missing name".to_string()))?;
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let payload = match name {
        "artifact_create" => {
            let req: CreateRequest = serde_json::from_value(arguments)
                .map_err(|err| error::McpError::InvalidArgs(err.to_string()))?;
            let result = tools::create::run(
                &req.kind,
                &PathBuf::from(&req.dir),
                tools::create::CreateOptions {
                    title: req.title.as_deref(),
                    visual_brief: &req.visual_brief,
                    overwrite: req.overwrite,
                },
            )?;
            serde_json::to_value(CreateResponse {
                artifact: tools::dto::CreateArtifactDto {
                    kind: result.artifact.kind.slug().to_string(),
                    root: path_string(&result.artifact.root),
                    primary_file: result.artifact.kind.primary_file().to_string(),
                    handoff: path_string(&result.artifact.handoff_path()),
                },
            })
            .expect("create response serializes")
        }
        "artifact_preview" => {
            let req: PreviewRequest = serde_json::from_value(arguments)
                .map_err(|err| error::McpError::InvalidArgs(err.to_string()))?;
            let dir = PathBuf::from(&req.dir);
            let result = tools::preview::run(tools::preview::PreviewOptions {
                dir: &dir,
                external_browser: req.external_browser,
                watch: req.watch,
            })?;
            serde_json::to_value(PreviewResponse {
                started: result.started,
                mode: result.mode.to_string(),
            })
            .expect("preview response serializes")
        }
        "artifact_handoff" => {
            let req: HandoffRequest = serde_json::from_value(arguments)
                .map_err(|err| error::McpError::InvalidArgs(err.to_string()))?;
            let dir = PathBuf::from(&req.dir);
            let result = tools::handoff::run(tools::handoff::HandoffOptions {
                dir: &dir,
                agent: &req.agent,
                write: req.write,
            })?;
            serde_json::to_value(HandoffResponse {
                path: path_string(&result.path),
                content: result.content,
            })
            .expect("handoff response serializes")
        }
        "artifact_export" => {
            let req: ExportRequest = serde_json::from_value(arguments)
                .map_err(|err| error::McpError::InvalidArgs(err.to_string()))?;
            let dir = PathBuf::from(&req.dir);
            let out = req.out.as_ref().map(PathBuf::from);
            let result = tools::export::run(tools::export::ExportToolOptions {
                dir: &dir,
                format: req.format.as_str(),
                out: out.as_deref(),
            })?;
            serde_json::to_value(ExportResponse {
                out: path_string(&result.out),
                format: result.format.as_str().to_string(),
            })
            .expect("export response serializes")
        }
        _ => {
            return Err(error::McpError::InvalidArgs(format!(
                "unknown tool `{name}`"
            )))
        }
    };

    Ok(json!({
        "content": [{"type": "text", "text": payload.to_string()}],
        "structuredContent": payload
    }))
}

fn tool_description(name: &str) -> &'static str {
    match name {
        "artifact_create" => "Create an artifact using the shared od-core creation pipeline.",
        "artifact_preview" => "Required preview path for coding agents: start the persistent live-reloading odl preview window for an artifact dir. Call this instead of opening a system browser (no start/xdg-open/open/Playwright). Defaults: externalBrowser=false (webview window), watch=true.",
        "artifact_handoff" => "Read or generate artifact handoff.md content.",
        "artifact_export" => "Export an artifact as html, md, zip, or pdf.",
        _ => "Open Design Lite tool.",
    }
}

fn json_rpc_error_code(err: &error::McpError) -> i64 {
    match err {
        error::McpError::InvalidArgs(_) => -32602,
        error::McpError::NotImplemented(_) => -32601,
        error::McpError::FormatUnsupported(_) => -32001,
        error::McpError::PdfBackendMissing(_) => -32002,
        _ => -32000,
    }
}

fn path_string(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_list_includes_create() {
        let response = handle_json_rpc(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list"
        }))
        .unwrap();

        assert_eq!(response["result"]["tools"][0]["name"], "artifact_create");
    }

    #[test]
    fn invalid_tool_call_returns_json_rpc_error_with_code() {
        let response = handle_json_rpc(json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {"name": "artifact_create", "arguments": {"kind": "pdf", "dir": "."}}
        }))
        .unwrap();

        assert_eq!(response["error"]["data"]["code"], "invalid_args");
    }

    #[test]
    fn content_length_header_is_case_insensitive() {
        assert_eq!(content_length_value("Content-Length: 12"), Some("12"));
        assert_eq!(content_length_value("content-length: 12"), Some("12"));
        assert_eq!(content_length_value("CONTENT-LENGTH:12"), Some("12"));
        assert_eq!(content_length_value("Accept: application/json"), None);
    }

    #[test]
    fn tool_schemas_are_inline_without_refs() {
        for name in tool_names() {
            let schema = tool_input_schema(name).expect("schema");
            let text = schema.to_string();
            assert!(!text.contains("\"$ref\""), "{name} schema has $ref");
            assert!(
                !text.contains("definitions"),
                "{name} schema has definitions"
            );
        }
    }
}
