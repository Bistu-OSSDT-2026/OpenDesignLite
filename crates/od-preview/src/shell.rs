//! 预览壳（深色 chrome + 固定视口 iframe + design agent 面板占位）。
//!
//! custom protocol 布局：
//! - `/index.html`            → 内嵌壳页面（不落盘，不依赖 artifact 文件）
//! - `/artifact/<rel>`        → `artifact_root/<rel>`（Markdown 的入口映射到
//!                              `.odl/preview.html`）；`<rel>` 规范化后必须仍在
//!                              artifact_root 内，越权返回 404
//!
//! Windows 上 wry 把 `odl-shell://shell/...` 映射为 `http://odl-shell.shell/...`
//!（同源，iframe 与相对资源天然可用；examples/shell_spike.rs 已验证）。
//!
//! 安全规则：壳与产物页面都不获得 native IPC；`__odlShellBridge` 只是
//! M4 design agent 的空占位对象（ADR-0003 2026-07 修订）。
//!
//! Spec: docs/specs/preview.md（加载策略 / 窗口与视口 / 预留位置）

use od_core::ArtifactKind;
use std::borrow::Cow;
use std::path::PathBuf;
use wry::http::{header::CONTENT_TYPE, Request, Response, StatusCode};

/// 壳顶栏高度（CSS 像素）。窗口高度 = 视口高度 + 此常量，保证内容区精确。
pub const SHELL_HEADER_HEIGHT: u32 = 44;

/// 按产物类型的固定内容区（= 部署后浏览器视口）尺寸。
///
/// Spec: docs/specs/preview.md（窗口与视口）
pub fn viewport_for(kind: ArtifactKind) -> (u32, u32) {
    match kind {
        ArtifactKind::Slides => (1280, 720),
        _ => (1366, 768),
    }
}

/// 壳页面入口 URL（平台差异见模块注释）。
pub fn entry_url() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "http://odl-shell.shell/index.html"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "odl-shell://shell/index.html"
    }
}

/// 供 reload 信号调用的壳内 JS：闪一下状态点并强制刷新 iframe（带时间戳防缓存）。
pub const RELOAD_SCRIPT: &str = "window.__odlShell && window.__odlShell.reload();";

pub struct ShellConfig {
    pub artifact_root: PathBuf,
    pub kind: ArtifactKind,
    pub title: String,
}

/// 从两种 URL 形式里取出 path 并去掉 query：
/// - macOS/Linux: `odl-shell://shell/artifact/index.html?t=1`
/// - Windows:     `http://odl-shell.shell/artifact/index.html?t=1`
fn request_path(uri: &str) -> String {
    let after_scheme = uri.split("://").nth(1).unwrap_or(uri);
    let path = match after_scheme.find('/') {
        Some(idx) => &after_scheme[idx..],
        None => "/",
    };
    match path.find(['?', '#']) {
        Some(idx) => path[..idx].to_string(),
        None => path.to_string(),
    }
}

fn mime_for(path: &str) -> &'static str {
    let ext = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "txt" | "md" => "text/plain; charset=utf-8",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}

fn response(
    status: StatusCode,
    mime: &str,
    body: impl Into<Cow<'static, [u8]>>,
) -> Response<Cow<'static, [u8]>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, mime)
        .body(body.into())
        .expect("response builds")
}

fn not_found() -> Response<Cow<'static, [u8]>> {
    response(
        StatusCode::NOT_FOUND,
        "text/plain",
        &b"not found"[..],
    )
}

/// `/artifact/<rel>` → 磁盘文件。越权（`..`、绝对路径、symlink 逃逸）一律 404，
/// 校验模式与 od-core export 的 `strip_prefix` 同源：canonicalize 后必须仍在
/// artifact_root 内。
fn serve_artifact_file(config: &ShellConfig, rel: &str) -> Response<Cow<'static, [u8]>> {
    // 入口重定向：Markdown 的“主文件”是渲染产物 .odl/preview.html。
    let rel = if config.kind == ArtifactKind::Markdown && rel == config.kind.primary_file() {
        ".odl/preview.html".to_string()
    } else {
        rel.to_string()
    };

    // 显式拒绝可疑成分，canonicalize 兜底（对付 symlink）。
    if rel.contains("..") || rel.starts_with('/') || rel.contains(':') {
        return not_found();
    }
    let Ok(root) = config.artifact_root.canonicalize() else {
        return not_found();
    };
    let candidate = root.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    let Ok(resolved) = candidate.canonicalize() else {
        return not_found();
    };
    if !resolved.starts_with(&root) {
        return not_found();
    }
    match std::fs::read(&resolved) {
        Ok(bytes) => response(StatusCode::OK, mime_for(&rel), bytes),
        Err(_) => not_found(),
    }
}

/// custom protocol 请求分发。
pub fn handle_request(config: &ShellConfig, request: &Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let path = request_path(&request.uri().to_string());
    match path.as_str() {
        "/" | "/index.html" => response(
            StatusCode::OK,
            "text/html; charset=utf-8",
            shell_html(config).into_bytes(),
        ),
        p => match p.strip_prefix("/artifact/") {
            Some(rel) if !rel.is_empty() => serve_artifact_file(config, rel),
            _ => not_found(),
        },
    }
}

/// 生成壳页面。深色 chrome + 精确视口 iframe + 可折叠 design agent 面板占位。
fn shell_html(config: &ShellConfig) -> String {
    let (vw, vh) = viewport_for(config.kind);
    let kind = config.kind.slug();
    let title = html_escape(&config.title);
    let entry = format!("/artifact/{}", config.kind.primary_file());
    format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>{title} — Open Design Lite</title>
<style>
  :root {{ color-scheme: dark; }}
  * {{ box-sizing: border-box; }}
  body {{ margin: 0; background: #101014; color: #e8e6e1; font-family: system-ui, sans-serif; overflow: hidden; }}
  header {{
    height: {header}px; display: flex; align-items: center; gap: 10px; padding: 0 14px;
    background: #17171c; border-bottom: 1px solid #26262c; user-select: none;
  }}
  .dot {{ width: 8px; height: 8px; border-radius: 50%; background: #3f3f46; transition: background .15s; flex: none; }}
  .dot.live {{ background: #7bd88f; }}
  .title {{ font-size: 13px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }}
  .kind {{ font-size: 11px; padding: 2px 8px; border-radius: 999px; background: #26262c; color: #b7b3ab; flex: none; }}
  .viewport {{ font-size: 11px; color: #6d6a64; margin-left: auto; flex: none; }}
  .agent-toggle {{
    flex: none; font-size: 11px; padding: 3px 10px; border-radius: 999px; border: 1px solid #2e2e35;
    background: transparent; color: #8a877f; cursor: pointer;
  }}
  .agent-toggle:hover {{ color: #c9c5bd; border-color: #4a4a52; }}
  main {{ position: relative; width: {vw}px; height: {vh}px; }}
  iframe#stage {{ display: block; width: {vw}px; height: {vh}px; border: 0; background: #fff; }}
  aside#agent-panel {{
    position: absolute; top: 0; right: 0; bottom: 0; width: 300px;
    background: #17171cf2; border-left: 1px solid #26262c;
    display: none; align-items: center; justify-content: center; text-align: center;
  }}
  aside#agent-panel.open {{ display: flex; }}
  aside#agent-panel .empty {{ color: #6d6a64; font-size: 12px; padding: 0 24px; line-height: 1.8; }}
</style>
</head>
<body>
<header>
  <span class="dot" id="reload-dot"></span>
  <span class="title">{title}</span>
  <span class="kind">{kind}</span>
  <span class="viewport">{vw}×{vh}</span>
  <button class="agent-toggle" id="agent-toggle" type="button">Design agent</button>
</header>
<main>
  <iframe id="stage" src="{entry}"></iframe>
  <aside id="agent-panel" aria-hidden="true">
    <div class="empty">Design agent<br>(coming soon)</div>
  </aside>
</main>
<script>
  // M4 design agent 桥接接口占位：仅空对象，无任何实现（ADR-0003）。
  window.__odlShellBridge = {{}};
  (function () {{
    var stage = document.getElementById('stage');
    var dot = document.getElementById('reload-dot');
    var panel = document.getElementById('agent-panel');
    document.getElementById('agent-toggle').addEventListener('click', function () {{
      panel.classList.toggle('open');
      panel.setAttribute('aria-hidden', panel.classList.contains('open') ? 'false' : 'true');
    }});
    stage.addEventListener('load', function () {{
      dot.classList.add('live');
    }});
    window.__odlShell = {{
      reload: function () {{
        dot.classList.remove('live');
        stage.src = '{entry}?t=' + Date.now();
      }}
    }};
  }})();
</script>
</body>
</html>
"##,
        header = SHELL_HEADER_HEIGHT,
    )
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_config(tag: &str, kind: ArtifactKind) -> (PathBuf, ShellConfig) {
        // tag 保证并行测试各用各的目录，避免互删竞态。
        let root = std::env::temp_dir().join(format!(
            "odl-shell-test-{tag}-{}-{}",
            kind.slug(),
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("assets")).unwrap();
        std::fs::write(root.join(kind.primary_file()), "<html>hi</html>").unwrap();
        std::fs::write(root.join("assets").join("x.css"), "body{}").unwrap();
        (
            root.clone(),
            ShellConfig {
                artifact_root: root,
                kind,
                title: "T".into(),
            },
        )
    }

    fn get(config: &ShellConfig, uri: &str) -> Response<Cow<'static, [u8]>> {
        let request = Request::builder().uri(uri).body(Vec::new()).unwrap();
        handle_request(config, &request)
    }

    /// 壳页面与产物文件都可达（两种 URL 形式）。
    #[test]
    fn serves_shell_and_artifact_files() {
        let (root, config) = temp_config("serve", ArtifactKind::Html);
        for uri in [
            "odl-shell://shell/index.html",
            "http://odl-shell.shell/index.html",
        ] {
            let resp = get(&config, uri);
            assert_eq!(resp.status(), StatusCode::OK, "{uri}");
            let body = String::from_utf8_lossy(resp.body());
            assert!(body.contains("__odlShellBridge"), "bridge placeholder");
            assert!(body.contains("Design agent"), "agent panel placeholder");
        }
        let resp = get(&config, "http://odl-shell.shell/artifact/assets/x.css?t=123");
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            "text/css; charset=utf-8"
        );
        let _ = std::fs::remove_dir_all(root);
    }

    /// 路径越权（`..` / 绝对路径 / 盘符）一律 404。
    #[test]
    fn path_traversal_is_rejected() {
        let (root, config) = temp_config("traversal", ArtifactKind::Html);
        // 根外目标文件真实存在，专门验证越权读不到。
        std::fs::write(root.parent().unwrap().join("odl-secret.txt"), "s").unwrap();
        for uri in [
            "odl-shell://shell/artifact/../odl-secret.txt",
            "odl-shell://shell/artifact/..%2Fodl-secret.txt",
            "odl-shell://shell/artifact//etc/passwd",
            "odl-shell://shell/artifact/C:/Windows/win.ini",
        ] {
            let resp = get(&config, uri);
            assert_eq!(resp.status(), StatusCode::NOT_FOUND, "{uri}");
        }
        let _ = std::fs::remove_file(root.parent().unwrap().join("odl-secret.txt"));
        let _ = std::fs::remove_dir_all(root);
    }

    /// Markdown 的入口映射到 `.odl/preview.html`。
    #[test]
    fn markdown_entry_maps_to_rendered_preview() {
        let (root, config) = temp_config("md", ArtifactKind::Markdown);
        std::fs::create_dir_all(root.join(".odl")).unwrap();
        std::fs::write(root.join(".odl").join("preview.html"), "<html>md</html>").unwrap();
        let resp = get(&config, "odl-shell://shell/artifact/doc.md");
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body().as_ref(), b"<html>md</html>");
        let _ = std::fs::remove_dir_all(root);
    }

    /// 视口尺寸契约（preview.md：窗口与视口）。
    #[test]
    fn viewport_sizes_per_kind() {
        assert_eq!(viewport_for(ArtifactKind::Slides), (1280, 720));
        assert_eq!(viewport_for(ArtifactKind::Html), (1366, 768));
        assert_eq!(viewport_for(ArtifactKind::Markdown), (1366, 768));
    }

    /// query/fragment 剥离与两种 scheme 的 path 提取。
    #[test]
    fn request_path_extraction() {
        assert_eq!(
            request_path("odl-shell://shell/artifact/a.css?t=1"),
            "/artifact/a.css"
        );
        assert_eq!(
            request_path("http://odl-shell.shell/index.html#x"),
            "/index.html"
        );
        assert_eq!(request_path("http://odl-shell.shell"), "/");
    }
}
