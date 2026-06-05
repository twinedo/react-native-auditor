use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

fn rn_auditor() -> Command {
    Command::cargo_bin("rn-auditor").expect("rn-auditor binary should be available")
}

fn temp_project() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("temp project directory should be created");
    write_minimal_package_json(temp_dir.path());
    temp_dir
}

fn write_minimal_package_json(project_path: &Path) {
    fs::write(
        project_path.join("package.json"),
        r#"{"dependencies":{"react-native":"0.76.0"}}"#,
    )
    .expect("minimal package.json should be written");
}

#[test]
fn audit_succeeds_for_valid_temp_project_path() {
    let project = temp_project();

    rn_auditor()
        .arg("audit")
        .arg(project.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("React Native Auditor"))
        .stdout(predicate::str::contains("Project summary"));
}

#[test]
fn scan_succeeds_for_valid_temp_project_path() {
    let project = temp_project();

    rn_auditor()
        .arg("scan")
        .arg(project.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("React Native Auditor"));
}

#[test]
fn audit_fails_for_invalid_path() {
    let project = tempfile::tempdir().expect("temp directory should be created");
    let missing_path = project.path().join("does-not-exist");

    rn_auditor()
        .arg("audit")
        .arg(missing_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Path does not exist"));
}

#[test]
fn audit_fails_for_file_path() {
    let temp_file = NamedTempFile::new().expect("temp file should be created");

    rn_auditor()
        .arg("audit")
        .arg(temp_file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Path is not a directory"));
}

#[test]
fn audit_detects_yarn_from_lockfile() {
    let project = temp_project();
    fs::write(project.path().join("yarn.lock"), "").expect("yarn.lock should be written");

    rn_auditor()
        .arg("audit")
        .arg(project.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Package manager: Yarn"));
}

#[test]
fn audit_warns_for_multiple_lockfiles() {
    let project = temp_project();
    fs::write(project.path().join("yarn.lock"), "").expect("yarn.lock should be written");
    fs::write(project.path().join("package-lock.json"), "{}")
        .expect("package-lock.json should be written");

    rn_auditor()
        .arg("audit")
        .arg(project.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Package manager: Multiple / Ambiguous",
        ))
        .stdout(predicate::str::contains("RNA_LOCKFILE_001"));
}

#[test]
fn audit_warns_for_env_without_env_example_without_printing_secret() {
    let project = temp_project();
    fs::write(project.path().join(".env"), "API_KEY=test").expect(".env should be written");

    rn_auditor()
        .arg("audit")
        .arg(project.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("RNA_ENV_001"))
        .stdout(predicate::str::contains("API_KEY=test").not());
}

#[test]
fn report_creates_default_html_file_without_exposing_env_secret() {
    let project = temp_project();
    let secret = "API_KEY=do-not-expose";
    fs::write(project.path().join(".env"), secret).expect(".env should be written");

    rn_auditor()
        .current_dir(project.path())
        .arg("report")
        .arg("--html")
        .assert()
        .success()
        .stdout(predicate::str::contains("HTML report written to:"));

    let html = fs::read_to_string(project.path().join("rn-auditor-report.html"))
        .expect("HTML report should be readable");

    assert!(html.contains("React Native Auditor Report"));
    assert!(html.contains("Project type"));
    assert!(html.contains("Package manager"));
    assert!(html.contains("RNA_ENV_001"));
    assert!(!html.contains(secret));
}

#[test]
fn report_writes_to_custom_output_path() {
    let project = temp_project();
    let output_dir = tempfile::tempdir().expect("output directory should be created");
    let output_path = output_dir.path().join("custom-report.html");

    rn_auditor()
        .arg("report")
        .arg("--html")
        .arg(project.path())
        .arg("--output")
        .arg(&output_path)
        .assert()
        .success();

    let html = fs::read_to_string(output_path).expect("custom HTML report should be readable");
    assert!(html.contains("React Native Auditor Report"));
}

#[test]
fn report_fails_for_invalid_project_path() {
    let project = tempfile::tempdir().expect("temp directory should be created");
    let missing_path = project.path().join("does-not-exist");

    rn_auditor()
        .arg("report")
        .arg("--html")
        .arg(missing_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Path does not exist"));
}

#[test]
fn report_requires_html_flag() {
    rn_auditor()
        .arg("report")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--html"));
}
