use std::path::Path;

use crate::issue::{Issue, Severity};
use crate::parsers::app_json::AppJson;

pub fn release_metadata_issues(root: &Path, app_json: &AppJson) -> Vec<Issue> {
    let mut issues = Vec::new();

    if is_missing_string(app_json.name()) {
        issues.push(Issue::new(
            "RNA_EXPO_META_001",
            "Missing Expo app name",
            Severity::Warning,
            "expo.name is important for release metadata and the app identity shown in stores and on devices.",
            Some(root.join("app.json")),
        ));
    }

    if is_missing_string(app_json.slug()) {
        issues.push(Issue::new(
            "RNA_EXPO_META_002",
            "Missing Expo slug",
            Severity::Warning,
            "expo.slug is important for Expo project identity and release workflows.",
            Some(root.join("app.json")),
        ));
    }

    if is_missing_string(app_json.version()) {
        issues.push(Issue::new(
            "RNA_EXPO_META_003",
            "Missing Expo app version",
            Severity::Warning,
            "expo.version is important for release tracking across builds and submissions.",
            Some(root.join("app.json")),
        ));
    }

    if is_missing_string(app_json.ios_build_number()) {
        issues.push(Issue::new(
            "RNA_EXPO_IOS_002",
            "Missing iOS build number",
            Severity::Warning,
            "expo.ios.buildNumber is important for App Store release submissions and build tracking.",
            Some(root.join("app.json")),
        ));
    }

    if app_json.android_version_code().is_none() {
        issues.push(Issue::new(
            "RNA_EXPO_ANDROID_002",
            "Missing Android version code",
            Severity::Warning,
            "expo.android.versionCode is important for Play Store release submissions and build tracking.",
            Some(root.join("app.json")),
        ));
    }

    issues
}

fn is_missing_string(value: Option<&str>) -> bool {
    value.is_none_or(|value| value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_app_json(json: &str) -> AppJson {
        serde_json::from_str(json).expect("app.json fixture should parse")
    }

    #[test]
    fn complete_expo_app_metadata_produces_no_release_metadata_issues() {
        let app_json = parse_app_json(
            r#"{
                "expo": {
                    "name": "Example App",
                    "slug": "example-app",
                    "version": "1.0.0",
                    "ios": {
                        "bundleIdentifier": "com.example.app",
                        "buildNumber": "1"
                    },
                    "android": {
                        "package": "com.example.app",
                        "versionCode": 1
                    }
                }
            }"#,
        );

        let issues = release_metadata_issues(Path::new("/project"), &app_json);

        assert!(issues.is_empty());
    }

    #[test]
    fn missing_expo_app_metadata_produces_expected_issue_codes() {
        let app_json = parse_app_json(
            r#"{
                "expo": {
                    "ios": {},
                    "android": {}
                }
            }"#,
        );

        let issue_codes = release_metadata_issues(Path::new("/project"), &app_json)
            .into_iter()
            .map(|issue| issue.code)
            .collect::<Vec<_>>();

        assert_eq!(
            issue_codes,
            vec![
                "RNA_EXPO_META_001",
                "RNA_EXPO_META_002",
                "RNA_EXPO_META_003",
                "RNA_EXPO_IOS_002",
                "RNA_EXPO_ANDROID_002"
            ]
        );
    }
}
