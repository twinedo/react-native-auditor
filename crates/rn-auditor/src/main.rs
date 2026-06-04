mod app_json;
mod eas_json;
mod issue;
mod package_json;
mod reporter;
mod scanner;

use clap::{Parser, Subcommand};
use reporter::print_terminal_report;
use scanner::ProjectScan;
use std::{env, path::PathBuf, process};

#[derive(Parser, Debug)]
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

#[derive(Subcommand, Debug)]
enum Commands {
    Audit { path: Option<PathBuf> },
    Scan { path: Option<PathBuf> },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Audit { path } | Commands::Scan { path } => run_audit(path),
    };

    if let Err(error) = result {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}
fn run_audit(path: Option<PathBuf>) -> Result<(), String> {
    let project_path = resolve_project_path(path)?;

    let scan = ProjectScan::scan(&project_path);
    let issues = scan.issues();

    print_terminal_report(&scan, &issues);

    Ok(())
}

fn resolve_project_path(path: Option<PathBuf>) -> Result<PathBuf, String> {
    let project_path = match path {
        Some(path) => path,
        None => env::current_dir()
            .map_err(|error| format!("Failed to read current directory: {error}"))?,
    };

    if !project_path.exists() {
        return Err(format!("Path does not exist: {}", project_path.display()));
    }

    if !project_path.is_dir() {
        return Err(format!(
            "Path is not a directory: {}",
            project_path.display()
        ));
    }

    Ok(project_path)
}
