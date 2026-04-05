use anyhow::Result;
use crate::config::get_chain_config;
use crate::onchainos::{wallet_contract_call, extract_tx_hash};

pub async fn run(
    chain_id: u64,
    token: &str,    // ERC-20 token address to approve
    dry_run: bool,
) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;

    // Build approve(router, MaxUint256) calldata manually
    // selector: 0x095ea7b3
    // param[0]: spender (Router) padded to 32 bytes
    // param[1]: amount (MaxUint256 = 0xff...ff)
    let router = cfg.router.strip_prefix("0x").unwrap_or(cfg.router);
    let input_data = format!(
        "0x095ea7b3{:0>64}{}",
        router.to_lowercase(),
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    );

    println!("Approve token for GMX Router");
    println!("  Token:   {}", token);
    println!("  Spender: {}", cfg.router);
    println!("  Amount:  MaxUint256 (unlimited)");

    if dry_run {
        println!("  [DRY RUN] Calldata: {}", input_data);
        return Ok(());
    }

    println!("\nPlease confirm: submit ERC-20 approve to GMX Router? (proceeding...)");
    let result = wallet_contract_call(chain_id, token, &input_data, None, false).await?;

    let tx_hash = extract_tx_hash(&result);
    println!("Approval transaction submitted: {}", tx_hash);
    Ok(())
}
