mod issue;
mod parsers;
mod quality_gate;
mod reporters;
mod rules;
mod scanner;

use clap::{ArgGroup, Parser, Subcommand};
use quality_gate::{should_fail, FailOn};
use reporters::{print_terminal_report, render_json_report, write_html_report, write_json_report};
use scanner::ProjectScan;
use std::{env, path::PathBuf, process::ExitCode};

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
    Audit {
        path: Option<PathBuf>,
        #[arg(long, value_enum)]
        fail_on: Option<FailOn>,
    },
    Scan {
        path: Option<PathBuf>,
        #[arg(long, value_enum)]
        fail_on: Option<FailOn>,
    },
    #[command(group(
        ArgGroup::new("format")
            .required(true)
            .multiple(false)
            .args(["html", "json"])
    ))]
    Report {
        #[arg(long)]
        html: bool,
        #[arg(long)]
        json: bool,
        path: Option<PathBuf>,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, value_enum)]
        fail_on: Option<FailOn>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Audit { path, fail_on } | Commands::Scan { path, fail_on } => {
            run_audit(path, fail_on)
        }
        Commands::Report {
            html,
            json: _,
            path,
            output,
            fail_on,
        } => {
            if html {
                run_html_report(path, output, fail_on)
            } else {
                run_json_report(path, output, fail_on)
            }
        }
    };

    match result {
        Ok(true) => ExitCode::FAILURE,
        Ok(false) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
fn run_audit(path: Option<PathBuf>, fail_on: Option<FailOn>) -> Result<bool, String> {
    let project_path = resolve_project_path(path)?;

    let scan = ProjectScan::scan(&project_path);
    let issues = scan.issues();

    print_terminal_report(&scan, &issues);

    Ok(should_fail(&issues, fail_on))
}

fn run_html_report(
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    fail_on: Option<FailOn>,
) -> Result<bool, String> {
    let project_path = resolve_project_path(path)?;
    let output_path = resolve_output_path(output, "rn-auditor-report.html")?;

    let scan = ProjectScan::scan(&project_path);
    let issues = scan.issues();

    write_html_report(&scan, &issues, &output_path)?;
    println!("HTML report written to: {}", output_path.display());

    Ok(should_fail(&issues, fail_on))
}

fn run_json_report(
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    fail_on: Option<FailOn>,
) -> Result<bool, String> {
    let project_path = resolve_project_path(path)?
        .canonicalize()
        .map_err(|error| format!("Failed to resolve project path: {error}"))?;
    let scan = ProjectScan::scan(&project_path);
    let issues = scan.issues();

    if let Some(output) = output {
        let output_path = resolve_output_path(Some(output), "rn-auditor-report.json")?;
        write_json_report(&scan, &issues, &output_path)?;
        println!("JSON report written to: {}", output_path.display());
    } else {
        print!("{}", render_json_report(&scan, &issues)?);
    }

    Ok(should_fail(&issues, fail_on))
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

fn resolve_output_path(output: Option<PathBuf>, default_name: &str) -> Result<PathBuf, String> {
    let current_dir =
        env::current_dir().map_err(|error| format!("Failed to read current directory: {error}"))?;

    Ok(match output {
        Some(path) if path.is_absolute() => path,
        Some(path) => current_dir.join(path),
        None => current_dir.join(default_name),
    })
}
