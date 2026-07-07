//! `odl export`：导出 artifact（M4）。占位返回 not_implemented（退出码 10）。
//!
//! Spec: docs/specs/cli.md, export.md

use od_core::{OdError, Result};
use std::path::Path;

pub fn run(_root: &Path, _format: &str) -> Result<()> {
    Err(OdError::NotImplemented("odl export (M4)"))
}
