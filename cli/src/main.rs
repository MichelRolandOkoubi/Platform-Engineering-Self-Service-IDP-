mod commands;
mod api_client;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "idpctl")]
#[command(about = "Internal Developer Platform CLI", long_about = None)]
struct Cli {
    @command(subcommand)
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy an application
    Deploy {
        #[arg(short, long)]
        name: String,
    },
    /// View application logs
    Logs {
        #[arg(short, long)]
        name: String,
    },
    /// Estimate cloud costs
    Cost {
        #[arg(short, long)]
        name: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Deploy { name } => {
            commands::deploy::execute(name).await;
        }
        Commands::Logs { name } => {
            commands::logs::execute(name).await;
        }
        Commands::Cost { name } => {
            commands::cost::execute(name).await;
        }
    }
}
