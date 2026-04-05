use alloy_primitives::{U256, B256};
use anyhow::Result;
use crate::config::get_chain_config;
use crate::multicall::{
    build_multicall, encode_send_wnt, encode_create_order,
    parse_address, to_hex,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};

// ORDER_TYPE: 4 = MarketDecrease
const MARKET_DECREASE: u8 = 4;

const DEFAULT_EXECUTION_FEE: u64 = 500_000_000_000_000;

#[allow(clippy::too_many_arguments)]
pub async fn run(
    chain_id: u64,
    market: &str,
    collateral_token: &str,
    size_usd: f64,         // amount to close in USD (full position size to close entirely)
    collateral_delta: u128, // collateral to withdraw (set to 0 to keep collateral, full amount to close)
    is_long: bool,
    oracle_price_30dec: u128,
    execution_fee: Option<u64>,
    dry_run: bool,
) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;

    let wallet = if dry_run {
        "0x0000000000000000000000000000000000000001".to_string()
    } else {
        let w = resolve_wallet(chain_id)?;
        if w.is_empty() {
            anyhow::bail!("Cannot get wallet address. Ensure onchainos is logged in.");
        }
        w
    };

    let fee = execution_fee.unwrap_or(DEFAULT_EXECUTION_FEE);

    let size_delta_usd = U256::from((size_usd * 1e30) as u128);

    // For decrease: longs accept min price (oracle * 0.995), shorts accept max price (oracle * 1.005)
    let slippage_bps = 50u128;
    let acceptable_price = if is_long {
        U256::from(oracle_price_30dec - oracle_price_30dec * slippage_bps / 10_000)
    } else {
        U256::from(oracle_price_30dec + oracle_price_30dec * slippage_bps / 10_000)
    };

    let receiver = parse_address(&wallet)?;
    let market_addr = parse_address(market)?;
    let collateral_addr = parse_address(collateral_token)?;
    let order_vault = parse_address(cfg.order_vault)?;
    let execution_fee_u256 = U256::from(fee);

    // Close = sendWnt + createOrder (no sendTokens — collateral is already in position)
    let send_wnt = encode_send_wnt(order_vault, execution_fee_u256);
    let create_order = encode_create_order(
        receiver,
        receiver,
        market_addr,
        collateral_addr,
        vec![],
        size_delta_usd,
        U256::from(collateral_delta),
        U256::ZERO,
        acceptable_price,
        execution_fee_u256,
        U256::ZERO,
        MARKET_DECREASE,
        is_long,
        B256::ZERO,
    );

    let multicall_data = build_multicall(vec![send_wnt, create_order]);
    let hex_data = to_hex(&multicall_data);

    let direction = if is_long { "LONG" } else { "SHORT" };
    println!("Closing {} position on GMX V2", direction);
    println!("  Market:   {}", market);
    println!("  Size USD: ${:.2}", size_usd);
    println!("  Exec fee: {} wei", fee);

    if dry_run {
        println!("  [DRY RUN] Calldata: {}", hex_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit close-position order to GMX V2? (proceeding...)");
    let result = wallet_contract_call(
        chain_id,
        cfg.exchange_router,
        &hex_data,
        Some(fee),
        false,
    ).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!("Collateral + PnL will be returned once the keeper executes the order.");
    Ok(())
}
