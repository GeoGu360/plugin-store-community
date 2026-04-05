use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct WrapArgs {
    /// Amount of eETH to wrap into weETH (in ETH units, e.g. 3.0)
    #[arg(long, alias = "amount")]
    pub amount_eth: f64,

    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Chain ID (currently only Ethereum mainnet / chain 1 is supported; accepted for compatibility)
    #[arg(long)]
    pub chain: Option<u64>,
}

pub async fn run(args: WrapArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    // ── Resolve wallet (skip on dry_run; use zero address placeholder) ────
    let wallet = if args.dry_run {
        args.from
            .clone()
            .unwrap_or_else(|| config::ZERO_ADDRESS.to_string())
    } else {
        match args.from.clone() {
            Some(a) => a,
            None => onchainos::resolve_wallet(chain_id)?,
        }
    };
    if wallet.is_empty() {
        anyhow::bail!("Cannot get wallet address. Pass --from or ensure onchainos is logged in.");
    }

    let amount_wei = (args.amount_eth * 1e18) as u128;
    if amount_wei == 0 {
        anyhow::bail!("Wrap amount must be greater than 0");
    }

    // ── Check current eETH allowance for weETH contract ──────────────────
    let allowance_calldata =
        rpc::calldata_allowance(&wallet, config::WEETH_ADDRESS);
    let allowance_result =
        onchainos::eth_call(config::RPC_URL, config::EETH_ADDRESS, &allowance_calldata)?;

    let current_allowance = match rpc::extract_return_data(&allowance_result) {
        Ok(hex) => rpc::decode_uint256(&hex).unwrap_or(0),
        Err(_) => 0,
    };

    let needs_approve = current_allowance < amount_wei;

    // ── Build calldatas ───────────────────────────────────────────────────
    let approve_calldata = rpc::calldata_approve(config::WEETH_ADDRESS, amount_wei);
    let wrap_calldata = rpc::calldata_wrap(amount_wei);

    println!("=== ether.fi Wrap eETH → weETH ===");
    println!("From:              {}", wallet);
    println!("Amount:            {} eETH ({} wei)", args.amount_eth, amount_wei);
    println!("Current allowance: {} wei", current_allowance);
    println!();

    if needs_approve {
        println!("Step 1/2: Approve eETH to weETH contract");
        println!("  eETH contract:   {}", config::EETH_ADDRESS);
        println!("  Spender:         {}", config::WEETH_ADDRESS);
        println!("  Calldata:        {}", approve_calldata);
    } else {
        println!("Step 1/2: Approve — SKIPPED (sufficient allowance: {} wei)", current_allowance);
    }

    println!();
    println!("Step 2/2: Wrap eETH → weETH");
    println!("  weETH contract:  {}", config::WEETH_ADDRESS);
    println!("  Calldata:        {}", wrap_calldata);
    println!();

    // Ask user to confirm the transaction(s) before submitting
    println!("Please confirm the wrap operation above before it is submitted.");

    if args.dry_run {
        println!("[dry-run] Transaction(s) NOT submitted.");
        return Ok(());
    }

    // ── Step 1: Approve (if needed) ───────────────────────────────────────
    if needs_approve {
        println!("Step 1/2: Approving eETH spend...");
        let approve_result = onchainos::wallet_contract_call(
            chain_id,
            config::EETH_ADDRESS,
            &approve_calldata,
            Some(&wallet),
            None,
            false,
        )
        .await?;
        let approve_tx = onchainos::extract_tx_hash(&approve_result);
        println!("Approve tx: {}", approve_tx);
    }

    // ── Step 2: Wrap ──────────────────────────────────────────────────────
    println!("Step 2/2: Wrapping eETH to weETH...");
    let wrap_result = onchainos::wallet_contract_call(
        chain_id,
        config::WEETH_ADDRESS,
        &wrap_calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let wrap_tx = onchainos::extract_tx_hash(&wrap_result);
    println!("Wrap tx: {}", wrap_tx);
    println!();
    println!(
        "weETH has been minted to your wallet. Check balance with `etherfi-stake balance`."
    );
    println!("weETH is non-rebasing — its ETH value increases as restaking rewards accrue.");

    Ok(())
}
