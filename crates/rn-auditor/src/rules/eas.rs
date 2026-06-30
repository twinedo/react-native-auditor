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
    } else if eas_json_error.is_none()
        && eas_json.is_some_and(|eas_json| eas_json.production_profile_is_empty_object())
    {
        issues.push(Issue::new(
            "RNA_EAS_003",
            "Weak EAS production build profile",
            Severity::Info,
            "build.production is an empty object and may rely on defaults. Review it before release to confirm production builds are configured intentionally.",
            Some(root.join("eas.json")),
        ));
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_eas_json(json: &str) -> EasJson {
        serde_json::from_str(json).expect("eas.json fixture should parse")
    }

    #[test]
    fn empty_eas_production_profile_produces_weak_profile_issue() {
        let eas_json = parse_eas_json(r#"{"build":{"production":{}}}"#);

        let issue_codes =
            release_readiness_issues(Path::new("/project"), true, true, None, Some(&eas_json))
                .into_iter()
                .map(|issue| issue.code)
                .collect::<Vec<_>>();

        assert_eq!(issue_codes, vec!["RNA_EAS_003"]);
    }

    #[test]
    fn missing_build_production_still_produces_missing_profile_issue() {
        let eas_json = parse_eas_json(r#"{"build":{"preview":{}}}"#);

        let issue_codes =
            release_readiness_issues(Path::new("/project"), true, true, None, Some(&eas_json))
                .into_iter()
                .map(|issue| issue.code)
                .collect::<Vec<_>>();

        assert_eq!(issue_codes, vec!["RNA_EAS_002"]);
    }
}
