mod issue;
mod scanner;

use clap::{Parser, Subcommand};
use scanner::ProjectScan;
use std::env;

#[derive(Parser)]
#[command(name = "rn-auditor")]
#[command(version)]
#[command(about = "React Native Auditor")]
#[command(
    long_about = "A local-first React Native & Expo project auditor for release readiness, dependency health, and risk reports."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Audit,
    Scan,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Audit | Commands::Scan => {
            let current_dir = env::current_dir().expect("Failed to read current directory");

            println!("React Native Auditor");
            println!("Scanning: {}", current_dir.display());
            println!();

            let scan = ProjectScan::scan(&current_dir);
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

            println!();
            println!("Detected lockfiles:");

            if scan.lockfiles.is_empty() {
                println!("  none");
            } else {
                for lockfile in &scan.lockfiles {
                    println!("  {}", lockfile.display());
                }
            }

            let issues = scan.issues();
            println!();
            println!("Issues:");

            if issues.is_empty() {
                println!("  No issues found.");
            } else {
                for issue in issues {
                    println!("  [{:?}] {} - {}", issue.severity, issue.code, issue.title);

                    println!("      {}", issue.message);

                    if let Some(path) = issue.file_path {
                        println!("      File: {}", path.display());
                    }
                }
            }
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
