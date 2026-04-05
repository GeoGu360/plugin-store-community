use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct QueueWithdrawalArgs {
    /// Strategy address to withdraw from (use this OR --symbol)
    #[arg(long, default_value = "")]
    pub strategy: String,

    /// LST symbol shortcut (stETH, rETH, cbETH) — alternative to --strategy
    #[arg(long)]
    pub symbol: Option<String>,

    /// Number of shares to withdraw (in wei). Use 0 to withdraw all.
    #[arg(long, default_value_t = 0)]
    pub shares: u128,

    /// Wallet address that will receive the withdrawal (optional, defaults to caller)
    #[arg(long)]
    pub withdrawer: Option<String>,

    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub async fn run(args: QueueWithdrawalArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    // Determine strategy address
    let strategy_addr = if let Some(sym) = &args.symbol {
        config::STRATEGIES
            .iter()
            .find(|s| s.symbol.to_lowercase() == sym.to_lowercase())
            .map(|s| s.address.to_string())
            .ok_or_else(|| anyhow::anyhow!("Unknown symbol '{}'", sym))?
    } else if !args.strategy.is_empty() {
        args.strategy.clone()
    } else {
        anyhow::bail!("Must provide either --strategy <ADDRESS> or --symbol <SYMBOL>");
    };

    // Resolve wallet
    let wallet = if args.dry_run {
        args.from.clone().unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string())
    } else {
        args.from.clone().unwrap_or_else(|| {
            onchainos::resolve_wallet(chain_id).unwrap_or_default()
        })
    };

    if wallet.is_empty() || wallet == "0x" {
        anyhow::bail!("Cannot resolve wallet address. Pass --from or ensure onchainos is logged in.");
    }

    let withdrawer = args.withdrawer.clone().unwrap_or_else(|| wallet.clone());

    // Determine shares to withdraw
    let shares = if args.shares == 0 {
        // Query current shares in this strategy
        let deposits_calldata =
            rpc::calldata_single_address(config::SEL_GET_DEPOSITS, &wallet);
        let deposits_result =
            onchainos::eth_call(chain_id, config::STRATEGY_MANAGER, &deposits_calldata);
        let all_deposits = if let Ok(res) = deposits_result {
            if let Ok(data) = rpc::extract_return_data(&res) {
                rpc::decode_deposits(&data)
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        all_deposits
            .iter()
            .find(|(strat, _)| strat.to_lowercase() == strategy_addr.to_lowercase())
            .map(|(_, s)| *s)
            .unwrap_or(0)
    } else {
        args.shares
    };

    if shares == 0 && !args.dry_run {
        anyhow::bail!("No shares found in strategy {} for {}", strategy_addr, wallet);
    }

    let calldata = rpc::calldata_queue_withdrawal(&strategy_addr, shares, &withdrawer);

    let symbol = config::STRATEGIES
        .iter()
        .find(|s| s.address.to_lowercase() == strategy_addr.to_lowercase())
        .map(|s| s.symbol)
        .unwrap_or("unknown");

    println!("=== EigenLayer Queue Withdrawal ===");
    println!("From:         {}", wallet);
    println!("Strategy:     {} ({})", strategy_addr, symbol);
    println!("Shares:       {} ({:.6} {})", shares, shares as f64 / 1e18, symbol);
    println!("Withdrawer:   {}", withdrawer);
    println!("Contract:     {}", config::DELEGATION_MANAGER);
    println!("Calldata:     {}", calldata);
    println!();
    println!("NOTE: Withdrawal delay is approximately 7 days after queuing.");

    if args.dry_run {
        println!();
        println!("[dry-run] Transaction NOT submitted.");
        println!("NOTE: Ask user to confirm before queuing withdrawal.");
        return Ok(());
    }

    println!();
    println!("NOTE: Ask user to confirm before submitting withdrawal queue.");
    println!("Submitting queueWithdrawals transaction...");

    let result = onchainos::wallet_contract_call(
        chain_id,
        config::DELEGATION_MANAGER,
        &calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&result);
    println!("Queue withdrawal tx: {}", tx_hash);
    println!("Withdrawal queued. Wait ~7 days then complete via DelegationManager.completeQueuedWithdrawal().");

    Ok(())
}
