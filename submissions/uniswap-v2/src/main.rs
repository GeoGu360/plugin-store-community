use clap::{Parser, Subcommand};

mod commands;
mod config;
mod onchainos;
mod rpc;

#[derive(Parser)]
#[command(name = "uniswap-v2", about = "Uniswap V2 AMM plugin for Ethereum mainnet", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a swap quote (expected output amount) via getAmountsOut
    Quote {
        /// Input token symbol or address (e.g. ETH, USDC, 0x...)
        #[arg(long)]
        token_in: String,
        /// Output token symbol or address (e.g. USDT, 0x...)
        #[arg(long)]
        token_out: String,
        /// Input amount in raw units (wei)
        #[arg(long)]
        amount_in: u128,
    },

    /// Swap tokens on Uniswap V2
    Swap {
        /// Input token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_in: String,
        /// Output token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_out: String,
        /// Input amount in raw units (wei)
        #[arg(long)]
        amount_in: u128,
        /// Dry run: build calldata but do not broadcast
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// Add liquidity to a Uniswap V2 pool
    AddLiquidity {
        /// First token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_a: String,
        /// Second token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_b: String,
        /// Desired amount of tokenA in raw units (wei)
        #[arg(long)]
        amount_a: u128,
        /// Desired amount of tokenB in raw units (wei)
        #[arg(long)]
        amount_b: u128,
        /// Dry run: build calldata but do not broadcast
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// Remove liquidity from a Uniswap V2 pool
    RemoveLiquidity {
        /// First token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_a: String,
        /// Second token symbol or address (use ETH for native ETH)
        #[arg(long)]
        token_b: String,
        /// LP token amount to burn in raw units (omit = full balance)
        #[arg(long)]
        liquidity: Option<u128>,
        /// Dry run: build calldata but do not broadcast
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// Get the pair contract address for two tokens
    GetPair {
        /// First token symbol or address
        #[arg(long)]
        token_a: String,
        /// Second token symbol or address
        #[arg(long)]
        token_b: String,
    },

    /// Get the price of tokenA in tokenB from on-chain reserves
    GetPrice {
        /// Token to price (e.g. ETH, UNI)
        #[arg(long)]
        token_a: String,
        /// Quote token (e.g. USDC, USDT)
        #[arg(long)]
        token_b: String,
    },

    /// Get current reserves for a pair
    GetReserves {
        /// First token symbol or address
        #[arg(long)]
        token_a: String,
        /// Second token symbol or address
        #[arg(long)]
        token_b: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Quote { token_in, token_out, amount_in } => {
            commands::quote::run(&token_in, &token_out, amount_in).await?;
        }

        Commands::Swap { token_in, token_out, amount_in, dry_run } => {
            commands::swap::run(&token_in, &token_out, amount_in, dry_run).await?;
        }

        Commands::AddLiquidity { token_a, token_b, amount_a, amount_b, dry_run } => {
            commands::add_liquidity::run(&token_a, &token_b, amount_a, amount_b, dry_run).await?;
        }

        Commands::RemoveLiquidity { token_a, token_b, liquidity, dry_run } => {
            commands::remove_liquidity::run(&token_a, &token_b, liquidity, dry_run).await?;
        }

        Commands::GetPair { token_a, token_b } => {
            commands::get_pair::run(&token_a, &token_b).await?;
        }

        Commands::GetPrice { token_a, token_b } => {
            commands::get_price::run(&token_a, &token_b).await?;
        }

        Commands::GetReserves { token_a, token_b } => {
            commands::get_reserves::run(&token_a, &token_b).await?;
        }
    }

    Ok(())
}
