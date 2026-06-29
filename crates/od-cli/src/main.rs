use od_core::{workspace_manifest_path, Artifact, ArtifactKind};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return;
    };

    let result = match command.as_str() {
        "init" => {
            let root = args.next().unwrap_or_else(|| ".odl".to_string());
            init_workspace(PathBuf::from(root))
        }
        "new" => {
            let kind = args.next().unwrap_or_else(|| "html".to_string());
            let root = args.next().unwrap_or_else(|| "artifact".to_string());
            new_artifact(&kind, PathBuf::from(root))
        }
        "preview" => {
            let root = args.next().unwrap_or_else(|| ".".to_string());
            preview_artifact(PathBuf::from(root))
        }
        _ => {
            print_help();
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn init_workspace(root: PathBuf) -> Result<(), String> {
    fs::create_dir_all(root.join("artifacts")).map_err(|e| e.to_string())?;
    fs::create_dir_all(root.join("skills")).map_err(|e| e.to_string())?;
    let manifest = workspace_manifest_path(&root);
    if !manifest.exists() {
        fs::write(
            &manifest,
            "{\n  \"schemaVersion\": 1,\n  \"name\": \"Open Design Lite Workspace\"\n}\n",
        )
        .map_err(|e| e.to_string())?;
    }
    println!("initialized {}", root.display());
    Ok(())
}

fn new_artifact(kind_slug: &str, root: PathBuf) -> Result<(), String> {
    let kind = ArtifactKind::from_slug(kind_slug).ok_or_else(|| {
        format!("unknown artifact kind `{kind_slug}`; use html, docs, or slides")
    })?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let artifact = Artifact::new(kind, root);
    let primary = artifact.primary_path();
    if primary.exists() {
        return Err(format!("{} already exists", primary.display()));
    }
    fs::write(&primary, starter_content(kind)).map_err(|e| e.to_string())?;
    fs::write(
        artifact.root.join("handoff.md"),
        "# Handoff\n\nDescribe intent, constraints, and next agent steps here.\n",
    )
    .map_err(|e| e.to_string())?;
    println!("created {}", primary.display());
    Ok(())
}

fn preview_artifact(root: PathBuf) -> Result<(), String> {
    let candidates = ["index.html", "slides.html", "doc.md"];
    for candidate in candidates {
        let path = root.join(candidate);
        if path.exists() {
            println!("preview target: {}", path.display());
            println!("native shell preview is not implemented yet");
            return Ok(());
        }
    }
    Err(format!("no previewable artifact found in {}", root.display()))
}

fn starter_content(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Html => include_str!("../../../templates/html-page/basic.html"),
        ArtifactKind::Markdown => "# Draft\n\nStart writing here.\n",
        ArtifactKind::Slides => include_str!("../../../templates/slides/basic.html"),
    }
}

fn print_help() {
    println!(
        "odl\n\ncommands:\n  init [dir]\n  new <html|docs|slides> <dir>\n  preview <dir>"
    );
}
