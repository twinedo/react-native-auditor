use clap::ValueEnum;

use crate::issue::{Issue, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FailOn {
    Info,
    Warning,
    Error,
    None,
}

pub fn should_fail(issues: &[Issue], fail_on: Option<FailOn>) -> bool {
    let Some(fail_on) = fail_on else {
        return false;
    };

    issues.iter().any(|issue| match fail_on {
        FailOn::Info => matches!(
            issue.severity,
            Severity::Info | Severity::Warning | Severity::Error
        ),
        FailOn::Warning => matches!(issue.severity, Severity::Warning | Severity::Error),
        FailOn::Error => matches!(issue.severity, Severity::Error),
        FailOn::None => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omitted_fail_on_never_fails_due_to_issues() {
        let issues = vec![
            issue(Severity::Info),
            issue(Severity::Warning),
            issue(Severity::Error),
        ];

        assert!(!should_fail(&issues, None));
    }

    #[test]
    fn fail_on_error_fails_only_on_error() {
        assert!(!should_fail(&[], Some(FailOn::Error)));
        assert!(!should_fail(&[issue(Severity::Info)], Some(FailOn::Error)));
        assert!(!should_fail(
            &[issue(Severity::Warning)],
            Some(FailOn::Error)
        ));
        assert!(should_fail(&[issue(Severity::Error)], Some(FailOn::Error)));
    }

    #[test]
    fn fail_on_warning_fails_on_warning_or_error() {
        assert!(!should_fail(
            &[issue(Severity::Info)],
            Some(FailOn::Warning)
        ));
        assert!(should_fail(
            &[issue(Severity::Warning)],
            Some(FailOn::Warning)
        ));
        assert!(should_fail(
            &[issue(Severity::Error)],
            Some(FailOn::Warning)
        ));
    }

    #[test]
    fn fail_on_info_fails_on_any_issue() {
        assert!(should_fail(&[issue(Severity::Info)], Some(FailOn::Info)));
        assert!(should_fail(&[issue(Severity::Warning)], Some(FailOn::Info)));
        assert!(should_fail(&[issue(Severity::Error)], Some(FailOn::Info)));
    }

    #[test]
    fn fail_on_none_never_fails() {
        let issues = vec![
            issue(Severity::Info),
            issue(Severity::Warning),
            issue(Severity::Error),
        ];

        assert!(!should_fail(&issues, Some(FailOn::None)));
    }

    #[test]
    fn empty_issue_list_never_fails() {
        assert!(!should_fail(&[], None));
        assert!(!should_fail(&[], Some(FailOn::Info)));
        assert!(!should_fail(&[], Some(FailOn::Warning)));
        assert!(!should_fail(&[], Some(FailOn::Error)));
        assert!(!should_fail(&[], Some(FailOn::None)));
    }

    fn issue(severity: Severity) -> Issue {
        Issue::new("TEST", "Test issue", severity, "Test message", None)
    }
}
