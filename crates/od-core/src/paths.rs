//! 路径工具：manifest 路径统一为 `/`；防止 `..` 越出 artifact root。
//!
//! Spec: docs/specs/artifact-workspace.md（路径规则）

use crate::error::{OdError, Result};
use std::path::{Component, Path};

/// 把相对路径转成 manifest 使用的 `/` 分隔形式。
pub fn to_manifest_path(rel: &Path) -> String {
    rel.components()
        .filter_map(|c| match c {
            Component::Normal(s) => s.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// 确认 `child` 不通过 `..` 越出 artifact root。M1 会做更严格的规范化校验。
pub fn ensure_within(root: &Path, child: &Path) -> Result<()> {
    let _ = root;
    if child.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(OdError::PathEscape(child.to_path_buf()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn manifest_path_uses_forward_slash() {
        let p: PathBuf = ["assets", "od-design.css"].iter().collect();
        assert_eq!(to_manifest_path(&p), "assets/od-design.css");
    }

    #[test]
    fn rejects_parent_escape() {
        assert!(ensure_within(Path::new("/root"), Path::new("../secret")).is_err());
        assert!(ensure_within(Path::new("/root"), Path::new("assets/x.css")).is_ok());
    }
}
