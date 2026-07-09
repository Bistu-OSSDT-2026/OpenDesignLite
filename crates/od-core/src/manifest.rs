//! manifest.json schema（serde）与读写。字段命名遵循 artifact-workspace spec 的 camelCase。
//!
//! Spec: docs/specs/artifact-workspace.md

use crate::design::{VisualBrief, KERNEL_VERSION, STYLESHEET_ASSET};
use crate::error::{OdError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// M1 固定为 1。
pub const SCHEMA_VERSION: u32 = 1;
const WORKSPACE_TYPE: &str = "workspace";
const ARTIFACT_TYPE: &str = "artifact";

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

impl WorkspaceManifest {
    /// 用当前 UTC 时间构造最小 workspace manifest。
    pub fn new(name: &str) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            r#type: WORKSPACE_TYPE.into(),
            name: name.into(),
            created_by: "odl".into(),
            created_at: now_iso(),
        }
    }
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

impl ArtifactManifest {
    /// 构造最小 artifact manifest，含 design kernel 元数据。
    pub fn new(kind_slug: &str, title: &str, primary_file: &str, brief: Option<&str>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            r#type: ARTIFACT_TYPE.into(),
            kind: kind_slug.into(),
            title: title.into(),
            primary_file: primary_file.into(),
            created_by: "odl".into(),
            created_at: now_iso(),
            design: Some(DesignMeta {
                kernel_version: KERNEL_VERSION,
                stylesheet: Some(STYLESHEET_ASSET.into()),
                visual_brief: brief.map(|b| b.into()),
            }),
        }
    }
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

/// 当前 UTC 时间的 RFC3339 字符串（manifest 的 `createdAt`）。
fn now_iso() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}

/// 读 workspace manifest 并校验 schemaVersion 与 type。
pub fn load_workspace(path: &Path) -> Result<WorkspaceManifest> {
    let raw = std::fs::read_to_string(path).map_err(|e| OdError::ManifestInvalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    let m: WorkspaceManifest =
        serde_json::from_str(&raw).map_err(|e| OdError::ManifestInvalid {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })?;
    validate(&m.schema_version, &m.r#type, WORKSPACE_TYPE, path)?;
    Ok(m)
}

/// 读 artifact manifest 并校验 schemaVersion 与 type。
pub fn load_artifact(path: &Path) -> Result<ArtifactManifest> {
    let raw = std::fs::read_to_string(path).map_err(|e| OdError::ManifestInvalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    let m: ArtifactManifest = serde_json::from_str(&raw).map_err(|e| OdError::ManifestInvalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    validate(&m.schema_version, &m.r#type, ARTIFACT_TYPE, path)?;
    Ok(m)
}

/// 写 workspace manifest（pretty JSON）。
pub fn save_workspace(m: &WorkspaceManifest, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(m).map_err(|e| OdError::ManifestInvalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    std::fs::write(path, format!("{json}\n"))?;
    Ok(())
}

/// 写 artifact manifest（pretty JSON）。
pub fn save_artifact(m: &ArtifactManifest, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(m).map_err(|e| OdError::ManifestInvalid {
        path: path.to_path_buf(),
        reason: e.to_string(),
    })?;
    std::fs::write(path, format!("{json}\n"))?;
    Ok(())
}

fn validate(schema: &u32, ty: &str, expected_ty: &str, path: &Path) -> Result<()> {
    if *schema != SCHEMA_VERSION {
        return Err(OdError::ManifestInvalid {
            path: path.to_path_buf(),
            reason: format!("unsupported schemaVersion {schema} (expected {SCHEMA_VERSION})"),
        });
    }
    if ty != expected_ty {
        return Err(OdError::ManifestInvalid {
            path: path.to_path_buf(),
            reason: format!("type `{ty}` does not match expected `{expected_ty}`"),
        });
    }
    Ok(())
}

/// visual brief 字符串 → `VisualBrief`（manifest `design.visualBrief` 反序列化用）。
pub fn brief_from_str(value: &str) -> Option<VisualBrief> {
    VisualBrief::parse(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_save_load_roundtrip() {
        let tmp = std::env::temp_dir().join("od-manifest-ws-rt");
        let path = tmp.join("manifest.json");
        std::fs::create_dir_all(&tmp).unwrap();
        let m = WorkspaceManifest::new("test workspace");
        save_workspace(&m, &path).unwrap();
        let loaded = load_workspace(&path).unwrap();
        assert_eq!(loaded.schema_version, 1);
        assert_eq!(loaded.r#type, "workspace");
        assert_eq!(loaded.name, "test workspace");
        assert_eq!(loaded.created_by, "odl");
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn artifact_save_load_roundtrip_with_design() {
        let tmp = std::env::temp_dir().join("od-manifest-art-rt");
        let path = tmp.join("manifest.json");
        std::fs::create_dir_all(&tmp).unwrap();
        let m = ArtifactManifest::new("html", "My Page", "index.html", Some("editorial"));
        save_artifact(&m, &path).unwrap();
        let loaded = load_artifact(&path).unwrap();
        assert_eq!(loaded.kind, "html");
        assert_eq!(loaded.title, "My Page");
        assert_eq!(loaded.primary_file, "index.html");
        let design = loaded.design.expect("design present");
        assert_eq!(design.kernel_version, KERNEL_VERSION);
        assert_eq!(design.stylesheet.as_deref(), Some(STYLESHEET_ASSET));
        assert_eq!(design.visual_brief.as_deref(), Some("editorial"));
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn wrong_schema_version_rejected() {
        let tmp = std::env::temp_dir().join("od-manifest-bad-schema");
        let path = tmp.join("manifest.json");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(&path, "{\"schemaVersion\":99,\"type\":\"workspace\",\"name\":\"x\",\"createdBy\":\"odl\",\"createdAt\":\"t\"}").unwrap();
        let err = load_workspace(&path).unwrap_err();
        assert_eq!(err.code(), "manifest_invalid");
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn wrong_type_rejected() {
        let tmp = std::env::temp_dir().join("od-manifest-bad-type");
        let path = tmp.join("manifest.json");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(&path, "{\"schemaVersion\":1,\"type\":\"artifact\",\"name\":\"x\",\"createdBy\":\"odl\",\"createdAt\":\"t\"}").unwrap();
        assert!(load_workspace(&path).is_err());
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn primary_file_uses_forward_slash() {
        // manifest 的 primaryFile 必须是 `/` 分隔；构造时直接传字符串，确保不出现反斜杠。
        let m = ArtifactManifest::new("html", "t", "index.html", None);
        assert!(!m.primary_file.contains('\\'));
        assert_eq!(m.primary_file, "index.html");
    }
}
