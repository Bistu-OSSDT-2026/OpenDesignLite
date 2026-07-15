//! Artifact export: html / md / zip / pdf.
//!
//! Shared by `odl export` and MCP `artifact_export`. Keeps packaging rules in
//! one place so CLI and MCP cannot drift.
//!
//! Spec: docs/specs/export.md

use crate::artifact::{detect_kind, Artifact, ArtifactKind, PRIMARY_FILE_ORDER};
use crate::design::{css_for, css_for_kind, VisualBrief, STYLESHEET_ASSET};
use crate::manifest::ArtifactManifest;
use crate::{OdError, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

/// Supported export formats (CLI `--format` / MCP `format`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Html,
    Md,
    Zip,
    Pdf,
}

impl ExportFormat {
    pub fn parse(value: &str) -> Result<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "html" => Ok(Self::Html),
            "md" | "markdown" => Ok(Self::Md),
            "zip" => Ok(Self::Zip),
            "pdf" => Ok(Self::Pdf),
            other => Err(OdError::FormatUnsupported(format!(
                "unknown export format `{other}`; use html, md, zip, or pdf"
            ))),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::Md => "md",
            Self::Zip => "zip",
            Self::Pdf => "pdf",
        }
    }
}

pub struct ExportOptions<'a> {
    /// Output path. When `None`, a format-specific default under the current
    /// directory is chosen (see [`default_out_path`]).
    pub out: Option<&'a Path>,
}

#[derive(Debug)]
pub struct ExportResult {
    pub out: PathBuf,
    pub format: ExportFormat,
}

/// Export an artifact directory to `format`.
pub fn run(root: &Path, format: ExportFormat, options: ExportOptions<'_>) -> Result<ExportResult> {
    let artifact = artifact_from_root(root)?;
    let out = resolve_out_path(&artifact, format, options.out)?;

    match format {
        ExportFormat::Zip => export_zip(&artifact, &out)?,
        ExportFormat::Html => export_html(&artifact, &out)?,
        ExportFormat::Md => export_md(&artifact, &out)?,
        ExportFormat::Pdf => export_pdf(&artifact, &out)?,
    }

    Ok(ExportResult { out, format })
}

pub fn artifact_from_root(root: &Path) -> Result<Artifact> {
    if !root.is_dir() {
        return Err(OdError::WorkspaceNotFound(root.to_path_buf()));
    }
    for primary in PRIMARY_FILE_ORDER {
        let path = root.join(primary);
        if path.exists() {
            let kind = detect_kind(primary).expect("primary file order maps to kind");
            return Ok(Artifact::new(kind, root));
        }
    }
    Err(OdError::PrimaryFileMissing(root.join("index.html")))
}

fn resolve_out_path(
    artifact: &Artifact,
    format: ExportFormat,
    out: Option<&Path>,
) -> Result<PathBuf> {
    match out {
        Some(path) => Ok(path.to_path_buf()),
        None => default_out_path(artifact, format),
    }
}

fn default_out_path(artifact: &Artifact, format: ExportFormat) -> Result<PathBuf> {
    let cwd = std::env::current_dir().map_err(OdError::Io)?;
    let name = artifact_name(artifact);
    Ok(match format {
        ExportFormat::Html => cwd.join("export"),
        ExportFormat::Md => cwd.join("doc.md"),
        ExportFormat::Zip => cwd.join(format!("{name}.zip")),
        ExportFormat::Pdf => cwd.join(format!("{name}.pdf")),
    })
}

fn artifact_name(artifact: &Artifact) -> String {
    artifact
        .root
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("artifact")
        .to_string()
}

// ───────────────────────────── ZIP ─────────────────────────────────────

const ZIP_EXCLUDE_DIRS: &[&str] = &[".git", ".log", ".odl"];

fn export_zip(artifact: &Artifact, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = File::create(out)
        .map_err(|e| OdError::ExportFailed(format!("create zip {}: {e}", out.display())))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut entries = Vec::new();
    collect_zip_entries(&artifact.root, &artifact.root, &mut entries)?;
    entries.sort();

    if entries.is_empty() {
        return Err(OdError::ResourceMissing(
            "artifact has no files to package".into(),
        ));
    }

    for rel in entries {
        let abs = artifact.root.join(&rel);
        let name = zip_path(&rel);
        if abs.is_dir() {
            let dir_name = format!("{}/", name.trim_end_matches('/'));
            zip.add_directory(&dir_name, options)
                .map_err(|e| OdError::ExportFailed(format!("zip directory `{dir_name}`: {e}")))?;
        } else {
            let data = fs::read(&abs)
                .map_err(|e| OdError::ExportFailed(format!("read {}: {e}", abs.display())))?;
            zip.start_file(&name, options)
                .map_err(|e| OdError::ExportFailed(format!("zip start `{name}`: {e}")))?;
            zip.write_all(&data)
                .map_err(|e| OdError::ExportFailed(format!("zip write `{name}`: {e}")))?;
        }
    }

    zip.finish()
        .map_err(|e| OdError::ExportFailed(format!("finalize zip: {e}")))?;
    Ok(())
}

fn collect_zip_entries(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    let read = fs::read_dir(dir)
        .map_err(|e| OdError::ExportFailed(format!("read dir {}: {e}", dir.display())))?;
    for entry in read {
        let entry = entry.map_err(|e| OdError::ExportFailed(e.to_string()))?;
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if ZIP_EXCLUDE_DIRS.iter().any(|d| *d == name_str) {
            continue;
        }
        // Skip common OS junk / temp markers.
        if name_str == ".DS_Store" || name_str == "Thumbs.db" || name_str.starts_with("~$") {
            continue;
        }

        let rel = path
            .strip_prefix(root)
            .map_err(|_| OdError::PathEscape(path.clone()))?;
        let rel_str = path_to_zip_rel(rel);

        if path.is_dir() {
            out.push(rel_str);
            collect_zip_entries(root, &path, out)?;
        } else if path.is_file() {
            out.push(rel_str);
        }
    }
    Ok(())
}

fn path_to_zip_rel(rel: &Path) -> String {
    rel.components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn zip_path(rel: &str) -> String {
    rel.replace('\\', "/")
}

// ───────────────────────────── HTML ────────────────────────────────────

fn export_html(artifact: &Artifact, out: &Path) -> Result<()> {
    fs::create_dir_all(out)?;
    ensure_clean_export_dir(out)?;

    match artifact.kind {
        ArtifactKind::Html | ArtifactKind::Slides => {
            let primary = artifact.kind.primary_file();
            let src = artifact.primary_path();
            if !src.exists() {
                return Err(OdError::ResourceMissing(format!(
                    "primary file missing: {}",
                    src.display()
                )));
            }
            fs::copy(&src, out.join(primary))
                .map_err(|e| OdError::ExportFailed(format!("copy {primary}: {e}")))?;
            copy_assets_dir(artifact, out)?;
        }
        ArtifactKind::Markdown => {
            let html = render_doc_html(artifact)?;
            fs::write(out.join("doc.html"), html)
                .map_err(|e| OdError::ExportFailed(format!("write doc.html: {e}")))?;
            copy_assets_dir(artifact, out)?;
            ensure_design_css(artifact, out)?;
        }
    }
    Ok(())
}

fn ensure_clean_export_dir(out: &Path) -> Result<()> {
    // Allow writing into an empty or new directory; refuse non-empty dirs that
    // look unrelated so we do not clobber user files silently.
    let mut entries = fs::read_dir(out).map_err(OdError::Io)?;
    if entries.next().is_none() {
        return Ok(());
    }
    // Non-empty: only proceed if it already looks like a previous export
    // (has index.html / slides.html / doc.html or assets/).
    let markers = ["index.html", "slides.html", "doc.html", "assets"];
    let has_marker = markers.iter().any(|m| out.join(m).exists());
    if !has_marker {
        return Err(OdError::ExportFailed(format!(
            "export directory {} is not empty; choose an empty --out path",
            out.display()
        )));
    }
    Ok(())
}

fn copy_assets_dir(artifact: &Artifact, out: &Path) -> Result<()> {
    let src = artifact.assets_dir();
    if !src.is_dir() {
        return Ok(());
    }
    let dest = out.join("assets");
    copy_dir_recursive(&src, &dest)?;
    Ok(())
}

fn ensure_design_css(artifact: &Artifact, out: &Path) -> Result<()> {
    let dest = out.join(STYLESHEET_ASSET);
    if dest.exists() {
        return Ok(());
    }
    let brief = visual_brief_for(artifact);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&dest, css_for(brief))?;
    Ok(())
}

fn render_doc_html(artifact: &Artifact) -> Result<String> {
    let src = artifact.primary_path();
    if !src.exists() {
        return Err(OdError::ResourceMissing(format!(
            "primary file missing: {}",
            src.display()
        )));
    }
    let md = fs::read_to_string(&src)
        .map_err(|e| OdError::ExportFailed(format!("read {}: {e}", src.display())))?;
    let fragment = comrak::markdown_to_html(&md, &comrak::Options::default());
    let title = artifact_name(artifact);
    Ok(format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{title}</title>\n\
         <link rel=\"stylesheet\" href=\"{stylesheet}\">\n\
         </head>\n\
         <body>\n\
         <div class=\"od-container od-doc\">\n{fragment}\n</div>\n\
         </body>\n\
         </html>\n",
        stylesheet = STYLESHEET_ASSET,
    ))
}

fn visual_brief_for(artifact: &Artifact) -> VisualBrief {
    if let Ok(raw) = fs::read_to_string(artifact.manifest_path()) {
        if let Ok(manifest) = serde_json::from_str::<ArtifactManifest>(&raw) {
            if let Some(design) = manifest.design {
                if let Some(brief) = design.visual_brief.as_deref().and_then(VisualBrief::parse) {
                    return brief;
                }
            }
        }
    }
    VisualBrief::default_for(artifact.kind)
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src).map_err(OdError::Io)? {
        let entry = entry.map_err(OdError::Io)?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else if from.is_file() {
            fs::copy(&from, &to).map_err(OdError::Io)?;
        }
    }
    Ok(())
}

// ───────────────────────────── MD ──────────────────────────────────────

fn export_md(artifact: &Artifact, out: &Path) -> Result<()> {
    match artifact.kind {
        ArtifactKind::Markdown => {
            let src = artifact.primary_path();
            if !src.exists() {
                return Err(OdError::ResourceMissing(format!(
                    "primary file missing: {}",
                    src.display()
                )));
            }
            if let Some(parent) = out.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            // If --out is a directory, write doc.md inside it.
            let dest = if out.is_dir()
                || out
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("md"))
                    != Some(true)
            {
                fs::create_dir_all(out)?;
                out.join("doc.md")
            } else {
                out.to_path_buf()
            };
            fs::copy(&src, &dest)
                .map_err(|e| OdError::ExportFailed(format!("copy doc.md: {e}")))?;
            Ok(())
        }
        ArtifactKind::Html | ArtifactKind::Slides => Err(OdError::FormatUnsupported(format!(
            "format `md` is not supported for `{}` artifacts; use html, zip, or pdf",
            artifact.kind.slug()
        ))),
    }
}

// ───────────────────────────── PDF ─────────────────────────────────────

fn export_pdf(artifact: &Artifact, out: &Path) -> Result<()> {
    let browser = find_pdf_browser().ok_or_else(|| {
        OdError::PdfBackendMissing(
            "no Chrome/Edge found for PDF export; install Google Chrome or Microsoft Edge, or export html/zip instead".into(),
        )
    })?;

    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let staging = tempfile_dir("odl-export-pdf")?;
    let html_path = prepare_pdf_source(artifact, &staging)?;
    let html_url = path_to_file_url(&html_path)?;
    let out_abs = canonicalize_for_print(out)?;

    let status = Command::new(&browser)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--no-pdf-header-footer")
        .arg(format!("--print-to-pdf={}", out_abs.display()))
        .arg(&html_url)
        .status()
        .map_err(|e| OdError::ExportFailed(format!("launch PDF backend: {e}")))?;

    let _ = fs::remove_dir_all(&staging);

    if !status.success() {
        return Err(OdError::ExportFailed(format!(
            "PDF backend exited with {status}"
        )));
    }
    if !out_abs.exists() {
        return Err(OdError::ExportFailed(
            "PDF backend finished but output file was not created".into(),
        ));
    }
    Ok(())
}

fn prepare_pdf_source(artifact: &Artifact, staging: &Path) -> Result<PathBuf> {
    let html = match artifact.kind {
        ArtifactKind::Html | ArtifactKind::Slides => {
            // Copy into staging so relative assets resolve under file://.
            export_html(artifact, staging)?;
            staging.join(artifact.kind.primary_file())
        }
        ArtifactKind::Markdown => {
            export_html(artifact, staging)?;
            staging.join("doc.html")
        }
    };
    // PDF 始终按当前内核样式渲染：staging 副本里重写 od-design.css，
    // 旧产物（创建时无 16:9 打印规则）导出也能拿到；产物原文件一概不动
    //（export.md：staging 内样式升级）。
    refresh_design_css(artifact, staging)?;
    Ok(html)
}

/// 与 `ensure_design_css`（缺才写）不同：**总是**用当前内核版本重写。
/// 仅用于 PDF staging 副本，不得作用于产物目录本身。
fn refresh_design_css(artifact: &Artifact, out: &Path) -> Result<()> {
    let dest = out.join(STYLESHEET_ASSET);
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let brief = visual_brief_for(artifact);
    fs::write(&dest, css_for_kind(brief, artifact.kind))?;
    Ok(())
}

fn tempfile_dir(prefix: &str) -> Result<PathBuf> {
    let base = std::env::temp_dir().join(format!(
        "{prefix}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&base)?;
    Ok(base)
}

fn canonicalize_for_print(out: &Path) -> Result<PathBuf> {
    if out.exists() {
        return out.canonicalize().map_err(OdError::Io);
    }
    // Create empty file so canonicalize works on Windows.
    if let Some(parent) = out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    File::create(out)?;
    let abs = out.canonicalize().map_err(OdError::Io)?;
    Ok(abs)
}

fn path_to_file_url(path: &Path) -> Result<String> {
    let abs = path.canonicalize().map_err(OdError::Io)?;
    let mut path_str = abs.to_string_lossy().replace('\\', "/");
    // Windows canonicalize 会加 \\?\ verbatim 前缀（替换后是 //?/C:/...）。
    // 带着它拼出的 file:////?/C:/... 里 base URL 是坏的：页面能加载，但
    // 相对路径的 assets/od-design.css 解析不出来 → PDF 无样式、@page 失效。
    if let Some(stripped) = path_str.strip_prefix("//?/") {
        path_str = stripped.to_string();
    }
    // Windows: C:/... → file:///C:/...
    if path_str.len() >= 2 && path_str.as_bytes().get(1) == Some(&b':') {
        return Ok(format!("file:///{path_str}"));
    }
    if !path_str.starts_with('/') {
        path_str = format!("/{path_str}");
    }
    Ok(format!("file://{path_str}"))
}

fn find_pdf_browser() -> Option<PathBuf> {
    // Explicit override for CI / custom installs.
    if let Ok(custom) = std::env::var("ODL_PDF_BROWSER") {
        let p = PathBuf::from(custom);
        if p.is_file() {
            return Some(p);
        }
    }

    #[cfg(windows)]
    {
        let local = std::env::var_os("LOCALAPPDATA");
        let program = std::env::var_os("PROGRAMFILES");
        let program_x86 = std::env::var_os("PROGRAMFILES(X86)");
        let candidates = [
            local
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Google\Chrome\Application\chrome.exe")),
            program
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Google\Chrome\Application\chrome.exe")),
            program_x86
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Google\Chrome\Application\chrome.exe")),
            local
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Microsoft\Edge\Application\msedge.exe")),
            program
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Microsoft\Edge\Application\msedge.exe")),
            program_x86
                .as_ref()
                .map(|p| PathBuf::from(p).join(r"Microsoft\Edge\Application\msedge.exe")),
        ];
        for c in candidates.into_iter().flatten() {
            if c.is_file() {
                return Some(c);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = [
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
        ];
        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return Some(p);
            }
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        for name in [
            "google-chrome",
            "google-chrome-stable",
            "chromium",
            "chromium-browser",
            "microsoft-edge",
            "microsoft-edge-stable",
        ] {
            if let Some(p) = which_bin(name) {
                return Some(p);
            }
        }
    }

    None
}

#[cfg(all(unix, not(target_os = "macos")))]
fn which_bin(name: &str) -> Option<PathBuf> {
    let output = Command::new("which").arg(name).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return None;
    }
    let p = PathBuf::from(path);
    p.is_file().then_some(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create::{self, CreateOptions};
    use std::io::Read;
    use zip::ZipArchive;

    fn temp_artifact(kind: &str, name: &str) -> (tempfile::TempDir, PathBuf) {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join(name);
        create::run(
            kind,
            &root,
            CreateOptions {
                title: Some(name),
                visual_brief: "editorial",
                embed_css: false,
                overwrite: false,
            },
        )
        .unwrap();
        // Keep temp alive by returning it.
        (temp, root)
    }

    // tempfile is a dev-dep of od-cli; for unit tests in od-core use std temp.
    mod tempfile {
        use std::path::{Path, PathBuf};
        pub struct TempDir(PathBuf);
        impl TempDir {
            pub fn path(&self) -> &Path {
                &self.0
            }
        }
        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        pub fn tempdir() -> std::io::Result<TempDir> {
            let path = std::env::temp_dir().join(format!(
                "od-core-export-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            ));
            std::fs::create_dir_all(&path)?;
            Ok(TempDir(path))
        }
    }

    #[test]
    fn parses_formats() {
        assert_eq!(ExportFormat::parse("HTML").unwrap(), ExportFormat::Html);
        assert_eq!(ExportFormat::parse("md").unwrap(), ExportFormat::Md);
        assert!(ExportFormat::parse("pptx").is_err());
    }

    #[test]
    fn zip_export_uses_forward_slashes() {
        let (_tmp, root) = temp_artifact("html", "zip-demo");
        // Ensure nested asset path exists.
        assert!(root.join("assets").join("od-design.css").exists());
        // Add a file that must be excluded.
        fs::create_dir_all(root.join(".odl")).unwrap();
        fs::write(root.join(".odl").join("preview.html"), "x").unwrap();
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::write(root.join(".git").join("config"), "x").unwrap();

        let out = root.parent().unwrap().join("zip-demo.zip");
        let result = run(&root, ExportFormat::Zip, ExportOptions { out: Some(&out) }).unwrap();
        assert_eq!(result.out, out);

        let file = File::open(&out).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        let mut names = Vec::new();
        for i in 0..archive.len() {
            let entry = archive.by_index(i).unwrap();
            let name = entry.name().to_string();
            assert!(!name.contains('\\'), "zip path must use /: {name}");
            names.push(name);
        }
        assert!(names
            .iter()
            .any(|n| n == "index.html" || n == "manifest.json"));
        assert!(names.iter().any(|n| n == "assets/od-design.css"));
        assert!(names.iter().any(|n| n == "handoff.md"));
        assert!(names.iter().all(|n| !n.starts_with(".odl")));
        assert!(names.iter().all(|n| !n.starts_with(".git")));
    }

    #[test]
    fn html_export_is_offline_openable() {
        let (_tmp, root) = temp_artifact("html", "html-demo");
        let out = root.parent().unwrap().join("html-out");
        run(&root, ExportFormat::Html, ExportOptions { out: Some(&out) }).unwrap();

        assert!(out.join("index.html").exists());
        assert!(out.join("assets").join("od-design.css").exists());
        let html = fs::read_to_string(out.join("index.html")).unwrap();
        assert!(
            html.contains("assets/od-design.css") || html.contains("od-"),
            "exported html should reference local design assets"
        );
    }

    #[test]
    fn docs_html_export_uses_design_css() {
        let (_tmp, root) = temp_artifact("docs", "docs-demo");
        let out = root.parent().unwrap().join("docs-html-out");
        run(&root, ExportFormat::Html, ExportOptions { out: Some(&out) }).unwrap();
        assert!(out.join("doc.html").exists());
        assert!(out.join("assets").join("od-design.css").exists());
        let html = fs::read_to_string(out.join("doc.html")).unwrap();
        assert!(html.contains("assets/od-design.css"));
        assert!(html.contains("od-container"));
    }

    #[test]
    fn md_export_for_docs() {
        let (_tmp, root) = temp_artifact("docs", "md-demo");
        let out = root.parent().unwrap().join("exported.md");
        run(&root, ExportFormat::Md, ExportOptions { out: Some(&out) }).unwrap();
        assert!(out.exists());
        let body = fs::read_to_string(&out).unwrap();
        assert!(!body.is_empty());
    }

    #[test]
    fn md_unsupported_for_html() {
        let (_tmp, root) = temp_artifact("html", "md-bad");
        let out = root.parent().unwrap().join("nope.md");
        let err = run(&root, ExportFormat::Md, ExportOptions { out: Some(&out) }).unwrap_err();
        assert_eq!(err.code(), "format_unsupported");
    }

    #[test]
    fn pdf_missing_backend_error_code() {
        let err = OdError::PdfBackendMissing(
            "no Chrome/Edge found for PDF export; install Google Chrome or Microsoft Edge, or export html/zip instead".into(),
        );
        assert_eq!(err.code(), "pdf_backend_missing");
        let msg = err.to_string();
        assert!(msg.contains("Chrome") || msg.contains("Edge"));
    }

    /// PDF staging 里的 od-design.css 总是被当前内核重写（含 16:9 @page 规则），
    /// 即使产物自带的是旧版/手改过的样式表；产物原文件不动（export.md）。
    #[test]
    fn pdf_staging_refreshes_design_css_without_touching_artifact() {
        let (_temp, root) = temp_artifact("slides", "deck");
        let artifact = artifact_from_root(&root).unwrap();

        // 模拟旧产物：CSS 是不含打印规则的旧版本。
        let artifact_css = root.join(STYLESHEET_ASSET);
        fs::write(&artifact_css, "/* legacy css, no print rules */").unwrap();

        let staging = tempfile_dir("odl-pdf-staging-test").unwrap();
        let html = prepare_pdf_source(&artifact, &staging).unwrap();
        assert!(html.exists());

        let staged_css = fs::read_to_string(staging.join(STYLESHEET_ASSET)).unwrap();
        assert!(
            staged_css.contains("@page { size: 13.333in 7.5in; margin: 0; }"),
            "staging css must carry current 16:9 print rules"
        );
        // 产物目录里的原文件保持用户内容不变。
        let original = fs::read_to_string(&artifact_css).unwrap();
        assert_eq!(original, "/* legacy css, no print rules */");

        let _ = fs::remove_dir_all(&staging);
    }

    /// Windows canonicalize 的 `\\?\` 前缀必须剥掉：带前缀的 file:// URL
    /// 会让浏览器解析不出相对 assets（PDF 无样式、@page 失效的历史根因）。
    #[test]
    fn file_url_strips_verbatim_prefix() {
        let (_temp, root) = temp_artifact("slides", "urlcheck");
        let url = path_to_file_url(&root.join("slides.html")).unwrap();
        assert!(!url.contains("//?/"), "verbatim prefix leaked: {url}");
        assert!(url.starts_with("file://"), "unexpected url: {url}");
    }

    /// html/docs 的 PDF staging 不携带 @page（纸张维持浏览器默认）。
    #[test]
    fn pdf_staging_html_has_no_page_rule() {
        let (_temp, root) = temp_artifact("html", "page");
        let artifact = artifact_from_root(&root).unwrap();
        let staging = tempfile_dir("odl-pdf-staging-html-test").unwrap();
        prepare_pdf_source(&artifact, &staging).unwrap();
        let staged_css = fs::read_to_string(staging.join(STYLESHEET_ASSET)).unwrap();
        assert!(!staged_css.contains("@page"));
        let _ = fs::remove_dir_all(&staging);
    }

    #[test]
    fn zip_roundtrip_reads_manifest() {
        let (_tmp, root) = temp_artifact("slides", "slides-zip");
        let out = root.parent().unwrap().join("slides.zip");
        run(&root, ExportFormat::Zip, ExportOptions { out: Some(&out) }).unwrap();
        let file = File::open(&out).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        let mut entry = archive.by_name("manifest.json").unwrap();
        let mut buf = String::new();
        entry.read_to_string(&mut buf).unwrap();
        assert!(buf.contains("slides"));
    }
}
