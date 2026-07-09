//! `odl preview`：调用 od-preview 打开预览窗口并监听文件变更。
//!
//! Spec: docs/specs/cli.md, preview.md

use od_preview::{preview, PreviewError, PreviewOptions};
use std::path::Path;

pub fn run(
    dir: &Path,
    external_browser: bool,
    no_watch: bool,
    devtools: bool,
) -> std::result::Result<(), PreviewError> {
    let opts = PreviewOptions {
        artifact_root: dir.to_path_buf(),
        external_browser,
        watch: !no_watch,
        devtools,
    };
    preview(&opts)
}
