//! 集成测试：skill 模板不得依赖 CDN。
//!
//! Spec: docs/specs/built-in-skills.md（测试表：HTML / slides template 无默认 CDN 依赖）

use std::fs;
use std::path::PathBuf;

/// 仓库根的 `skills/` 目录（od-cli 位于 `<repo>/crates/od-cli`）。
fn skills_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills")
}

#[test]
fn html_templates_have_no_cdn_dependency() {
    let skills = skills_root();
    let mut checked = 0;
    for entry in fs::read_dir(&skills).unwrap().flatten() {
        let skill_dir = entry.path();
        let tmpl_dir = skill_dir.join("templates");
        let tmpl_dir = match tmpl_dir.canonicalize() {
            Ok(d) => d,
            Err(_) => continue,
        };
        for html in fs::read_dir(&tmpl_dir).unwrap().flatten() {
            let path = html.path();
            if path.extension().and_then(|e| e.to_str()) != Some("html") {
                continue;
            }
            let content = fs::read_to_string(&path).unwrap();
            assert!(
                !content.contains("<script src=\"https://"),
                "{}: must not load remote scripts",
                path.display()
            );
            assert!(
                !content.contains("<link href=\"https://"),
                "{}: must not load remote stylesheets",
                path.display()
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 2,
        "expected at least 2 HTML templates, checked {checked}"
    );
}
