use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

fn odl() -> Command {
    Command::cargo_bin("odl").unwrap()
}

#[test]
fn help_contains_m1_commands() {
    odl()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("preview"))
        .stdout(predicate::str::contains("handoff"))
        .stdout(predicate::str::contains("export"))
        .stdout(predicate::str::contains("skill"));
}

#[test]
fn init_creates_workspace_manifest() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("workspace");

    odl()
        .args(["init", root.to_str().unwrap()])
        .assert()
        .success();

    assert!(root.join("manifest.json").exists());
    assert!(root.join("artifacts").is_dir());
    assert!(root.join("skills").is_dir());
}

#[test]
fn new_html_creates_artifact_files() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("artifact");

    odl()
        .args(["new", "html", root.to_str().unwrap(), "--title", "Demo"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "warning: skill `html-page` has no usable template",
        ));

    assert!(root.join("index.html").exists());
    assert!(root.join("manifest.json").exists());
    assert!(root.join("handoff.md").exists());
    assert!(root.join("assets").join("od-design.css").exists());

    let manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["kind"], "html");
    assert_eq!(manifest["title"], "Demo");
    assert_eq!(manifest["primaryFile"], "index.html");
}

#[test]
fn json_new_outputs_parseable_success_envelope() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("artifact");

    let output = odl()
        .args(["--json", "new", "docs", root.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: Value = serde_json::from_slice(&output).unwrap();

    assert_eq!(value["ok"], true);
    assert_eq!(value["artifact"]["kind"], "docs");
    assert_eq!(value["artifact"]["primaryFile"], "doc.md");
}

#[test]
fn parameter_error_returns_exit_code_2() {
    odl().arg("new").assert().code(2);
}

#[test]
fn skill_lists_builtin_skills_and_json_is_parseable() {
    odl()
        .arg("skill")
        .assert()
        .success()
        .stdout(predicate::str::contains("html-page | html"))
        .stdout(predicate::str::contains("docs-polish | docs"))
        .stdout(predicate::str::contains("slides-html | slides"));

    let output = odl()
        .args(["--json", "skill"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: Value = serde_json::from_slice(&output).unwrap();

    assert_eq!(value.as_array().unwrap().len(), 3);
}
