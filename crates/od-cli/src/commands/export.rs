//! `odl export`：导出 artifact（html / md / zip / pdf）。
//!
//! Spec: docs/specs/cli.md, export.md

use od_core::export::{self, ExportFormat, ExportOptions, ExportResult};
use od_core::Result;
use std::path::Path;

pub fn run(root: &Path, format: &str, out: Option<&Path>) -> Result<ExportResult> {
    let format = ExportFormat::parse(format)?;
    export::run(root, format, ExportOptions { out })
}
