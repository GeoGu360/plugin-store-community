use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct StakeArgs {
    /// LST symbol to stake (stETH, rETH, cbETH). Default: stETH
    #[arg(long, default_value = "stETH")]
    pub symbol: String,

    /// Amount of LST to stake (in token units, e.g. 1.5)
    #[arg(long)]
    pub amount: f64,

    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub async fn run(args: StakeArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    // Find strategy for the given symbol
    let strategy_meta = config::STRATEGIES
        .iter()
        .find(|s| s.symbol.to_lowercase() == args.symbol.to_lowercase())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown symbol '{}'. Supported: stETH, rETH, cbETH, ETHx, ankrETH",
                args.symbol
            )
        })?;

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

    // Convert to wei
    let amount_wei = (args.amount * 1e18) as u128;
    if amount_wei == 0 {
        anyhow::bail!("Stake amount must be greater than 0");
    }

    // Check current LST balance
    let balance_calldata = rpc::calldata_single_address(config::SEL_BALANCE_OF, &wallet);
    let balance_result =
        onchainos::eth_call(chain_id, strategy_meta.token, &balance_calldata);
    let balance = if let Ok(res) = balance_result {
        if let Ok(data) = rpc::extract_return_data(&res) {
            rpc::decode_uint256(&data).unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };

    // Check current allowance
    let allowance_calldata =
        rpc::calldata_two_addresses(config::SEL_ALLOWANCE, &wallet, config::STRATEGY_MANAGER);
    let allowance_result =
        onchainos::eth_call(chain_id, strategy_meta.token, &allowance_calldata);
    let allowance = if let Ok(res) = allowance_result {
        if let Ok(data) = rpc::extract_return_data(&res) {
            rpc::decode_uint256(&data).unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };

    println!("=== EigenLayer Stake ===");
    println!("From:         {}", wallet);
    println!("Symbol:       {}", strategy_meta.symbol);
    println!("Amount:       {} {} ({} wei)", args.amount, strategy_meta.symbol, amount_wei);
    println!("Strategy:     {}", strategy_meta.address);
    println!("Token:        {}", strategy_meta.token);
    println!(
        "Balance:      {:.6} {}",
        balance as f64 / 1e18,
        strategy_meta.symbol
    );
    println!(
        "Allowance:    {:.6} {} for StrategyManager",
        allowance as f64 / 1e18,
        strategy_meta.symbol
    );

    if balance < amount_wei && !args.dry_run {
        anyhow::bail!(
            "Insufficient {} balance: have {:.6}, need {:.6}",
            strategy_meta.symbol,
            balance as f64 / 1e18,
            args.amount
        );
    }

    // Build calldatas
    let approve_calldata = rpc::calldata_approve(config::STRATEGY_MANAGER, amount_wei);
    let deposit_calldata = rpc::calldata_deposit_into_strategy(
        strategy_meta.address,
        strategy_meta.token,
        amount_wei,
    );

    println!();
    println!("Step 1 — Approve {} for StrategyManager:", strategy_meta.symbol);
    println!("  Calldata: {}", approve_calldata);
    println!("Step 2 — depositIntoStrategy:");
    println!("  Calldata: {}", deposit_calldata);

    if args.dry_run {
        println!();
        println!("[dry-run] Transactions NOT submitted.");
        println!("NOTE: Confirm with user before executing staking transactions.");
        return Ok(());
    }

    println!();
    println!("NOTE: Ask user to confirm before submitting.");
    println!("Submitting approve transaction...");
    if allowance < amount_wei {
        let approve_result = onchainos::wallet_contract_call(
            chain_id,
            strategy_meta.token,
            &approve_calldata,
            Some(&wallet),
            None,
            false,
        )
        .await?;
        let tx_hash = onchainos::extract_tx_hash(&approve_result);
        println!("Approve tx: {}", tx_hash);
    } else {
        println!("Allowance sufficient, skipping approve.");
    }

    println!("Submitting depositIntoStrategy transaction...");
    let deposit_result = onchainos::wallet_contract_call(
        chain_id,
        config::STRATEGY_MANAGER,
        &deposit_calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&deposit_result);
    println!("Deposit tx: {}", tx_hash);
    println!(
        "Successfully staked {:.6} {} into EigenLayer.",
        args.amount, strategy_meta.symbol
    );

    Ok(())
}
