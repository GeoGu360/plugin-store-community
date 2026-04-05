use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct PositionsArgs {
    /// Wallet address to query (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub address: Option<String>,
}

pub async fn run(args: PositionsArgs) -> anyhow::Result<()> {
    let address = if let Some(addr) = args.address {
        addr
    } else {
        onchainos::resolve_wallet(config::CHAIN_ID)?
    };

    println!("=== EigenLayer Positions ===");
    println!("Address:  {}", address);
    println!();

    // Get all deposits via getDeposits(address)
    let deposits_calldata =
        rpc::calldata_single_address(config::SEL_GET_DEPOSITS, &address);
    let deposits_result =
        onchainos::eth_call(config::CHAIN_ID, config::STRATEGY_MANAGER, &deposits_calldata)?;

    let deposits = if let Ok(data) = rpc::extract_return_data(&deposits_result) {
        rpc::decode_deposits(&data)
    } else {
        vec![]
    };

    // Get delegated operator
    let delegated_calldata =
        rpc::calldata_single_address(config::SEL_DELEGATED_TO, &address);
    let delegated_result =
        onchainos::eth_call(config::CHAIN_ID, config::DELEGATION_MANAGER, &delegated_calldata)?;

    let operator = if let Ok(data) = rpc::extract_return_data(&delegated_result) {
        rpc::decode_address(&data)
    } else {
        "0x0000000000000000000000000000000000000000".to_string()
    };

    let zero_addr = "0x0000000000000000000000000000000000000000";
    let is_delegated = operator != zero_addr;

    // Display deposits
    if deposits.is_empty() {
        println!("  No restaked positions found.");
    } else {
        println!("  Restaked Positions:");
        for (strategy_addr, shares) in &deposits {
            // Try to map strategy address to symbol
            let symbol = config::STRATEGIES
                .iter()
                .find(|s| s.address.to_lowercase() == strategy_addr.to_lowercase())
                .map(|s| s.symbol)
                .unwrap_or("unknown");
            let shares_eth = *shares as f64 / 1e18;
            println!(
                "    Strategy: {} ({})  Shares: {:.6}",
                strategy_addr, symbol, shares_eth
            );
        }
    }

    println!();
    if is_delegated {
        // Try to map operator to known name
        let op_name = config::KNOWN_OPERATORS
            .iter()
            .find(|o| o.address.to_lowercase() == operator.to_lowercase())
            .map(|o| o.name)
            .unwrap_or("unknown");
        println!("  Delegated to: {} ({})", operator, op_name);
    } else {
        println!("  Not delegated to any operator.");
        println!("  Tip: Use 'eigencloud delegate --operator <ADDR>' to start earning AVS rewards.");
    }

    Ok(())
}
