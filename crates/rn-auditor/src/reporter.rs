use crate::issue::Issue;
use crate::scanner::ProjectScan;

pub fn print_terminal_report(scan: &ProjectScan, issues: &[Issue]) {
    println!("React Native Auditor");
    println!("Scanning: {}", scan.root.display());
    println!();

    print_detected_files(scan);
    println!();

    print_lockfiles(scan);
    println!();

    print_issues(issues);
}

fn print_detected_files(scan: &ProjectScan) {
    println!("Detected files:");
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
    println!();
    println!("Detected lockfiles:");

    if scan.lockfiles.is_empty() {
        println!("  none");
    } else {
        for lockfile in &scan.lockfiles {
            println!("  {}", lockfile.display());
        }
    }

    println!();
    println!("Package manager: {}", scan.package_manager.label());
    println!("Project type: {}", scan.project_type.label());
}

fn print_issues(issues: &[Issue]) {
    println!("Issues:");

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

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}
