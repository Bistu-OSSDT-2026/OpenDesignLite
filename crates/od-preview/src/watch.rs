//! 文件监听 + debounce（notify）。
//!
//! Spec: docs/specs/preview.md（文件监听）

use crate::PreviewError;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// 默认 debounce（preview spec 允许 50–200ms）。
pub const DEFAULT_DEBOUNCE: Duration = Duration::from_millis(100);

/// 监听时忽略的目录/文件片段。
pub const IGNORED: [&str; 2] = [".git", ".log"];

/// 监听 artifact root，变更（debounce 后）触发回调。阻塞当前线程。
///
/// 忽略 `.git/`、`.log/`、临时文件与编辑器 swap 文件。watcher 初始化失败返回
/// `WatchFailed`，调用方可继续无 watch 预览。
pub fn watch(root: &Path, mut on_change: impl FnMut()) -> Result<(), PreviewError> {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                // 过滤被忽略的路径。
                let ignored = event.paths.iter().any(|p| {
                    let s = p.to_string_lossy();
                    IGNORED.iter().any(|ig| s.contains(ig)) || is_temp_or_swap(&s)
                });
                if !ignored {
                    let _ = tx.send(Instant::now());
                }
            }
        },
        notify::Config::default(),
    )
    .map_err(|e| PreviewError::WatchFailed(e.to_string()))?;

    watcher
        .watch(root, RecursiveMode::Recursive)
        .map_err(|e| PreviewError::WatchFailed(e.to_string()))?;

    let mut last_fired: Option<Instant> = None;
    while let Ok(t) = rx.recv() {
        // debounce：距上次触发不足 DEFAULT_DEBOUNCE 则跳过。
        if let Some(last) = last_fired {
            if t.duration_since(last) < DEFAULT_DEBOUNCE {
                continue;
            }
        }
        last_fired = Some(t);
        on_change();
    }
    Ok(())
}

/// 判断是否为临时/swap 文件（编辑器或系统生成）。
fn is_temp_or_swap(path: &str) -> bool {
    let name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    name.starts_with('.')
        && (name.ends_with(".swp")
            || name.ends_with(".swo")
            || name.ends_with(".tmp")
            || name.ends_with("~"))
}
