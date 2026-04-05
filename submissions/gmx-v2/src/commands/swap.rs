use alloy_primitives::{U256, B256};
use anyhow::Result;
use crate::config::get_chain_config;
use crate::multicall::{
    build_multicall, encode_send_wnt, encode_send_tokens, encode_create_order,
    parse_address, to_hex,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};

// ORDER_TYPE: 0 = MarketSwap
const MARKET_SWAP: u8 = 0;

const DEFAULT_EXECUTION_FEE: u64 = 500_000_000_000_000;

#[allow(clippy::too_many_arguments)]
pub async fn run(
    chain_id: u64,
    input_token: &str,
    input_amount: u128,    // in token's smallest unit
    swap_path: Vec<String>, // market token addresses for routing
    min_output_amount: u128,
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

    let receiver = parse_address(&wallet)?;
    let input_token_addr = parse_address(input_token)?;
    let order_vault = parse_address(cfg.order_vault)?;
    let execution_fee_u256 = U256::from(fee);
    let input_u256 = U256::from(input_amount);

    let path_addresses: Vec<alloy_primitives::Address> = swap_path
        .iter()
        .map(|s| parse_address(s))
        .collect::<anyhow::Result<Vec<_>>>()?;

    // Swap multicall: sendTokens + sendWnt + createOrder
    let send_tokens = encode_send_tokens(input_token_addr, order_vault, input_u256);
    let send_wnt = encode_send_wnt(order_vault, execution_fee_u256);
    let create_order = encode_create_order(
        receiver,
        receiver,
        alloy_primitives::Address::ZERO, // market = address(0) for pure swap
        input_token_addr,
        path_addresses,
        U256::ZERO,            // sizeDeltaUsd = 0 for swap
        input_u256,
        U256::ZERO,            // trigger price
        U256::ZERO,            // acceptable price (not needed for swap)
        execution_fee_u256,
        U256::from(min_output_amount),
        MARKET_SWAP,
        false,                 // isLong not applicable for swap
        B256::ZERO,
    );

    let multicall_data = build_multicall(vec![send_tokens, send_wnt, create_order]);
    let hex_data = to_hex(&multicall_data);

    println!("Swap via GMX V2");
    println!("  Input token:  {}", input_token);
    println!("  Input amount: {}", input_amount);
    println!("  Swap path:    {:?}", swap_path);
    println!("  Min output:   {}", min_output_amount);
    println!("  Exec fee:     {} wei", fee);

    if dry_run {
        println!("  [DRY RUN] Calldata: {}", hex_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit swap order to GMX V2? (proceeding...)");
    let result = wallet_contract_call(
        chain_id,
        cfg.exchange_router,
        &hex_data,
        Some(fee),
        false,
    ).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!("Swap will execute when a keeper processes the order (~5-30 seconds).");
    Ok(())
}
