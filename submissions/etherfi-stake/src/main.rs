mod commands;
mod config;
mod onchainos;
mod rpc;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "etherfi-stake",
    about = "ether.fi liquid restaking plugin for onchainos — stake ETH to receive eETH/weETH, manage withdrawals"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Stake ETH to receive weETH (or eETH) via ether.fi liquid restaking
    Stake(commands::stake::StakeArgs),
    /// Get current ether.fi weETH staking APY from DefiLlama
    GetApy,
    /// Get eETH and weETH balance for an address
    Balance(commands::balance::BalanceArgs),
    /// Request withdrawal of eETH from ether.fi LiquidityPool (burns eETH, mints WithdrawRequestNFT)
    RequestWithdrawal(commands::request_withdrawal::RequestWithdrawalArgs),
    /// Check status of pending withdrawal requests (WithdrawRequestNFT token IDs)
    GetWithdrawals(commands::get_withdrawals::GetWithdrawalsArgs),
    /// Claim finalized withdrawal(s) by WithdrawRequestNFT token ID(s)
    ClaimWithdrawal(commands::claim_withdrawal::ClaimWithdrawalArgs),
    /// Wrap eETH into weETH (requires prior eETH approval)
    Wrap(commands::wrap::WrapArgs),
    /// Unwrap weETH back to eETH
    Unwrap(commands::unwrap::UnwrapArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Stake(args) => commands::stake::run(args).await,
        Commands::GetApy => commands::get_apy::run().await,
        Commands::Balance(args) => commands::balance::run(args).await,
        Commands::RequestWithdrawal(args) => commands::request_withdrawal::run(args).await,
        Commands::GetWithdrawals(args) => commands::get_withdrawals::run(args).await,
        Commands::ClaimWithdrawal(args) => commands::claim_withdrawal::run(args).await,
        Commands::Wrap(args) => commands::wrap::run(args).await,
        Commands::Unwrap(args) => commands::unwrap::run(args).await,
    }
}
