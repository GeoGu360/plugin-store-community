use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct RequestWithdrawalArgs {
    /// Amount of eETH to withdraw (in ETH units, e.g. 1.5)
    #[arg(long)]
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

pub async fn run(args: RequestWithdrawalArgs) -> anyhow::Result<()> {
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
        anyhow::bail!("Withdrawal amount must be greater than 0");
    }

    // ── Build calldata ────────────────────────────────────────────────────
    // LiquidityPool.requestWithdraw(address recipient, uint256 amount)
    // Note: no prior approve() needed — LiquidityPool calls eETH.burnShares() internally.
    let calldata = rpc::calldata_request_withdraw(&wallet, amount_wei);

    println!("=== ether.fi Request Withdrawal ===");
    println!("From:       {}", wallet);
    println!("Recipient:  {}", wallet);
    println!("Amount:     {} eETH ({} wei)", args.amount_eth, amount_wei);
    println!("Contract:   {}", config::LIQUIDITY_POOL_ADDRESS);
    println!("Calldata:   {}", calldata);
    println!();
    println!("This will burn your eETH and mint a WithdrawRequestNFT (ERC-721) to your wallet.");
    println!("The NFT represents your right to claim ETH once the request is finalized.");
    println!("Typical finalization: a few hours, but may take longer during high demand.");
    println!("WARNING: Whoever holds the NFT at claim time receives the ETH.");
    println!();

    // Ask user to confirm the transaction before submitting
    println!("Please confirm the withdrawal request above before it is submitted.");

    if args.dry_run {
        println!("[dry-run] Transaction NOT submitted.");
        return Ok(());
    }

    // ── Submit transaction ────────────────────────────────────────────────
    println!("Submitting withdrawal request...");
    let result = onchainos::wallet_contract_call(
        chain_id,
        config::LIQUIDITY_POOL_ADDRESS,
        &calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!();
    println!("Withdrawal request submitted. A WithdrawRequestNFT has been minted to your wallet.");
    println!(
        "Use `etherfi-stake get-withdrawals --token-ids <ID>` to check finalization status."
    );

    Ok(())
}
