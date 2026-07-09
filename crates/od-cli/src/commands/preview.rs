//! `odl preview`：委托给 od-preview 打开原生预览窗口。
//!
//! CLI 只负责把命令行参数翻译成 `PreviewOptions`，detect / render / webview / watch
//! 全部交给 od-preview。本函数不再自己 detect 主文件——那是预览层的职责
//!（见 docs/architecture/boundaries.md 归属表）。
//!
//! Spec: docs/specs/cli.md, preview.md

use crate::error::Result;
use od_preview::{preview, PreviewOptions};
use std::path::Path;

/// 构造 `PreviewOptions` 并调用 `od_preview::preview`。
///
/// `no_watch` 是用户视角的否定式 flag（`--no-watch`），这里翻转成
/// `PreviewOptions.watch`（肯定式，对代码友好）。
pub fn run(root: &Path, external_browser: bool, watch: bool, devtools: bool) -> Result<()> {
    let options = PreviewOptions {
        artifact_root: root.to_path_buf(),
        external_browser,
        watch,
        devtools,
    };
    preview(&options)?;
    Ok(())
}
