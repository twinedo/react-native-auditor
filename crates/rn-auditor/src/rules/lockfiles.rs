use std::path::PathBuf;

use crate::issue::{Issue, Severity};

pub fn multiple_lockfiles_issue(lockfiles: &[PathBuf]) -> Option<Issue> {
    if lockfiles.len() <= 1 {
        return None;
    }

    Some(Issue::new(
        "RNA_LOCKFILE_001",
        "Multiple lockfiles detected",
        Severity::Warning,
        "Multiple package manager lockfiles were found. This can cause dependency installs to differ between local machines and CI.",
        None,
    ))
}
