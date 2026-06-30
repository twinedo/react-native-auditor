use std::path::{Path, PathBuf};

use crate::issue::{Issue, Severity};
use crate::parsers::app_json::AppJson;
use crate::parsers::eas_json::EasJson;
use crate::parsers::package_json::{PackageJson, ProjectType};
use crate::rules::{app_config, eas, env, expo_release, lockfiles, reanimated};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageManager {
    Yarn,
    Npm,
    Pnpm,
    Bun,
    Unknown,
    Multiple,
}

impl PackageManager {
    pub fn label(&self) -> String {
        match self {
            PackageManager::Yarn => "Yarn".to_string(),
            PackageManager::Npm => "npm".to_string(),
            PackageManager::Pnpm => "pnpm".to_string(),
            PackageManager::Bun => "Bun".to_string(),
            PackageManager::Unknown => "Unknown".to_string(),
            PackageManager::Multiple => "Multiple / Ambiguous".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectScan {
    pub root: PathBuf,

    pub has_package_json: bool,
    pub has_app_json: bool,
    pub has_app_config_js: bool,
    pub has_app_config_ts: bool,
    pub has_eas_json: bool,

    pub has_env: bool,
    pub has_env_example: bool,

    pub lockfiles: Vec<PathBuf>,

    pub has_babel_config_js: bool,
    pub has_metro_config_js: bool,

    pub package_manager: PackageManager,

    pub project_type: ProjectType,
    pub package_json: Option<PackageJson>,
    pub app_json: Option<AppJson>,
    pub eas_json: Option<EasJson>,
    pub app_json_error: Option<String>,
    pub eas_json_error: Option<String>,
    pub package_json_error: Option<String>,
}

impl ProjectScan {
    pub fn scan(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();

        let has_package_json = root.join("package.json").exists();

        let has_app_json = root.join("app.json").exists();
        let has_app_config_js = root.join("app.config.js").exists();
        let has_app_config_ts = root.join("app.config.ts").exists();
        let has_eas_json = root.join("eas.json").exists();

        let has_env = root.join(".env").exists();
        let has_env_example = root.join(".env.example").exists();

        let has_yarn_lock = root.join("yarn.lock").exists();
        let has_package_lock = root.join("package-lock.json").exists();
        let has_pnpm_lock = root.join("pnpm-lock.yaml").exists();
        let has_bun_lock = root.join("bun.lock").exists();
        let has_bun_lockb = root.join("bun.lockb").exists();

        let has_babel_config_js = root.join("babel.config.js").exists();
        let has_metro_config_js = root.join("metro.config.js").exists();

        let lockfiles = [
            ("yarn.lock", has_yarn_lock),
            ("package-lock.json", has_package_lock),
            ("pnpm-lock.yaml", has_pnpm_lock),
            ("bun.lock", has_bun_lock),
            ("bun.lockb", has_bun_lockb),
        ]
        .iter()
        .filter_map(
            |(name, exists)| {
                if *exists {
                    Some(root.join(name))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

        let package_manager = detect_package_manager(&lockfiles);

        let app_json_path = root.join("app.json");
        let (app_json, app_json_error) = if app_json_path.exists() {
            match AppJson::read_from(&app_json_path) {
                Ok(app_json) => (Some(app_json), None),
                Err(error) => (None, Some(error)),
            }
        } else {
            (None, None)
        };

        let eas_json_path = root.join("eas.json");
        let (eas_json, eas_json_error) = if eas_json_path.exists() {
            match EasJson::read_from(&eas_json_path) {
                Ok(eas_json) => (Some(eas_json), None),
                Err(error) => (None, Some(error)),
            }
        } else {
            (None, None)
        };

        let package_json_path = root.join("package.json");

        let (package_json, package_json_error, project_type) = if package_json_path.exists() {
            match PackageJson::read_from(&package_json_path) {
                Ok(package_json) => {
                    let project_type = package_json.detect_project_type();
                    (Some(package_json), None, project_type)
                }
                Err(error) => (None, Some(error), ProjectType::Unknown),
            }
        } else {
            (None, None, ProjectType::Unknown)
        };

        Self {
            root,
            has_package_json,
            has_app_json,
            has_app_config_js,
            has_app_config_ts,
            has_eas_json,
            has_env,
            has_env_example,
            lockfiles,
            has_babel_config_js,
            has_metro_config_js,
            package_manager,
            package_json,
            app_json,
            eas_json,
            app_json_error,
            eas_json_error,
            package_json_error,
            project_type,
        }
    }

    pub fn issues(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        if !self.has_package_json {
            issues.push(Issue::new(
                "RNA_PROJECT_001",
                "Missing package.json",
                Severity::Error,
                "No package.json was found in the project root. React Native Auditor expects to run from a React Native or Expo project root.",
                Some(self.root.join("package.json")),
            ));
        }

        if let Some(issue) = lockfiles::multiple_lockfiles_issue(&self.lockfiles) {
            issues.push(issue);
        }

        if let Some(issue) =
            env::missing_env_example_issue(&self.root, self.has_env, self.has_env_example)
        {
            issues.push(issue);
        }

        if let Some(issue) = app_config::dynamic_app_config_issue(
            &self.root,
            self.has_app_config_js,
            self.has_app_config_ts,
        ) {
            issues.push(issue);
        }

        if let Some(error) = &self.package_json_error {
            issues.push(Issue {
                code: "RNA_PACKAGE_001".to_string(),
                title: "Invalid package.json".to_string(),
                severity: Severity::Error,
                message: error.to_string(),
                file_path: Some(self.root.join("package.json")),
            });
        }

        if let Some(issue) = reanimated::babel_setup_issue(
            &self.root,
            self.package_json.as_ref(),
            self.has_babel_config_js,
        ) {
            issues.push(issue);
        }

        if matches!(self.project_type, ProjectType::Expo) {
            if let Some(app_json) = &self.app_json {
                issues.extend(expo_release::release_metadata_issues(&self.root, app_json));

                if is_missing_string(app_json.ios_bundle_identifier()) {
                    issues.push(Issue::new(
                        "RNA_EXPO_IOS_001",
                        "Missing iOS bundle identifier",
                        Severity::Warning,
                        "expo.ios.bundleIdentifier is missing or empty. This field is important for iOS release and build identity.",
                        Some(self.root.join("app.json")),
                    ));
                }

                if is_missing_string(app_json.android_package()) {
                    issues.push(Issue::new(
                        "RNA_EXPO_ANDROID_001",
                        "Missing Android package name",
                        Severity::Warning,
                        "expo.android.package is missing or empty. This field is important for Android release and build identity.",
                        Some(self.root.join("app.json")),
                    ));
                }
            }
        }

        issues.extend(eas::release_readiness_issues(
            &self.root,
            matches!(self.project_type, ProjectType::Expo),
            self.has_eas_json,
            self.eas_json_error.as_ref(),
            self.eas_json.as_ref(),
        ));

        if self.app_json_error.is_some() {
            issues.push(Issue::new(
                "RNA_APP_JSON_001",
                "Invalid app.json",
                Severity::Warning,
                "app.json could not be parsed as valid JSON, so some static Expo config checks may not be able to run.",
                Some(self.root.join("app.json")),
            ));
        }

        if self.eas_json_error.is_some() {
            issues.push(Issue::new(
                "RNA_EAS_JSON_001",
                "Invalid eas.json",
                Severity::Warning,
                "eas.json could not be parsed as valid JSON, so some EAS release-readiness checks may not be able to run.",
                Some(self.root.join("eas.json")),
            ));
        }

        issues
    }
}

fn detect_package_manager(lockfiles: &[PathBuf]) -> PackageManager {
    match lockfiles.len() {
        0 => PackageManager::Unknown,
        1 => match lockfiles[0]
            .file_name()
            .and_then(|file_name| file_name.to_str())
        {
            Some("yarn.lock") => PackageManager::Yarn,
            Some("package-lock.json") => PackageManager::Npm,
            Some("pnpm-lock.yaml") => PackageManager::Pnpm,
            Some("bun.lock" | "bun.lockb") => PackageManager::Bun,
            _ => PackageManager::Unknown,
        },
        _ => PackageManager::Multiple,
    }
}

fn is_missing_string(value: Option<&str>) -> bool {
    value.is_none_or(|value| value.trim().is_empty())
}
