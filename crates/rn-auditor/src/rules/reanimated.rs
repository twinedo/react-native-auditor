use std::fs;
use std::path::{Path, PathBuf};

use crate::issue::{Issue, Severity};
use crate::parsers::package_json::PackageJson;

pub fn babel_setup_issue(
    root: &Path,
    package_json: Option<&PackageJson>,
    has_babel_config_js: bool,
) -> Option<Issue> {
    if !package_json
        .is_some_and(|package_json| package_json.has_dependency("react-native-reanimated"))
    {
        return None;
    }

    let babel_config_path = root.join("babel.config.js");

    if has_babel_config_js && babel_config_contains_reanimated_plugin(&babel_config_path) {
        return None;
    }

    Some(missing_reanimated_babel_plugin_issue(babel_config_path))
}

fn babel_config_contains_reanimated_plugin(path: &Path) -> bool {
    fs::read_to_string(path).is_ok_and(|content| content.contains("react-native-reanimated/plugin"))
}

fn missing_reanimated_babel_plugin_issue(file_path: PathBuf) -> Issue {
    Issue::new(
        "RNA_REANIMATED_001",
        "Missing Reanimated Babel plugin",
        Severity::Warning,
        "react-native-reanimated usually requires the Babel plugin react-native-reanimated/plugin so React Native and Reanimated setup works correctly.",
        Some(file_path),
    )
}
