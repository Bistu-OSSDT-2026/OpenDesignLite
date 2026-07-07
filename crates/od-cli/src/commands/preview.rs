//! `odl preview`：定位主文件（M1 由 od-preview 打开 WebView）。
//!
//! Spec: docs/specs/cli.md, preview.md

use od_core::artifact::PRIMARY_FILE_ORDER;
use od_core::{OdError, Result};
use std::path::{Path, PathBuf};

/// M0 行为保留：按检测顺序返回可预览主文件。
/// M1 补齐：调用 `od_preview::preview(PreviewOptions)` 打开窗口并监听。
pub fn run(root: &Path) -> Result<PathBuf> {
    for candidate in PRIMARY_FILE_ORDER {
        let path = root.join(candidate);
        if path.exists() {
            return Ok(path);
        }
    }
    Err(OdError::PrimaryFileMissing(root.join("index.html")))
}
