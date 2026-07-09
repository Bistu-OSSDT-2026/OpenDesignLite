//! `artifact_export` → 等价 `odl export`，转发到 od-core。
//!
//! Spec: docs/specs/mcp.md（artifact_export）, export.md

use crate::error::McpError;
use od_core::export::{self, ExportFormat, ExportOptions, ExportResult};
use od_core::OdError;
use std::path::Path;

/// `artifact_export` 输入（对齐 docs/specs/mcp.md）。
pub struct ExportToolOptions<'a> {
    pub dir: &'a Path,
    pub format: &'a str,
    pub out: Option<&'a Path>,
}

/// 导出 artifact。对应 CLI `odl export`。
pub fn run(options: ExportToolOptions<'_>) -> Result<ExportResult, McpError> {
    let format = ExportFormat::parse(options.format).map_err(from_core_err)?;
    export::run(
        options.dir,
        format,
        ExportOptions { out: options.out },
    )
    .map_err(from_core_err)
}

fn from_core_err(err: OdError) -> McpError {
    match err {
        OdError::FormatUnsupported(msg) => McpError::FormatUnsupported(msg),
        OdError::ExportFailed(msg) => McpError::ExportFailed(msg),
        OdError::PdfBackendMissing(msg) => McpError::PdfBackendMissing(msg),
        OdError::ResourceMissing(msg) => McpError::ResourceMissing(msg),
        OdError::PrimaryFileMissing(p) => {
            McpError::ArtifactNotFound(format!("primary file missing at {}", p.display()))
        }
        OdError::WorkspaceNotFound(p) => {
            McpError::ArtifactNotFound(format!("artifact not found at {}", p.display()))
        }
        OdError::ManifestInvalid { path, reason } => {
            McpError::ManifestInvalid(format!("{}: {reason}", path.display()))
        }
        OdError::Io(e) => McpError::InvalidArgs(format!("io error: {e}")),
        other => McpError::InvalidArgs(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use od_core::create::{self, CreateOptions};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!("od-mcp-export-{label}-{nanos}"));
        let _ = fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn zip_export_works() {
        let root = temp_root("zip");
        create::run(
            "html",
            &root,
            CreateOptions {
                title: Some("Zip"),
                visual_brief: "editorial",
                embed_css: false,
                overwrite: false,
            },
        )
        .unwrap();
        let out = root.parent().unwrap().join(format!(
            "mcp-export-{}.zip",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let result = run(ExportToolOptions {
            dir: &root,
            format: "zip",
            out: Some(&out),
        })
        .unwrap();
        assert!(result.out.exists());
        let _ = fs::remove_file(&out);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn unsupported_format_maps_error() {
        let root = temp_root("bad");
        create::run(
            "html",
            &root,
            CreateOptions {
                title: Some("Bad"),
                visual_brief: "editorial",
                embed_css: false,
                overwrite: false,
            },
        )
        .unwrap();
        let err = run(ExportToolOptions {
            dir: &root,
            format: "pptx",
            out: None,
        })
        .unwrap_err();
        assert_eq!(err.code(), "format_unsupported");
        let _ = fs::remove_dir_all(&root);
    }
}
