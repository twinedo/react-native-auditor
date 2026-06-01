use clap::{Parser, Subcommand};

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
            println!("React Native Auditor");
            println!("Running audit...");
        }
    }
}
