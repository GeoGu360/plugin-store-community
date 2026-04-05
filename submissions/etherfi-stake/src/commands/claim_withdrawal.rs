use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct ClaimWithdrawalArgs {
    /// Comma-separated WithdrawRequestNFT token IDs to claim (e.g. 7842,7843)
    #[arg(long, alias = "id", value_delimiter = ',')]
    pub token_ids: Vec<u128>,

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

pub async fn run(args: ClaimWithdrawalArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    if args.token_ids.is_empty() {
        anyhow::bail!("No token IDs provided. Use --token-ids <ID1,ID2,...>");
    }

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

    println!("=== ether.fi Claim Withdrawal ===");
    println!("From:      {}", wallet);
    println!("Token IDs: {:?}", args.token_ids);
    println!();

    // ── Build calldata (needed for both dry-run and live) ─────────────────
    let calldata = if args.token_ids.len() == 1 {
        rpc::calldata_claim_withdraw(args.token_ids[0])
    } else {
        rpc::calldata_batch_claim_withdraw(&args.token_ids)
    };

    println!("Contract:  {}", config::WITHDRAW_REQUEST_NFT_ADDRESS);
    println!("Calldata:  {}", calldata);
    println!();

    // Ask user to confirm the transaction before submitting
    println!("Please confirm the claim transaction above before it is submitted.");

    if args.dry_run {
        println!("[dry-run] Transaction NOT submitted.");
        return Ok(());
    }

    // ── Pre-flight: verify all token IDs are finalized (live only) ────────
    let mut total_claimable_wei: u128 = 0;
    let mut not_ready: Vec<u128> = Vec::new();

    for &token_id in &args.token_ids {
        let fin_calldata = rpc::calldata_is_finalized(token_id);
        let fin_result = onchainos::eth_call(
            config::RPC_URL,
            config::WITHDRAW_REQUEST_NFT_ADDRESS,
            &fin_calldata,
        )?;

        let finalized = match rpc::extract_return_data(&fin_result) {
            Ok(hex) => rpc::decode_bool(&hex).unwrap_or(false),
            Err(_) => false,
        };

        if finalized {
            let claimable_calldata = rpc::calldata_get_claimable_amount(token_id);
            let claimable_result = onchainos::eth_call(
                config::RPC_URL,
                config::WITHDRAW_REQUEST_NFT_ADDRESS,
                &claimable_calldata,
            )?;
            let claimable_wei = match rpc::extract_return_data(&claimable_result) {
                Ok(hex) => rpc::decode_uint256(&hex).unwrap_or(0),
                Err(_) => 0,
            };
            println!(
                "  Token #{}: READY — {} ETH claimable",
                token_id,
                rpc::format_eth(claimable_wei)
            );
            total_claimable_wei = total_claimable_wei.saturating_add(claimable_wei);
        } else {
            println!("  Token #{}: PENDING — not yet finalized", token_id);
            not_ready.push(token_id);
        }
    }

    if !not_ready.is_empty() {
        anyhow::bail!(
            "The following token IDs are not yet finalized: {:?}. \
             Use `etherfi-stake get-withdrawals --token-ids {:?}` to check status.",
            not_ready,
            not_ready
        );
    }

    println!();
    println!(
        "Total ETH to claim: {} ETH ({} wei)",
        rpc::format_eth(total_claimable_wei),
        total_claimable_wei
    );
    println!("The WithdrawRequestNFT(s) will be burned and ETH sent to msg.sender.");
    println!();

    // ── Submit transaction ────────────────────────────────────────────────
    println!("Submitting claim transaction...");
    let result = onchainos::wallet_contract_call(
        chain_id,
        config::WITHDRAW_REQUEST_NFT_ADDRESS,
        &calldata,
        Some(&wallet),
        None,
        false,
    )
    .await?;

    let tx_hash = onchainos::extract_tx_hash(&result);
    println!("Claim transaction submitted: {}", tx_hash);
    println!();
    println!(
        "ETH will be sent to your wallet ({}). NFT(s) burned.",
        wallet
    );
    println!("Check your ETH balance with `onchainos wallet balance --chain 1`.");

    Ok(())
}
