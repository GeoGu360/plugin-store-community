use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct UnwrapArgs {
    /// Amount of weETH to unwrap back to eETH (in ETH units, e.g. 2.87)
    #[arg(long)]
    pub amount_eth: f64,

    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub async fn run(args: UnwrapArgs) -> anyhow::Result<()> {
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
        anyhow::bail!("Unwrap amount must be greater than 0");
    }

    // ── Fetch exchange rate to show expected eETH output ─────────────────
    let eeth_out_wei = fetch_eeth_by_weeth(amount_wei).unwrap_or(0);

    // ── Build calldata ────────────────────────────────────────────────────
    // weETH.unwrap(uint256) — no prior approve needed
    let calldata = rpc::calldata_unwrap(amount_wei);

    println!("=== ether.fi Unwrap weETH → eETH ===");
    println!("From:           {}", wallet);
    println!("weETH amount:   {} weETH ({} wei)", args.amount_eth, amount_wei);
    if eeth_out_wei > 0 {
        println!(
            "Expected eETH:  ~{} eETH ({} wei)",
            rpc::format_eth(eeth_out_wei),
            eeth_out_wei
        );
    }
    println!("Contract:       {}", config::WEETH_ADDRESS);
    println!("Calldata:       {}", calldata);
    println!();
    println!("No approval needed — unwrap is a direct weETH burn.");
    println!();

    // Ask user to confirm the transaction before submitting
    println!("Please confirm the unwrap transaction above before it is submitted.");

    if args.dry_run {
        println!("[dry-run] Transaction NOT submitted.");
        return Ok(());
    }

    // ── Submit transaction ────────────────────────────────────────────────
    println!("Submitting unwrap transaction...");
    let result = onchainos::wallet_contract_call(
        chain_id,
        config::WEETH_ADDRESS,
        &calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&result);
    println!("Unwrap transaction submitted: {}", tx_hash);
    println!();
    println!("eETH has been minted to your wallet. Check balance with `etherfi-stake balance`.");
    println!("eETH is a rebasing token — its balance grows daily without transfers.");

    Ok(())
}

/// Fetch how much eETH a given weETH amount is worth (read-only, non-fatal).
fn fetch_eeth_by_weeth(weeth_amount_wei: u128) -> Option<u128> {
    let calldata = rpc::calldata_get_eeth_by_weeth(weeth_amount_wei);
    let result =
        onchainos::eth_call(config::RPC_URL, config::WEETH_ADDRESS, &calldata).ok()?;
    let hex = rpc::extract_return_data(&result).ok()?;
    rpc::decode_uint256(&hex).ok()
}
