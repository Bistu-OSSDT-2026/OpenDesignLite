//! manifest.json schema（serde）。字段命名遵循 artifact-workspace spec 的 camelCase。
//! 本次仅定义类型契约；实际读写在 M1 实现。
//!
//! Spec: docs/specs/artifact-workspace.md

use serde::{Deserialize, Serialize};

/// M1 固定为 1。
pub const SCHEMA_VERSION: u32 = 1;

/// 工作区 manifest。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    /// 固定为 `workspace`。
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// 产物 manifest。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: u32,
    /// 固定为 `artifact`。
    #[serde(rename = "type")]
    pub r#type: String,
    /// `html` | `docs` | `slides`。
    pub kind: String,
    pub title: String,
    #[serde(rename = "primaryFile")]
    pub primary_file: String,
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub design: Option<DesignMeta>,
}

/// design kernel 元数据（artifact manifest 的 `design` 字段）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignMeta {
    #[serde(rename = "kernelVersion")]
    pub kernel_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stylesheet: Option<String>,
    #[serde(rename = "visualBrief", skip_serializing_if = "Option::is_none")]
    pub visual_brief: Option<String>,
}
