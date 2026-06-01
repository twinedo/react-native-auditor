mod issue;
mod reporter;
mod scanner;

use clap::{Parser, Subcommand};
use reporter::print_terminal_report;
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

            let scan = ProjectScan::scan(&current_dir);
            let issues = scan.issues();

            print_terminal_report(&scan, &issues);
        }
    }
}
