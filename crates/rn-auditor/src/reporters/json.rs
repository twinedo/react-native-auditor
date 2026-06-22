use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::issue::{Issue, Severity};
use crate::scanner::ProjectScan;

#[derive(Debug, Serialize)]
struct JsonReport {
    tool: &'static str,
    version: &'static str,
    root: String,
    summary: JsonSummary,
    detected: JsonDetected,
    issues: Vec<JsonIssue>,
}

#[derive(Debug, Serialize)]
struct JsonSummary {
    issues: JsonIssueCounts,
}

#[derive(Debug, Serialize)]
struct JsonIssueCounts {
    info: usize,
    warnings: usize,
    errors: usize,
}

#[derive(Debug, Serialize)]
struct JsonDetected {
    files: JsonDetectedFiles,
    lockfiles: Vec<String>,
}

#[derive(Debug, Serialize)]
struct JsonDetectedFiles {
    package_json: bool,
    app_json: bool,
    app_config_js: bool,
    app_config_ts: bool,
    eas_json: bool,
    env: bool,
    env_example: bool,
    babel_config_js: bool,
    metro_config_js: bool,
}

#[derive(Debug, Serialize)]
struct JsonIssue {
    code: String,
    severity: &'static str,
    title: String,
    message: String,
    file: Option<String>,
}

pub fn write_json_report(
    scan: &ProjectScan,
    issues: &[Issue],
    output_path: &Path,
) -> Result<(), String> {
    let json = render_json_report(scan, issues)?;

    fs::write(output_path, json).map_err(|error| {
        format!(
            "Failed to write JSON report to {}: {error}",
            output_path.display()
        )
    })
}

pub fn render_json_report(scan: &ProjectScan, issues: &[Issue]) -> Result<String, String> {
    let report = JsonReport {
        tool: "React Native Auditor",
        version: env!("CARGO_PKG_VERSION"),
        root: scan.root.display().to_string(),
        summary: JsonSummary {
            issues: issue_counts(issues),
        },
        detected: JsonDetected {
            files: JsonDetectedFiles {
                package_json: scan.has_package_json,
                app_json: scan.has_app_json,
                app_config_js: scan.has_app_config_js,
                app_config_ts: scan.has_app_config_ts,
                eas_json: scan.has_eas_json,
                env: scan.has_env,
                env_example: scan.has_env_example,
                babel_config_js: scan.has_babel_config_js,
                metro_config_js: scan.has_metro_config_js,
            },
            lockfiles: scan
                .lockfiles
                .iter()
                .map(|path| {
                    path.file_name()
                        .unwrap_or(path.as_os_str())
                        .to_string_lossy()
                        .into_owned()
                })
                .collect(),
        },
        issues: issues.iter().map(JsonIssue::from).collect(),
    };

    serde_json::to_string_pretty(&report)
        .map(|json| format!("{json}\n"))
        .map_err(|error| format!("Failed to serialize JSON report: {error}"))
}

fn issue_counts(issues: &[Issue]) -> JsonIssueCounts {
    let mut counts = JsonIssueCounts {
        info: 0,
        warnings: 0,
        errors: 0,
    };

    for issue in issues {
        match issue.severity {
            Severity::Info => counts.info += 1,
            Severity::Warning => counts.warnings += 1,
            Severity::Error => counts.errors += 1,
        }
    }

    counts
}

fn severity_label(severity: &Severity) -> &'static str {
    match severity {
        Severity::Info => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
    }
}

impl From<&Issue> for JsonIssue {
    fn from(issue: &Issue) -> Self {
        Self {
            code: issue.code.clone(),
            severity: severity_label(&issue.severity),
            title: issue.title.clone(),
            message: issue.message.clone(),
            file: issue
                .file_path
                .as_ref()
                .map(|path| path.display().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{issue_counts, severity_label};
    use crate::issue::{Issue, Severity};

    #[test]
    fn counts_issues_by_severity() {
        let issues = [
            issue(Severity::Info),
            issue(Severity::Warning),
            issue(Severity::Warning),
            issue(Severity::Error),
        ];

        let counts = issue_counts(&issues);

        assert_eq!(counts.info, 1);
        assert_eq!(counts.warnings, 2);
        assert_eq!(counts.errors, 1);
    }

    #[test]
    fn uses_lowercase_severity_labels() {
        assert_eq!(severity_label(&Severity::Info), "info");
        assert_eq!(severity_label(&Severity::Warning), "warning");
        assert_eq!(severity_label(&Severity::Error), "error");
    }

    fn issue(severity: Severity) -> Issue {
        Issue::new("TEST", "Test issue", severity, "Test message", None)
    }
}
