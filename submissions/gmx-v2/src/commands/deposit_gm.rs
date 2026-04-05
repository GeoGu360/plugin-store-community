use alloy_primitives::U256;
use anyhow::Result;
use crate::config::get_chain_config;
use crate::multicall::{
    build_multicall, encode_send_wnt, encode_send_tokens, encode_create_deposit,
    parse_address, to_hex,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};

const DEFAULT_EXECUTION_FEE: u64 = 1_000_000_000_000_000; // 0.001 ETH for deposits

#[allow(clippy::too_many_arguments)]
pub async fn run(
    chain_id: u64,
    market: &str,
    long_token: &str,
    short_token: &str,
    long_amount: u128,    // long token amount in token units (0 if not depositing long side)
    short_amount: u128,   // short token amount in token units (0 if not depositing short side)
    min_market_tokens: u128,
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
    let market_addr = parse_address(market)?;
    let long_token_addr = parse_address(long_token)?;
    let short_token_addr = parse_address(short_token)?;
    let deposit_vault = parse_address(cfg.deposit_vault)?;
    let execution_fee_u256 = U256::from(fee);

    let mut calls = vec![];

    // Always send execution fee as WNT first
    calls.push(encode_send_wnt(deposit_vault, execution_fee_u256));

    // Send long token if amount > 0
    if long_amount > 0 {
        calls.push(encode_send_tokens(
            long_token_addr,
            deposit_vault,
            U256::from(long_amount),
        ));
    }

    // Send short token if amount > 0
    if short_amount > 0 {
        calls.push(encode_send_tokens(
            short_token_addr,
            deposit_vault,
            U256::from(short_amount),
        ));
    }

    // Create deposit
    calls.push(encode_create_deposit(
        receiver,
        market_addr,
        long_token_addr,
        short_token_addr,
        U256::from(min_market_tokens),
        execution_fee_u256,
    ));

    let multicall_data = build_multicall(calls);
    let hex_data = to_hex(&multicall_data);

    println!("Deposit into GM Pool on GMX V2");
    println!("  Market:       {}", market);
    println!("  Long token:   {} (amount: {})", long_token, long_amount);
    println!("  Short token:  {} (amount: {})", short_token, short_amount);
    println!("  Min GM tokens:{}", min_market_tokens);
    println!("  Exec fee:     {} wei", fee);

    if dry_run {
        println!("  [DRY RUN] Calldata: {}", hex_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit deposit-gm order to GMX V2? (proceeding...)");
    let result = wallet_contract_call(
        chain_id,
        cfg.exchange_router,
        &hex_data,
        Some(fee),
        false,
    ).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!("GM tokens will be minted after a keeper executes the deposit (~5-30 seconds).");
    Ok(())
}
