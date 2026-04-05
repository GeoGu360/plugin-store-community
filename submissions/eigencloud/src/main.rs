mod commands;
mod config;
mod onchainos;
mod rpc;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "eigencloud", about = "EigenLayer restaking plugin for onchainos")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available EigenLayer strategies and their TVL
    Strategies,
    /// List active operators available for delegation
    Operators,
    /// Show restaked positions for a wallet address
    Positions(commands::positions::PositionsArgs),
    /// Stake LST tokens into an EigenLayer strategy
    Stake(commands::stake::StakeArgs),
    /// Delegate restaked stake to an operator
    Delegate(commands::delegate::DelegateArgs),
    /// Undelegate from current operator (queues withdrawal for all shares)
    Undelegate(commands::undelegate::UndelegateArgs),
    /// Queue a withdrawal of restaked shares
    QueueWithdrawal(commands::queue_withdrawal::QueueWithdrawalArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Strategies => commands::strategies::run().await,
        Commands::Operators => commands::operators::run().await,
        Commands::Positions(args) => commands::positions::run(args).await,
        Commands::Stake(args) => commands::stake::run(args).await,
        Commands::Delegate(args) => commands::delegate::run(args).await,
        Commands::Undelegate(args) => commands::undelegate::run(args).await,
        Commands::QueueWithdrawal(args) => commands::queue_withdrawal::run(args).await,
    }
}
