use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct GetWithdrawalsArgs {
    /// Comma-separated WithdrawRequestNFT token IDs to check (e.g. 7842,7843)
    #[arg(long, value_delimiter = ',')]
    pub token_ids: Vec<u128>,

    /// Address to resolve (used for display only — token ID lookup requires known IDs)
    #[arg(long)]
    pub address: Option<String>,

    /// Chain ID (currently only Ethereum mainnet / chain 1 is supported; accepted for compatibility)
    #[arg(long)]
    pub chain: Option<u64>,
}

pub async fn run(args: GetWithdrawalsArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    if args.token_ids.is_empty() {
        // Address-only query: show helpful guidance without error
        let display_address = args.address.as_deref().unwrap_or("(not specified)");
        println!("=== ether.fi Withdrawal Requests ===");
        println!("Address:   {}", display_address);
        println!();
        println!("No token IDs specified. To check specific withdrawal requests, use:");
        println!("  etherfi-stake get-withdrawals --token-ids <ID1,ID2,...>");
        println!();
        println!("WithdrawRequestNFT contract: {}", config::WITHDRAW_REQUEST_NFT_ADDRESS);
        println!(
            "You can find your token IDs by checking your wallet on Etherscan or using:\n  onchainos wallet balance --chain 1"
        );
        return Ok(());
    }

    let address = args
        .address
        .clone()
        .unwrap_or_else(|| {
            onchainos::resolve_wallet(chain_id).unwrap_or_default()
        });

    println!("=== ether.fi Withdrawal Requests ===");
    if !address.is_empty() {
        println!("Address:   {}", address);
    }
    println!(
        "Checking {} token ID(s): {:?}",
        args.token_ids.len(),
        args.token_ids
    );
    println!();

    let mut any_claimable = false;

    for &token_id in &args.token_ids {
        println!("--- Token ID #{} ---", token_id);

        // ── getRequest(tokenId) → WithdrawRequest struct ──────────────────
        let req_calldata = rpc::calldata_get_request(token_id);
        let req_result = onchainos::eth_call(
            config::RPC_URL,
            config::WITHDRAW_REQUEST_NFT_ADDRESS,
            &req_calldata,
        )?;

        if let Ok(hex) = rpc::extract_return_data(&req_result) {
            let hex = hex.trim_start_matches("0x");
            // WithdrawRequest struct: amountOfEEth(uint96) + shareOfEEth(uint96) + isValid(bool) + feeGwei(uint32)
            // ABI packs each as 32-byte slot
            if hex.len() >= 4 * 64 {
                let amount_eeth_wei =
                    u128::from_str_radix(&hex[0..64], 16).unwrap_or(0);
                let is_valid =
                    u128::from_str_radix(&hex[2 * 64..3 * 64], 16).unwrap_or(0) != 0;
                let fee_gwei =
                    u128::from_str_radix(&hex[3 * 64..4 * 64], 16).unwrap_or(0);

                println!(
                    "  eETH amount at request: {} eETH ({} wei)",
                    rpc::format_eth(amount_eeth_wei),
                    amount_eeth_wei
                );
                println!("  Valid:                  {}", is_valid);
                println!("  Fee:                    {} gwei", fee_gwei);

                if !is_valid {
                    println!("  Status:  INVALIDATED by admin — this request cannot be claimed.");
                    println!();
                    continue;
                }
            }
        }

        // ── isFinalized(tokenId) → bool ───────────────────────────────────
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

        // ── getClaimableAmount(tokenId) → uint256 ─────────────────────────
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

        if finalized {
            println!(
                "  Status:   READY TO CLAIM — {} ETH available ({} wei)",
                rpc::format_eth(claimable_wei),
                claimable_wei
            );
            any_claimable = true;
        } else {
            println!("  Status:   PENDING — not yet finalized");
            println!(
                "  Typically finalizes within hours, but may take longer during high withdrawal demand."
            );
        }
        println!();
    }

    if any_claimable {
        println!(
            "Run `etherfi-stake claim-withdrawal --token-ids <IDs>` to claim ready withdrawals."
        );
    }

    Ok(())
}
