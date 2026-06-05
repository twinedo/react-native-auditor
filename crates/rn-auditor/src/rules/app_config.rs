use std::path::{Path, PathBuf};

use crate::issue::{Issue, Severity};

pub fn dynamic_app_config_issue(
    root: &Path,
    has_app_config_js: bool,
    has_app_config_ts: bool,
) -> Option<Issue> {
    dynamic_app_config_path(root, has_app_config_js, has_app_config_ts).map(|app_config_path| {
        Issue::new(
            "RNA_EXPO_CONFIG_001",
            "Dynamic Expo config detected",
            Severity::Info,
            "React Native Auditor detected a dynamic Expo config. JS/TS config files are not executed for security, so some static Expo checks may be limited or skipped.",
            Some(app_config_path),
        )
    })
}

fn dynamic_app_config_path(
    root: &Path,
    has_app_config_js: bool,
    has_app_config_ts: bool,
) -> Option<PathBuf> {
    if has_app_config_js {
        Some(root.join("app.config.js"))
    } else if has_app_config_ts {
        Some(root.join("app.config.ts"))
    } else {
        None
    }
}
