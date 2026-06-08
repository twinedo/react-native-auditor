use std::env;
use std::fmt::Display;

use colored::{ColoredString, Colorize};

use crate::issue::{Issue, Severity};
use crate::scanner::ProjectScan;

const ISSUE_LINE_WIDTH: usize = 88;
const ISSUE_DETAIL_INDENT: &str = "      ";

pub fn print_terminal_report(scan: &ProjectScan, issues: &[Issue]) {
    configure_colors();

    println!("{}", title("React Native Auditor"));
    println!();

    print_section_header("Scanning path");
    println!("  {}", path(scan.root.display()));
    println!();

    print_project_summary(scan);
    println!();

    print_detected_files(scan);
    println!();

    print_lockfiles(scan);
    println!();

    print_issues(issues);
}

fn print_project_summary(scan: &ProjectScan) {
    print_section_header("Project summary");
    println!("  Project type: {}", scan.project_type.label());
    println!("  Package manager: {}", scan.package_manager.label());
}

fn print_detected_files(scan: &ProjectScan) {
    print_section_header("Detected files");
    println!("  package.json: {}", yes_no(scan.has_package_json));
    println!("  app.json: {}", yes_no(scan.has_app_json));
    println!("  app.config.js: {}", yes_no(scan.has_app_config_js));
    println!("  app.config.ts: {}", yes_no(scan.has_app_config_ts));
    println!("  eas.json: {}", yes_no(scan.has_eas_json));
    println!("  .env: {}", yes_no(scan.has_env));
    println!("  .env.example: {}", yes_no(scan.has_env_example));
    println!("  babel.config.js: {}", yes_no(scan.has_babel_config_js));
    println!("  metro.config.js: {}", yes_no(scan.has_metro_config_js));
}

fn print_lockfiles(scan: &ProjectScan) {
    print_section_header("Detected lockfiles");

    if scan.lockfiles.is_empty() {
        println!("  {}", "none".dimmed());
    } else {
        for lockfile in &scan.lockfiles {
            println!("  {}", path(lockfile.display()));
        }
    }
}

fn print_issues(issues: &[Issue]) {
    print_section_header("Issues");

    if issues.is_empty() {
        println!("  {}", success("No issues found."));
        return;
    }

    for (index, issue) in issues.iter().enumerate() {
        println!(
            "  {} {} — {}",
            severity_label(&issue.severity),
            code(&issue.code),
            issue.title
        );

        print_wrapped(ISSUE_DETAIL_INDENT, &issue.message, ISSUE_LINE_WIDTH);

        if let Some(file_path) = &issue.file_path {
            println!("{ISSUE_DETAIL_INDENT}File: {}", path(file_path.display()));
        }

        if index + 1 < issues.len() {
            println!();
        }
    }
}

fn print_wrapped(prefix: &str, text: &str, width: usize) {
    let content_width = width.saturating_sub(prefix.len()).max(1);

    for line in wrap_text(text, content_width) {
        println!("{prefix}{line}");
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();

    for paragraph in text.lines() {
        let mut current_line = String::new();

        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_owned();
            }
        }

        if current_line.is_empty() {
            lines.push(String::new());
        } else {
            lines.push(current_line);
        }
    }

    lines
}

fn print_section_header(title: &str) {
    println!("{}", section(&format!("{title}:")));
}

fn configure_colors() {
    if env::var_os("NO_COLOR").is_some() {
        colored::control::set_override(false);
    } else {
        colored::control::unset_override();
    }
}

fn title(value: &str) -> ColoredString {
    value.bold()
}

fn section(value: &str) -> ColoredString {
    value.bold()
}

fn success(value: &str) -> ColoredString {
    value.green()
}

fn warning(value: &str) -> ColoredString {
    value.yellow()
}

fn error(value: &str) -> ColoredString {
    value.red()
}

fn info(value: &str) -> ColoredString {
    value.cyan()
}

fn code(value: &str) -> ColoredString {
    value.bold()
}

fn path(value: impl Display) -> ColoredString {
    value.to_string().dimmed()
}

fn yes_no(value: bool) -> ColoredString {
    if value {
        "yes".green()
    } else {
        "no".dimmed()
    }
}

fn severity_label(severity: &Severity) -> ColoredString {
    match severity {
        Severity::Info => info("[Info]"),
        Severity::Warning => warning("[Warning]"),
        Severity::Error => error("[Error]"),
    }
}
