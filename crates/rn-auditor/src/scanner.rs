use std::path::{Path, PathBuf};

use crate::issue::{Issue, Severity};

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
}

impl ProjectScan {
    pub fn scan(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();

        let has_app_json = root.join("app.json").exists();
        let has_app_config_js = root.join("app.config.js").exists();
        let has_app_config_ts = root.join("app.config.ts").exists();
        let has_eas_json = root.join("eas.json").exists();
        let has_package_json = root.join("package.json").exists();

        let has_env = root.join(".env").exists();
        let has_env_example = root.join(".env.example").exists();

        let has_babel_config_js = root.join("babel.config.js").exists();
        let has_metro_config_js = root.join("metro.config.js").exists();

        let lockfile_names = [
            "yarn.lock",
            "package-lock.json",
            "pnpm-lock.yaml",
            "bun.lock",
            "bun.lockb",
        ];

        let lockfiles = lockfile_names
            .iter()
            .map(|name| root.join(name))
            .filter(|path| path.exists())
            .collect();

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
        }
    }

    pub fn issues(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        if !self.has_package_json {
            issues.push(Issue::new(
        "RNA_PROJECT_OO1",
        "Missing package.json",
        Severity::Error,
        "No package.json was found in the project root. React Native Auditor expects to run from a React Native or Expo project root.",
        Some(self.root.join("package.json"))
      ))
        }

        if self.lockfiles.len() > 1 {
            issues.push(Issue::new(
                "RNA_LOCK_001",
                "Multiple lockfiles detected",
                Severity::Warning,
                "Multiple package manager lockfiles were found. This can cause inconsistent installs across local machines and CI.",
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

        issues
    }
}
