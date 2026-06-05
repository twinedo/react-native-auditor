use crate::issue::Issue;
use crate::scanner::ProjectScan;

pub fn print_terminal_report(scan: &ProjectScan, issues: &[Issue]) {
    println!("React Native Auditor");
    println!();

    print_section_header("Scanning path");
    println!("  {}", scan.root.display());
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
        println!("  none");
    } else {
        for lockfile in &scan.lockfiles {
            println!("  {}", lockfile.display());
        }
    }
}

fn print_issues(issues: &[Issue]) {
    print_section_header("Issues");

    if issues.is_empty() {
        println!("  No issues found.");
        return;
    }

    for issue in issues {
        println!("  [{:?}] {} - {}", issue.severity, issue.code, issue.title);

        println!("      {}", issue.message);

        if let Some(path) = &issue.file_path {
            println!("      File: {}", path.display());
        }
    }
}

fn print_section_header(title: &str) {
    println!("{title}:");
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}
