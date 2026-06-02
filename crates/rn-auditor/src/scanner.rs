use std::path::{Path, PathBuf};

use crate::issue::{Issue, Severity};
use crate::package_json::{PackageJson, ProjectType};

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

        let package_json_path = root.join("package.json");

        let (package_json_error, project_type) = if package_json_path.exists() {
            match PackageJson::read_from(&package_json_path) {
                Ok(package_json) => {
                    let project_type = package_json.detect_project_type();
                    (None, project_type)
                }
                Err(error) => (Some(error), ProjectType::Unknown),
            }
        } else {
            (None, ProjectType::Unknown)
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

        if self.lockfiles.len() > 1 {
            issues.push(Issue::new(
                "RNA_LOCKFILE_001",
                "Multiple lockfiles detected",
                Severity::Warning,
                "Multiple package manager lockfiles were found. This can cause dependency installs to differ between local machines and CI.",
                None,
            ));
        }

        if self.has_env && !self.has_env_example {
            issues.push(Issue::new(
                "RNA_ENV_001",
                "Missing .env.example",
                Severity::Warning,
                "A .env file exists, but .env.example was not found. Teams and CI environments may miss required environment variables.",
                Some(self.root.join(".env.example")),
            ));
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
