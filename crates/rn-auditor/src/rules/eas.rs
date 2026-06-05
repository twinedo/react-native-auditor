use std::path::Path;

use crate::issue::{Issue, Severity};
use crate::parsers::eas_json::EasJson;

pub fn release_readiness_issues(
    root: &Path,
    is_expo_project: bool,
    has_eas_json: bool,
    eas_json_error: Option<&String>,
    eas_json: Option<&EasJson>,
) -> Vec<Issue> {
    let mut issues = Vec::new();

    if !is_expo_project {
        return issues;
    }

    if !has_eas_json {
        issues.push(Issue::new(
            "RNA_EAS_001",
            "Missing eas.json",
            Severity::Warning,
            "Expo release builds usually need eas.json to configure EAS build and submit behavior for releases.",
            Some(root.join("eas.json")),
        ));
    } else if eas_json_error.is_none()
        && eas_json.is_some_and(|eas_json| eas_json.build_production().is_none())
    {
        issues.push(Issue::new(
            "RNA_EAS_002",
            "Missing EAS production build profile",
            Severity::Warning,
            "build.production is important for release readiness and keeping CI and release builds consistent.",
            Some(root.join("eas.json")),
        ));
    }

    issues
}
