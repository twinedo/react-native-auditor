use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub code: String,
    pub title: String,
    pub severity: Severity,
    pub message: String,
    pub file_path: Option<PathBuf>,
}

impl Issue {
    pub fn new(
        code: impl Into<String>,
        title: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        file_path: Option<PathBuf>,
    ) -> Self {
        Self {
            code: code.into(),
            title: title.into(),
            severity,
            message: message.into(),
            file_path,
        }
    }
}
