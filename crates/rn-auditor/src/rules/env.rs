use std::path::Path;

use crate::issue::{Issue, Severity};

pub fn missing_env_example_issue(
    root: &Path,
    has_env: bool,
    has_env_example: bool,
) -> Option<Issue> {
    if !has_env || has_env_example {
        return None;
    }

    Some(Issue::new(
        "RNA_ENV_001",
        "Missing .env.example",
        Severity::Warning,
        "This project uses a .env file but does not provide a .env.example file to document required environment variables.",
        Some(root.join(".env.example")),
    ))
}
