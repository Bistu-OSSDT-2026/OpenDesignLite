//! 文件监听（notify）。M1 第一版：先跑通监听，debounce 由 wry reload 自身节流兜底。
//!
//! Spec: docs/specs/preview.md（文件监听）

use crate::PreviewError;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// 默认 debounce（preview spec 允许 50–200ms）。M1 暂未精确实现，留作后续优化。
pub const DEFAULT_DEBOUNCE: Duration = Duration::from_millis(100);

/// 监听时忽略的目录/文件片段。preview 自己生成的临时 HTML 也必须忽略，
/// 否则 Markdown 重渲染会自触发，形成死循环。
pub const IGNORED: [&str; 3] = [".git", ".log", ".odl"];

/// 监听 artifact root，返回 watcher 供调用方在主线程保活。
///
/// **关键**：notify 的 `RecommendedWatcher` 必须在创建它的线程里保活，
/// 不能搬到子线程（否则事件不派发，回调永不触发）。
/// 所以本函数不起后台线程，而是返回 watcher，由 `preview()` 在事件循环
/// 所在的主线程持有到程序结束。
///
/// `on_change` 约束 `Send + 'static`：notify 的回调在它的内部线程跑，
/// 闭包必须能跨线程移动且不借用主线程的局部变量。
pub fn watch(
    root: &Path,
    mut on_change: impl FnMut() + Send + 'static,
) -> Result<RecommendedWatcher, PreviewError> {
    let root: PathBuf = root.to_path_buf();

    let mut watcher = RecommendedWatcher::new(
        move |res: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                // 只关心内容修改 / 新建，忽略 Remove / Access 等噪声。
                if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    return;
                }
                // 跳过被忽略路径：按路径段匹配，不是子串匹配。
                // 否则 `.odl` 规则会误伤 `.odl-demo` 这种工作区目录名。
                let hit_ignored = event.paths.iter().any(|p| {
                    let segs: Vec<String> = p
                        .components()
                        .filter_map(|c| c.as_os_str().to_str())
                        .map(|s| s.to_string())
                        .collect();
                    IGNORED.iter().any(|ig| segs.iter().any(|seg| seg == ig))
                });
                if hit_ignored {
                    return;
                }
                on_change();
            }
        },
        notify::Config::default(),
    )
    .map_err(|e| PreviewError::WatchFailed(e.to_string()))?;

    // 用绝对路径监听：相对路径在 Windows 上 notify 可能不触发。
    let watch_root = root.canonicalize().unwrap_or(root);
    watcher
        .watch(&watch_root, RecursiveMode::Recursive)
        .map_err(|e| PreviewError::WatchFailed(e.to_string()))?;

    Ok(watcher)
}
