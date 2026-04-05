use crate::{config, onchainos, rpc};
use clap::Args;

#[derive(Args)]
pub struct BalanceArgs {
    /// Address to query balances for (optional, resolved from onchainos if omitted)
    #[arg(long)]
    pub address: Option<String>,

    /// Chain ID (currently only Ethereum mainnet / chain 1 is supported; accepted for compatibility)
    #[arg(long)]
    pub chain: Option<u64>,
}

pub async fn run(args: BalanceArgs) -> anyhow::Result<()> {
    let chain_id = config::CHAIN_ID;

    let address = match args.address.clone() {
        Some(a) => a,
        None => onchainos::resolve_wallet(chain_id)?,
    };
    if address.is_empty() {
        anyhow::bail!("Cannot get wallet address. Pass --address or ensure onchainos is logged in.");
    }

    println!("=== ether.fi Balance ===");
    println!("Address: {}", address);
    println!();

    // ── eETH balance ──────────────────────────────────────────────────────
    let eeth_calldata = rpc::calldata_single_address(config::SEL_BALANCE_OF, &address);
    let eeth_result = onchainos::eth_call(config::RPC_URL, config::EETH_ADDRESS, &eeth_calldata)?;

    match rpc::extract_return_data(&eeth_result) {
        Ok(hex) => match rpc::decode_uint256(&hex) {
            Ok(wei) => println!(
                "eETH balance:  {} eETH ({} wei)",
                rpc::format_eth(wei),
                wei
            ),
            Err(e) => println!("eETH balance:  (decode error: {})", e),
        },
        Err(e) => println!("eETH balance:  (RPC error: {})", e),
    }

    // ── weETH balance ─────────────────────────────────────────────────────
    let weeth_calldata = rpc::calldata_single_address(config::SEL_BALANCE_OF, &address);
    let weeth_result =
        onchainos::eth_call(config::RPC_URL, config::WEETH_ADDRESS, &weeth_calldata)?;

    match rpc::extract_return_data(&weeth_result) {
        Ok(hex) => match rpc::decode_uint256(&hex) {
            Ok(wei) => println!(
                "weETH balance: {} weETH ({} wei)",
                rpc::format_eth(wei),
                wei
            ),
            Err(e) => println!("weETH balance: (decode error: {})", e),
        },
        Err(e) => println!("weETH balance: (RPC error: {})", e),
    }

    // ── weETH/eETH exchange rate ──────────────────────────────────────────
    let rate_calldata = rpc::calldata_get_rate();
    let rate_result = onchainos::eth_call(config::RPC_URL, config::WEETH_ADDRESS, &rate_calldata)?;

    match rpc::extract_return_data(&rate_result) {
        Ok(hex) => match rpc::decode_uint256(&hex) {
            Ok(rate_wei) => {
                let rate = rate_wei as f64 / 1e18;
                println!("Exchange rate: 1 weETH = {:.6} eETH", rate);
            }
            Err(_) => {}
        },
        Err(_) => {}
    }

    println!();
    println!("Note: eETH is a rebasing token — its balance grows daily without Transfer events.");
    println!("      weETH is non-rebasing; its ETH value increases as the exchange rate rises.");

    Ok(())
}
