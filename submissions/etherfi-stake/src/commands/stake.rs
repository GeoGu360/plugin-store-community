use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct StakeArgs {
    /// Amount of ETH to stake (in ETH, not wei). Example: 1.5
    #[arg(long)]
    pub amount_eth: f64,

    /// Referral address (optional, defaults to zero address)
    #[arg(long)]
    pub referral: Option<String>,

    /// Wallet address to stake from (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// If true, receive eETH directly via LiquidityPool instead of weETH via DepositAdapter
    #[arg(long, default_value_t = false)]
    pub prefer_eeth: bool,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Chain ID (currently only Ethereum mainnet / chain 1 is supported; accepted for compatibility)
    #[arg(long)]
    pub chain: Option<u64>,
}

pub async fn run(args: StakeArgs) -> anyhow::Result<()> {
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

    // ── Validate amount ───────────────────────────────────────────────────
    let amount_wei = (args.amount_eth * 1e18) as u128;
    if amount_wei == 0 {
        anyhow::bail!("Stake amount must be greater than 0");
    }

    // ── Choose path: DepositAdapter (weETH) or LiquidityPool (eETH) ──────
    let referral = args
        .referral
        .as_deref()
        .unwrap_or(config::ZERO_ADDRESS);

    let (to, calldata, receive_token) = if args.prefer_eeth {
        // LiquidityPool.deposit() — receive eETH
        let cd = rpc::calldata_deposit();
        (config::LIQUIDITY_POOL_ADDRESS, cd, "eETH")
    } else {
        // DepositAdapter.depositETHForWeETH(address) — receive weETH directly (preferred)
        let cd = rpc::calldata_deposit_eth_for_weeth(referral);
        (config::DEPOSIT_ADAPTER_ADDRESS, cd, "weETH")
    };

    // ── Display pre-flight summary ────────────────────────────────────────
    println!("=== ether.fi Stake ===");
    println!("From:          {}", wallet);
    println!("Amount:        {} ETH ({} wei)", args.amount_eth, amount_wei);
    println!("Receive token: {}", receive_token);
    println!("Referral:      {}", referral);
    println!("Contract:      {}", to);
    println!("Calldata:      {}", calldata);
    println!();

    // ── Fetch and display current APY ─────────────────────────────────────
    if let Some(apy_str) = fetch_apy_display().await {
        println!("{}", apy_str);
        println!();
    }

    // Ask user to confirm the transaction before submitting
    println!("Please confirm the transaction above before it is submitted.");

    if args.dry_run {
        println!("[dry-run] Transaction NOT submitted.");
        return Ok(());
    }

    // ── Submit transaction ────────────────────────────────────────────────
    println!("Submitting stake transaction...");
    let result = onchainos::wallet_contract_call(
        chain_id,
        to,
        &calldata,
        Some(&wallet),
        Some(amount_wei),
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!(
        "You will receive {} in your wallet. Check your balance with `etherfi-stake balance`.",
        receive_token
    );
    if receive_token == "weETH" {
        println!("weETH appreciates in value over time as restaking rewards accrue.");
    } else {
        println!("eETH is a rebasing token — its balance grows daily without transfers.");
    }

    Ok(())
}

/// Attempt to fetch and format the current APY string. Non-fatal on error.
async fn fetch_apy_display() -> Option<String> {
    let url = format!(
        "{}/chart/{}",
        config::DEFILLAMA_YIELDS_URL,
        config::DEFILLAMA_POOL_ID
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "etherfi-stake-plugin/0.1.0")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body: serde_json::Value = resp.json().await.ok()?;
    let latest = body["data"].as_array()?.last()?;
    let apy = latest["apy"].as_f64()?;
    let apy7d = latest["apyBase7d"].as_f64();
    match apy7d {
        Some(v) => Some(format!(
            "Current weETH APY: {:.3}% (7-day avg: {:.3}%)",
            apy, v
        )),
        None => Some(format!("Current weETH APY: {:.3}%", apy)),
    }
}
