use alloy_primitives::U256;
use anyhow::Result;
use crate::config::get_chain_config;
use crate::multicall::{
    build_multicall, encode_send_wnt, encode_send_tokens, encode_create_withdrawal,
    parse_address, to_hex,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};

const DEFAULT_EXECUTION_FEE: u64 = 1_000_000_000_000_000; // 0.001 ETH for withdrawals

#[allow(clippy::too_many_arguments)]
pub async fn run(
    chain_id: u64,
    market: &str,          // GM market token address (also the token to burn)
    gm_token_amount: u128, // amount of GM tokens to burn
    min_long_amount: u128,
    min_short_amount: u128,
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
    let withdrawal_vault = parse_address(cfg.withdrawal_vault)?;
    let execution_fee_u256 = U256::from(fee);

    // Multicall: sendWnt(fee) + sendTokens(gmToken → WithdrawalVault) + createWithdrawal
    let send_wnt = encode_send_wnt(withdrawal_vault, execution_fee_u256);
    let send_gm = encode_send_tokens(
        market_addr,
        withdrawal_vault,
        U256::from(gm_token_amount),
    );
    let create_withdrawal = encode_create_withdrawal(
        receiver,
        market_addr,
        U256::from(min_long_amount),
        U256::from(min_short_amount),
        execution_fee_u256,
    );

    let multicall_data = build_multicall(vec![send_wnt, send_gm, create_withdrawal]);
    let hex_data = to_hex(&multicall_data);

    println!("Withdraw from GM Pool on GMX V2");
    println!("  Market (GM token): {}", market);
    println!("  GM amount:         {}", gm_token_amount);
    println!("  Min long out:      {}", min_long_amount);
    println!("  Min short out:     {}", min_short_amount);
    println!("  Exec fee:          {} wei", fee);

    if dry_run {
        println!("  [DRY RUN] Calldata: {}", hex_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit withdraw-gm order to GMX V2? (proceeding...)");
    let result = wallet_contract_call(
        chain_id,
        cfg.exchange_router,
        &hex_data,
        Some(fee),
        false,
    ).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Transaction submitted: {}", tx_hash);
    println!("Underlying tokens will be returned after a keeper executes the withdrawal.");
    Ok(())
}
