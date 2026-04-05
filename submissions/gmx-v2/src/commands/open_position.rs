use alloy_primitives::{U256, B256};
use anyhow::Result;
use crate::config::get_chain_config;
use crate::multicall::{
    build_multicall, encode_send_wnt, encode_send_tokens, encode_create_order,
    parse_address, to_hex,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};

// ORDER_TYPE: 2 = MarketIncrease
const MARKET_INCREASE: u8 = 2;

/// Execution fee in wei (0.0005 ETH on Arbitrum)
const DEFAULT_EXECUTION_FEE: u64 = 500_000_000_000_000;

#[allow(clippy::too_many_arguments)]
pub async fn run(
    chain_id: u64,
    market: &str,
    collateral_token: &str,
    size_usd: f64,       // e.g. 5000.0 for $5000
    collateral_amount: u128, // in token's smallest unit (e.g. 500_000_000 for 500 USDC)
    is_long: bool,
    oracle_price_30dec: u128, // raw 30-dec price from API
    execution_fee: Option<u64>,
    dry_run: bool,
) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;

    let wallet = if dry_run {
        // Use a placeholder wallet for dry-run (no onchainos session needed)
        "0x0000000000000000000000000000000000000001".to_string()
    } else {
        let w = resolve_wallet(chain_id)?;
        if w.is_empty() {
            anyhow::bail!("Cannot get wallet address. Ensure onchainos is logged in.");
        }
        w
    };

    let fee = execution_fee.unwrap_or(DEFAULT_EXECUTION_FEE);

    // Convert size to 30-decimal
    let size_delta_usd = U256::from((size_usd * 1e30) as u128);

    // Acceptable price: oracle ± 0.5% slippage buffer, in 30-decimal
    let slippage_bps = 50u128; // 0.5%
    let acceptable_price = if is_long {
        // max price long: oracle * 1.005
        U256::from(oracle_price_30dec + oracle_price_30dec * slippage_bps / 10_000)
    } else {
        // min price short: oracle * 0.995
        U256::from(oracle_price_30dec - oracle_price_30dec * slippage_bps / 10_000)
    };

    let receiver = parse_address(&wallet)?;
    let market_addr = parse_address(market)?;
    let collateral_addr = parse_address(collateral_token)?;
    let order_vault = parse_address(cfg.order_vault)?;

    let execution_fee_u256 = U256::from(fee);
    let collateral_u256 = U256::from(collateral_amount);

    // Build multicall inner calls
    let send_wnt = encode_send_wnt(order_vault, execution_fee_u256);
    let send_tokens = encode_send_tokens(collateral_addr, order_vault, collateral_u256);
    let create_order = encode_create_order(
        receiver,
        receiver, // cancellation receiver = same wallet
        market_addr,
        collateral_addr,
        vec![],  // no swap path for perp
        size_delta_usd,
        collateral_u256,
        U256::ZERO, // trigger price = 0 for market orders
        acceptable_price,
        execution_fee_u256,
        U256::ZERO, // min output amount
        MARKET_INCREASE,
        is_long,
        B256::ZERO,
    );

    let multicall_data = build_multicall(vec![send_wnt, send_tokens, create_order]);
    let hex_data = to_hex(&multicall_data);

    let direction = if is_long { "LONG" } else { "SHORT" };
    println!("Opening {} position on GMX V2", direction);
    println!("  Market:     {}", market);
    println!("  Collateral: {} ({})", collateral_amount, collateral_token);
    println!("  Size USD:   ${:.2}", size_usd);
    println!("  Exec fee:   {} wei", fee);
    if dry_run {
        println!("  [DRY RUN] Calldata: {}", hex_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit open-{} order to GMX V2? (proceeding...)", direction.to_lowercase());
    let result = wallet_contract_call(
        chain_id,
        cfg.exchange_router,
        &hex_data,
        Some(fee),
        false,
    ).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!("Note: Order will execute when a GMX keeper processes it (~5-30 seconds).");
    Ok(())
}
