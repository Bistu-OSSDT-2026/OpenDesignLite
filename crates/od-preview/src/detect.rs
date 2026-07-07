//! 主文件检测（复用 od-core 的 `PRIMARY_FILE_ORDER`）。
//!
//! Spec: docs/specs/preview.md（artifact 类型检测）

use od_core::artifact::{detect_kind, ArtifactKind, PRIMARY_FILE_ORDER};
use std::path::{Path, PathBuf};

/// 按 index.html → slides.html → doc.md 顺序返回主文件路径与 kind。
pub fn detect_primary(root: &Path) -> Option<(PathBuf, ArtifactKind)> {
    for name in PRIMARY_FILE_ORDER {
        let path = root.join(name);
        if path.exists() {
            if let Some(kind) = detect_kind(name) {
                return Some((path, kind));
            }
        }
    }
    None
}
