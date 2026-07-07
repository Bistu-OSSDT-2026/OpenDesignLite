//! 文件监听 + debounce（notify）。签名占位。
//!
//! Spec: docs/specs/preview.md（文件监听）

use crate::PreviewError;
use std::path::Path;
use std::time::Duration;

/// 默认 debounce（preview spec 允许 50–200ms）。
pub const DEFAULT_DEBOUNCE: Duration = Duration::from_millis(100);

/// 监听时忽略的目录/文件片段。
pub const IGNORED: [&str; 2] = [".git", ".log"];

/// 监听 artifact root，变更（debounce 后）触发回调。M1 接入 notify。
pub fn watch(_root: &Path, _on_change: impl FnMut()) -> Result<(), PreviewError> {
    todo!("M1: notify watcher with debounce; see docs/specs/preview.md")
}
