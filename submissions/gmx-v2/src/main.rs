mod api;
mod commands;
mod config;
mod multicall;
mod onchainos;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "gmx-v2",
    version = "0.1.0",
    about = "Trade perpetuals and manage GM pool liquidity on GMX V2 (Arbitrum/Avalanche)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available GM markets
    GetMarkets {
        /// Chain ID: 42161 (Arbitrum) or 43114 (Avalanche)
        #[arg(long, default_value = "42161")]
        chain: u64,
    },

    /// Fetch current oracle prices for all tokens
    GetPrices {
        #[arg(long, default_value = "42161")]
        chain: u64,
    },

    /// Fetch open perpetual positions for a wallet
    GetPositions {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// Wallet address (defaults to logged-in onchainos wallet)
        #[arg(long)]
        account: Option<String>,
    },

    /// Open a leveraged long position
    OpenLong {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// GM market token address (e.g. 0x70d9...6336 for ETH/USD)
        #[arg(long)]
        market: String,
        /// Collateral ERC-20 token address (e.g. USDC)
        #[arg(long)]
        collateral_token: String,
        /// Position size in USD (e.g. 5000 for $5000)
        #[arg(long)]
        size_usd: f64,
        /// Collateral amount in token's smallest unit (e.g. 500000000 for 500 USDC)
        #[arg(long)]
        collateral_amount: u128,
        /// Oracle price in GMX 30-decimal format (from get-prices)
        #[arg(long)]
        oracle_price: u128,
        /// Execution fee in wei (default: 500000000000000)
        #[arg(long)]
        execution_fee: Option<u64>,
        /// Dry-run: print calldata without submitting
        #[arg(long)]
        dry_run: bool,
    },

    /// Open a leveraged short position
    OpenShort {
        #[arg(long, default_value = "42161")]
        chain: u64,
        #[arg(long)]
        market: String,
        #[arg(long)]
        collateral_token: String,
        #[arg(long)]
        size_usd: f64,
        #[arg(long)]
        collateral_amount: u128,
        #[arg(long)]
        oracle_price: u128,
        #[arg(long)]
        execution_fee: Option<u64>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Close an open perpetual position
    ClosePosition {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// GM market token address
        #[arg(long)]
        market: String,
        /// Collateral token address of the position
        #[arg(long)]
        collateral_token: String,
        /// USD size to close (use full position size to close entirely)
        #[arg(long)]
        size_usd: f64,
        /// Collateral to withdraw in token units (0 to leave collateral)
        #[arg(long, default_value = "0")]
        collateral_delta: u128,
        /// Whether the position is long (true) or short (false)
        #[arg(long)]
        is_long: bool,
        /// Oracle price in GMX 30-decimal format
        #[arg(long)]
        oracle_price: u128,
        #[arg(long)]
        execution_fee: Option<u64>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Swap tokens via GMX V2 market swap
    Swap {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// Input token address
        #[arg(long)]
        input_token: String,
        /// Input amount in token's smallest unit
        #[arg(long)]
        input_amount: u128,
        /// Market token address(es) to route through (comma-separated)
        #[arg(long, value_delimiter = ',')]
        swap_path: Vec<String>,
        /// Minimum output amount (slippage protection)
        #[arg(long, default_value = "0")]
        min_output: u128,
        #[arg(long)]
        execution_fee: Option<u64>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Deposit tokens into a GM pool (provide liquidity)
    DepositGm {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// GM market token address
        #[arg(long)]
        market: String,
        /// Long token address (e.g. WETH)
        #[arg(long)]
        long_token: String,
        /// Short token address (e.g. USDC)
        #[arg(long)]
        short_token: String,
        /// Long token amount in token units (0 if not depositing long side)
        #[arg(long, default_value = "0")]
        long_amount: u128,
        /// Short token amount in token units (0 if not depositing short side)
        #[arg(long, default_value = "0")]
        short_amount: u128,
        /// Minimum GM tokens to receive (0 = accept any)
        #[arg(long, default_value = "0")]
        min_gm_tokens: u128,
        #[arg(long)]
        execution_fee: Option<u64>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Withdraw GM tokens from a pool (remove liquidity)
    WithdrawGm {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// GM market token address (the token to burn)
        #[arg(long)]
        market: String,
        /// Amount of GM tokens to burn
        #[arg(long)]
        gm_amount: u128,
        /// Minimum long tokens to receive
        #[arg(long, default_value = "0")]
        min_long: u128,
        /// Minimum short tokens to receive
        #[arg(long, default_value = "0")]
        min_short: u128,
        #[arg(long)]
        execution_fee: Option<u64>,
        #[arg(long)]
        dry_run: bool,
    },

    /// Approve a token for the GMX Router (required before first trade)
    ApproveToken {
        #[arg(long, default_value = "42161")]
        chain: u64,
        /// ERC-20 token address to approve
        #[arg(long)]
        token: String,
        #[arg(long)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetMarkets { chain } => {
            commands::get_markets::run(chain).await?;
        }

        Commands::GetPrices { chain } => {
            commands::get_prices::run(chain).await?;
        }

        Commands::GetPositions { chain, account } => {
            commands::get_positions::run(chain, account).await?;
        }

        Commands::OpenLong {
            chain,
            market,
            collateral_token,
            size_usd,
            collateral_amount,
            oracle_price,
            execution_fee,
            dry_run,
        } => {
            commands::open_position::run(
                chain,
                &market,
                &collateral_token,
                size_usd,
                collateral_amount,
                true, // is_long
                oracle_price,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::OpenShort {
            chain,
            market,
            collateral_token,
            size_usd,
            collateral_amount,
            oracle_price,
            execution_fee,
            dry_run,
        } => {
            commands::open_position::run(
                chain,
                &market,
                &collateral_token,
                size_usd,
                collateral_amount,
                false, // is_long
                oracle_price,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::ClosePosition {
            chain,
            market,
            collateral_token,
            size_usd,
            collateral_delta,
            is_long,
            oracle_price,
            execution_fee,
            dry_run,
        } => {
            commands::close_position::run(
                chain,
                &market,
                &collateral_token,
                size_usd,
                collateral_delta,
                is_long,
                oracle_price,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::Swap {
            chain,
            input_token,
            input_amount,
            swap_path,
            min_output,
            execution_fee,
            dry_run,
        } => {
            commands::swap::run(
                chain,
                &input_token,
                input_amount,
                swap_path,
                min_output,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::DepositGm {
            chain,
            market,
            long_token,
            short_token,
            long_amount,
            short_amount,
            min_gm_tokens,
            execution_fee,
            dry_run,
        } => {
            commands::deposit_gm::run(
                chain,
                &market,
                &long_token,
                &short_token,
                long_amount,
                short_amount,
                min_gm_tokens,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::WithdrawGm {
            chain,
            market,
            gm_amount,
            min_long,
            min_short,
            execution_fee,
            dry_run,
        } => {
            commands::withdraw_gm::run(
                chain,
                &market,
                gm_amount,
                min_long,
                min_short,
                execution_fee,
                dry_run,
            )
            .await?;
        }

        Commands::ApproveToken {
            chain,
            token,
            dry_run,
        } => {
            commands::approve_token::run(chain, &token, dry_run).await?;
        }
    }

    Ok(())
}
