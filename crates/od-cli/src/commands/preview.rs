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
    install_panic_hook(root);
    let options = PreviewOptions {
        artifact_root: root.to_path_buf(),
        external_browser,
        watch,
        devtools,
    };
    preview(&options)?;
    Ok(())
}

/// panic 信息落 `.odl/preview.log`：MCP 从子进程 stderr 重定向也能收到
/// 默认 panic 输出，但直接在终端跑 CLI 时 stderr 不落盘，这里补一份，
/// 保证「预览为什么崩」总能在日志里找到（spec: preview.md 稳定性）。
fn install_panic_hook(root: &Path) {
    use std::io::Write as _;
    let log = root.join(".odl").join("preview.log");
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Some(parent) = log.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
        {
            let _ = writeln!(f, "[panic] {info}");
            let _ = writeln!(f, "{}", std::backtrace::Backtrace::force_capture());
        }
        default_hook(info);
    }));
}
