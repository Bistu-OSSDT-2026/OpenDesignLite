//! MCP 工具的 JSON 请求/响应 DTO（对齐 docs/specs/mcp.md 的字段形状）。
//!
//! 模块级预备：这里只定义数据结构与 serde/schemars 派生，**不引用任何
//! `run()` 实现**，也不起 server。server 外壳（组长协调）拿这些 struct
//! 注册 rmcp handler 时，再做 DTO ↔ `run()` 入参的翻译。
//!
//! 字段命名一律 `#[serde(rename_all = "camelCase")]`，与 mcp.md 的 JSON
//! 示例（`visualBrief` / `externalBrowser` / `primaryFile`）逐一对齐。
//!
//! Spec: docs/specs/mcp.md（各 tool 输入/输出）

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ─────────────────────────── artifact_create ───────────────────────────

/// `artifact_create` 输入。Spec: mcp.md（artifact_create）。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    /// `html` | `docs` | `slides`。
    pub kind: String,
    /// artifact 根目录（绝对或相对）。
    pub dir: String,
    /// 可选标题；缺省时回退到目录名。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// `editorial` | `studio` | `workbench`；无法解析时回退。
    #[serde(default)]
    pub visual_brief: String,
    /// 目标已存在时是否覆盖。
    #[serde(default)]
    pub overwrite: bool,
}

/// `artifact_create` 输出里的 artifact 对象。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateArtifactDto {
    pub kind: String,
    pub root: String,
    pub primary_file: String,
    pub handoff: String,
}

/// `artifact_create` 输出。Spec: mcp.md（输出 `{artifact: {...}}`）。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateResponse {
    pub artifact: CreateArtifactDto,
}

// ─────────────────────────── artifact_preview ──────────────────────────

/// `artifact_preview` 输入。Spec: mcp.md（artifact_preview）。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreviewRequest {
    pub dir: String,
    /// `true` → 外部浏览器（对应 CLI `--external-browser`）。
    #[serde(default)]
    pub external_browser: bool,
    /// `true` → 监听变更自动刷新（默认开）。
    #[serde(default = "default_true")]
    pub watch: bool,
}

/// `artifact_preview` 输出。`mode` = `webview` | `external`。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PreviewResponse {
    pub started: bool,
    pub mode: String,
}

// ─────────────────────────── artifact_handoff ──────────────────────────

/// `artifact_handoff` 输入。Spec: mcp.md（artifact_handoff）。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HandoffRequest {
    pub dir: String,
    /// `generic` | `opencode` | `claude-code` | `codex`。
    #[serde(default = "default_agent")]
    pub agent: String,
    /// `true` → 覆盖重写 handoff.md；`false` → 只读。
    #[serde(default)]
    pub write: bool,
}

/// `artifact_handoff` 输出。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HandoffResponse {
    pub path: String,
    pub content: String,
}

// ─────────────────────────── artifact_export ───────────────────────────

/// `artifact_export` 输入。M4 前未实现；DTO 先占位以保持 tools 一致性。
/// 字段参照 CLI `odl export --format <f> --out <path>` 推断，待 spec 补齐。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub dir: String,
    pub format: ExportFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out: Option<String>,
}

/// 导出格式枚举。M4 再定；`other` 用 `#[serde(other)]` 捕获未知值
/// （不保留原始字符串，仅保证反序列化不报错）。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Pdf,
    Html,
    Pptx,
    /// spec 未覆盖的格式。不保留原始值，M4 补齐后再扩展。
    #[serde(other)]
    Other,
}

/// `artifact_export` 输出。M4 前无输出，占位。
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExportResponse {
    pub out: String,
}

// ───────────────────────────── 默认值 ──────────────────────────────────

fn default_true() -> bool {
    true
}

fn default_agent() -> String {
    "generic".to_string()
}

// ───────────────────────────── 测试 ──────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// create request 从 mcp.md JSON 反序列化（camelCase 对齐）。
    #[test]
    fn create_request_roundtrip() {
        let json = r#"{
            "kind": "html",
            "dir": "D:/work/demo",
            "title": "Demo",
            "visualBrief": "editorial",
            "overwrite": false
        }"#;
        let req: CreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.kind, "html");
        assert_eq!(req.visual_brief, "editorial");
        assert_eq!(req.title, Some("Demo".into()));
        assert!(!req.overwrite);

        let back = serde_json::to_value(&req).unwrap();
        assert_eq!(back["kind"], "html");
        assert_eq!(back["visualBrief"], "editorial");
        assert_eq!(back["title"], "Demo");
    }

    /// create 输出形状与 mcp.md 对齐：`{artifact: {kind, root, primaryFile, handoff}}`。
    #[test]
    fn create_response_shape() {
        let resp = CreateResponse {
            artifact: CreateArtifactDto {
                kind: "html".into(),
                root: "D:/work/demo".into(),
                primary_file: "index.html".into(),
                handoff: "handoff.md".into(),
            },
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["artifact"]["kind"], "html");
        assert_eq!(json["artifact"]["primaryFile"], "index.html");
        assert_eq!(json["artifact"]["handoff"], "handoff.md");
    }

    /// preview request 从 mcp.md JSON 反序列化，包括 `watch` 默认值。
    #[test]
    fn preview_request_roundtrip() {
        // watch 默认 true（mcp.md "watch": true）
        let json = r#"{"dir": "D:/work/demo", "externalBrowser": false, "watch": true}"#;
        let req: PreviewRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.dir, "D:/work/demo");
        assert!(!req.external_browser);
        assert!(req.watch);

        // 缺省字段用默认值：watch → true
        let json2 = r#"{"dir": "."}"#;
        let req2: PreviewRequest = serde_json::from_str(json2).unwrap();
        assert!(req2.watch, "watch defaults to true");
        assert!(!req2.external_browser);
    }

    /// preview 输出形状：`{started: true, mode: "webview"}`。
    #[test]
    fn preview_response_shape() {
        let resp = PreviewResponse {
            started: true,
            mode: "webview".into(),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert!(json["started"].as_bool().unwrap());
        assert_eq!(json["mode"], "webview");
    }

    /// handoff request 从 mcp.md JSON 反序列化，含 agent 默认值。
    #[test]
    fn handoff_request_roundtrip() {
        let json = r#"{"dir": "D:/work/demo", "agent": "opencode", "write": true}"#;
        let req: HandoffRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.agent, "opencode");
        assert!(req.write);

        // agent 缺省 → "generic"
        let json2 = r#"{"dir": "."}"#;
        let req2: HandoffRequest = serde_json::from_str(json2).unwrap();
        assert_eq!(req2.agent, "generic");
        assert!(!req2.write);
    }

    /// handoff 输出形状：`{path, content}`。
    #[test]
    fn handoff_response_shape() {
        let resp = HandoffResponse {
            path: "D:/work/demo/handoff.md".into(),
            content: "# Handoff: Demo\n...".into(),
        };
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["path"], "D:/work/demo/handoff.md");
        assert!(json["content"].as_str().unwrap().starts_with("# Handoff:"));
    }

    /// export 请求反序列化（M4 前无 run，但 schema 要有，供 tools/list 一致性）。
    #[test]
    fn export_request_roundtrip() {
        let json = r#"{"dir": ".", "format": "pdf"}"#;
        let req: ExportRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(req.format, ExportFormat::Pdf));

        let json2 = r#"{"dir": ".", "format": "html"}"#;
        let req2: ExportRequest = serde_json::from_str(json2).unwrap();
        assert!(matches!(req2.format, ExportFormat::Html));

        // 未识别的格式 → Other（不报错）
        let json3 = r#"{"dir": ".", "format": "docx"}"#;
        let req3: ExportRequest = serde_json::from_str(json3).unwrap();
        assert!(matches!(req3.format, ExportFormat::Other));
    }

    /// 保证每个 DTO 生成 schemars schema 且能被序列化（非空）——供 tools/list 用。
    #[test]
    fn schemas_are_generated() {
        for (label, schema) in [
            (
                "CreateRequest",
                serde_json::to_value(schemars::schema_for!(CreateRequest)).unwrap(),
            ),
            (
                "CreateResponse",
                serde_json::to_value(schemars::schema_for!(CreateResponse)).unwrap(),
            ),
            (
                "PreviewRequest",
                serde_json::to_value(schemars::schema_for!(PreviewRequest)).unwrap(),
            ),
            (
                "PreviewResponse",
                serde_json::to_value(schemars::schema_for!(PreviewResponse)).unwrap(),
            ),
            (
                "HandoffRequest",
                serde_json::to_value(schemars::schema_for!(HandoffRequest)).unwrap(),
            ),
            (
                "HandoffResponse",
                serde_json::to_value(schemars::schema_for!(HandoffResponse)).unwrap(),
            ),
            (
                "ExportRequest",
                serde_json::to_value(schemars::schema_for!(ExportRequest)).unwrap(),
            ),
            (
                "ExportResponse",
                serde_json::to_value(schemars::schema_for!(ExportResponse)).unwrap(),
            ),
        ] {
            assert!(
                !schema.as_object().unwrap().is_empty(),
                "{label} schema empty"
            );
        }
    }

    /// lib.rs 的 tool_input_schema / tool_output_schema 能调度到正确的 schema。
    #[test]
    fn schema_dispatch() {
        for name in [
            "artifact_create",
            "artifact_preview",
            "artifact_handoff",
            "artifact_export",
        ] {
            assert!(
                crate::tool_input_schema(name).is_some(),
                "{name} input schema missing"
            );
            assert!(
                crate::tool_output_schema(name).is_some(),
                "{name} output schema missing"
            );
        }
        assert!(crate::tool_input_schema("ghost").is_none());
        assert!(crate::tool_output_schema("ghost").is_none());
    }
}
