use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustbrew")]
#[command(about = "A package manager written in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install { package: String },
    Update,
    Upgrade,
    Remove { package: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { package } => install_package(&package).await,
        Commands::Update => update_repos().await,
        Commands::Upgrade => upgrade_packages().await,
        Commands::Remove { package } => remove_package(&package).await,
    }
}
