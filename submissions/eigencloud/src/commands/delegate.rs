use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct DelegateArgs {
    /// Operator address to delegate to
    #[arg(long)]
    pub operator: String,

    /// Wallet address (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub from: Option<String>,

    /// Dry run — show calldata without broadcasting
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

pub async fn run(args: DelegateArgs) -> anyhow::Result<()> {
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

    // Check if already delegated
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
    let already_delegated = current_operator != zero_addr;

    // Build calldata for delegateTo
    let calldata = rpc::calldata_delegate_to(&args.operator);

    // Try to get operator name
    let op_name = config::KNOWN_OPERATORS
        .iter()
        .find(|o| o.address.to_lowercase() == args.operator.to_lowercase())
        .map(|o| o.name)
        .unwrap_or("unknown");

    println!("=== EigenLayer Delegate ===");
    println!("From:             {}", wallet);
    println!("Operator:         {} ({})", args.operator, op_name);
    println!("Contract:         {}", config::DELEGATION_MANAGER);
    println!("Calldata:         {}", calldata);

    if already_delegated {
        println!();
        println!(
            "WARNING: Already delegated to {}. Must undelegate first.",
            current_operator
        );
        if !args.dry_run {
            anyhow::bail!(
                "Already delegated to {}. Use 'eigencloud undelegate' first.",
                current_operator
            );
        }
    }

    if args.dry_run {
        println!();
        println!("[dry-run] Transaction NOT submitted.");
        println!("NOTE: Ask user to confirm before delegating.");
        return Ok(());
    }

    println!();
    println!("NOTE: Ask user to confirm before submitting delegation.");
    println!("Submitting delegateTo transaction...");

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
    println!("Delegate tx: {}", tx_hash);
    println!(
        "Successfully delegated to {} ({}).",
        args.operator, op_name
    );

    Ok(())
}
