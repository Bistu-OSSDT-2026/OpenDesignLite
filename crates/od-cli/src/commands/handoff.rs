//! `odl handoff`：生成或刷新 handoff.md（M1）。
//!
//! Spec: docs/specs/cli.md, handoff.md

use od_core::{OdError, Result};
use std::path::Path;

pub fn run(_root: &Path, _agent: &str, _stdout: bool) -> Result<()> {
    Err(OdError::NotImplemented("odl handoff (M1)"))
}
