//! 工作区目录布局辅助。
//!
//! Spec: docs/specs/artifact-workspace.md（工作区布局）

use std::path::{Path, PathBuf};

pub fn workspace_manifest_path(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("manifest.json")
}

pub fn artifacts_dir(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("artifacts")
}

pub fn skills_dir(root: impl AsRef<Path>) -> PathBuf {
    root.as_ref().join("skills")
}

/// 从 `start` 向上查找最近含 workspace `manifest.json` 的祖先目录。
///
/// 用于定位 workspace `<ws>/skills/` 覆盖目录。找不到返回 `None`。
pub fn find_workspace_root(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_absolute() {
        start.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(start)
    };
    current = current.canonicalize().unwrap_or(current);
    loop {
        if workspace_manifest_path(&current).exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_workspace_root_walks_up() {
        let tmp = std::env::temp_dir().join("od-ws-root-test");
        let _ = std::fs::remove_dir_all(&tmp);
        let nested = tmp.join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(workspace_manifest_path(&tmp), "{}").unwrap();

        let found = find_workspace_root(&nested).unwrap();
        // canonicalize 后比较，避免大小写/前缀差异。
        assert_eq!(found.canonicalize().unwrap(), tmp.canonicalize().unwrap());

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn find_workspace_root_none_when_no_manifest() {
        let tmp = std::env::temp_dir().join("od-ws-root-none");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        assert!(find_workspace_root(&tmp).is_none());
        std::fs::remove_dir_all(&tmp).ok();
    }
}
