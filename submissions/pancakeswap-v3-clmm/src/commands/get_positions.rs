use clap::Args;
use crate::config::{nfpm_address, rpc_url};
use crate::onchainos::resolve_wallet;
use crate::rpc::{nfpm_balance_of, nfpm_positions, nfpm_token_of_owner_by_index};

#[derive(Args)]
pub struct GetPositionsArgs {
    /// Wallet address to query. Defaults to the connected onchainos wallet.
    #[arg(long)]
    pub owner: Option<String>,
    /// Chain ID (56=BSC, 8453=Base)
    #[arg(long, default_value = "56")]
    pub chain: u64,
}

pub async fn run(args: GetPositionsArgs) -> anyhow::Result<()> {
    let rpc = rpc_url(args.chain)?;
    let nfpm = nfpm_address(args.chain);

    let owner = match args.owner {
        Some(addr) => addr,
        None => resolve_wallet(args.chain)?,
    };

    println!("Fetching positions for wallet: {}", owner);

    // --- 1. Get total NFT count ---
    let count = nfpm_balance_of(nfpm, &owner, &rpc).await?;
    println!("Total positions: {}", count);

    if count == 0 {
        println!("{{\"ok\":true,\"owner\":\"{}\",\"positions\":[]}}", owner);
        return Ok(());
    }

    // --- 2. Enumerate token IDs ---
    let mut positions = Vec::new();
    for i in 0..count {
        let token_id = nfpm_token_of_owner_by_index(nfpm, &owner, i, &rpc).await?;
        match nfpm_positions(nfpm, token_id, &rpc).await {
            Ok(pos) => {
                println!(
                    "  #{}: token0={} token1={} fee={} tickLower={} tickUpper={} liquidity={} owed0={} owed1={}",
                    token_id,
                    pos.token0,
                    pos.token1,
                    pos.fee,
                    pos.tick_lower,
                    pos.tick_upper,
                    pos.liquidity,
                    pos.tokens_owed0,
                    pos.tokens_owed1
                );
                positions.push(serde_json::json!({
                    "tokenId": token_id,
                    "token0": pos.token0,
                    "token1": pos.token1,
                    "fee": pos.fee,
                    "tickLower": pos.tick_lower,
                    "tickUpper": pos.tick_upper,
                    "liquidity": pos.liquidity.to_string(),
                    "tokensOwed0": pos.tokens_owed0.to_string(),
                    "tokensOwed1": pos.tokens_owed1.to_string(),
                }));
            }
            Err(e) => {
                println!("  #{}: error fetching position: {}", token_id, e);
            }
        }
    }

    println!(
        "{{\"ok\":true,\"owner\":\"{}\",\"positions\":{}}}",
        owner,
        serde_json::to_string(&positions)?
    );

    Ok(())
}
