//! 黑盒集成测试：`odl new` 生成完整 artifact 四件套 + `odl skill` 列表。
//!
//! Spec: docs/specs/cli.md, built-in-skills.md（测试表）

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 编译产物 `odl` 二进制路径（cargo test 自动设置 `CARGO_BIN_EXE_odl`）。
fn odl_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_odl"))
}

fn workspace_dir() -> PathBuf {
    let dir = std::env::temp_dir().join("od-cli-integration-new");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn odl_new_html_creates_four_files() {
    let ws = workspace_dir();
    let odl = odl_bin();

    // init workspace
    let status = Command::new(&odl).arg("init").arg(&ws).status().unwrap();
    assert!(status.success(), "odl init failed");

    // new html artifact
    let artifact_dir = ws.join("artifacts").join("site");
    let status = Command::new(&odl)
        .arg("new")
        .arg("html")
        .arg(&artifact_dir)
        .arg("--title")
        .arg("Integration")
        .status()
        .unwrap();
    assert!(status.success(), "odl new html failed");

    // 四件套
    assert!(
        artifact_dir.join("index.html").exists(),
        "primary file missing"
    );
    assert!(
        artifact_dir.join("manifest.json").exists(),
        "manifest missing"
    );
    assert!(
        artifact_dir.join("assets/od-design.css").exists(),
        "css missing"
    );
    assert!(artifact_dir.join("handoff.md").exists(), "handoff missing");
    assert!(!fs::read_to_string(artifact_dir.join("index.html"))
        .unwrap()
        .is_empty());

    fs::remove_dir_all(&ws).ok();
}

#[test]
fn odl_skill_list_outputs_three_lines_and_json_is_valid() {
    let odl = odl_bin();

    // human 列表
    let output = Command::new(&odl).arg("skill").output().unwrap();
    assert!(output.status.success(), "odl skill failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert!(lines.len() >= 3, "expected >=3 skills, got: {stdout}");

    // JSON 数组可被 serde_json 解析
    let output = Command::new(&odl)
        .arg("skill")
        .arg("--json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let json = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("invalid JSON: {e}: {json}"));
    assert!(parsed.is_array(), "expected array, got: {json}");
    assert!(parsed.as_array().unwrap().len() >= 3);
}

#[test]
fn odl_skill_show_outputs_body() {
    let odl = odl_bin();

    // human 正文：包含 # 标题与 Visual brief 小节
    let output = Command::new(&odl)
        .args(["skill", "show", "html-page"])
        .output()
        .unwrap();
    assert!(output.status.success(), "odl skill show html-page failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("# html-page"),
        "body should start with heading, got: {stdout}"
    );
    assert!(
        stdout.contains("Visual brief"),
        "body should include the visual brief section, got: {stdout}"
    );

    // JSON 正文：含 name/mode/description/body，body 非空
    let output = Command::new(&odl)
        .args(["skill", "show", "slides-html", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let json = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("invalid JSON: {e}: {json}"));
    let obj = parsed
        .as_object()
        .unwrap_or_else(|| panic!("expected object, got: {json}"));
    assert_eq!(obj["name"], "slides-html");
    assert_eq!(obj["mode"], "slides");
    assert!(obj["description"].as_str().unwrap().contains("slides"));
    assert!(
        !obj["body"].as_str().unwrap().is_empty(),
        "body must be non-empty, got: {json}"
    );
    assert!(obj["root"].as_str().unwrap().contains("slides-html"));
}

#[test]
fn odl_skill_show_unknown_is_usage_error() {
    let odl = odl_bin();

    let output = Command::new(&odl)
        .args(["skill", "show", "nope"])
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "odl skill show nope should fail, got: {output:?}"
    );
    assert_eq!(output.status.code(), Some(2), "expected usage exit code 2");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("skill_not_found"),
        "stderr should mention skill_not_found, got: {stderr}"
    );
    assert!(
        stderr.contains("nope"),
        "stderr should mention the missing name, got: {stderr}"
    );
}

#[test]
fn odl_new_unknown_kind_is_usage_error() {
    let ws = std::env::temp_dir().join("od-cli-integration-unknown");
    let _ = fs::remove_dir_all(&ws);
    fs::create_dir_all(&ws).unwrap();
    let odl = odl_bin();

    let status = Command::new(&odl)
        .arg("new")
        .arg("pdf")
        .arg(&ws)
        .status()
        .unwrap();
    // 参数错误由 clap 处理为退出码 2；未知 kind 是 OdError::ArtifactKindUnknown -> 3。
    // 两种都非 0。
    assert!(!status.success(), "odl new pdf should fail");

    fs::remove_dir_all(&ws).ok();
}
