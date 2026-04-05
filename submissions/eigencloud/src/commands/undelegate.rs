use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct UndelegateArgs {
    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub async fn run(args: UndelegateArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

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

    // Check current delegation
    let delegated_calldata =
        rpc::calldata_single_address(config::SEL_DELEGATED_TO, &wallet);
    let delegated_result =
        onchainos::eth_call(chain_id, config::DELEGATION_MANAGER, &delegated_calldata);

    let current_operator = if let Ok(res) = delegated_result {
        if let Ok(data) = rpc::extract_return_data(&res) {
            rpc::decode_address(&data)
        } else {
            "0x0000000000000000000000000000000000000000".to_string()
        }
    } else {
        "0x0000000000000000000000000000000000000000".to_string()
    };

    let zero_addr = "0x0000000000000000000000000000000000000000";
    let is_delegated = current_operator != zero_addr;

    // Build calldata: undelegate(address staker)
    let calldata = rpc::calldata_undelegate(&wallet);

    println!("=== EigenLayer Undelegate ===");
    println!("From:             {}", wallet);
    println!("Current operator: {}", current_operator);
    println!("Contract:         {}", config::DELEGATION_MANAGER);
    println!("Calldata:         {}", calldata);

    if !is_delegated {
        println!();
        println!("Not currently delegated to any operator. Nothing to undelegate.");
        return Ok(());
    }

    println!();
    println!("WARNING: Undelegating will queue a withdrawal for ALL your restaked shares.");
    println!("         You will need to wait the withdrawal delay (~7 days) before completing.");

    if args.dry_run {
        println!();
        println!("[dry-run] Transaction NOT submitted.");
        println!("NOTE: Ask user to confirm before undelegating.");
        return Ok(());
    }

    println!();
    println!("NOTE: Ask user to confirm before submitting undelegation.");
    println!("Submitting undelegate transaction...");

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
    println!("Undelegate tx: {}", tx_hash);
    println!("Successfully undelegated. A withdrawal has been queued for your shares.");
    println!("Check withdrawal status with: eigencloud positions --address {}", wallet);

    Ok(())
}
