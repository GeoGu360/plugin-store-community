use anyhow::Context;
use serde_json::{json, Value};

/// Perform an eth_call via JSON-RPC.
pub async fn eth_call(to: &str, data: &str, rpc_url: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let body = json!({
        "jsonrpc": "2.0",
        "method": "eth_call",
        "params": [
            {"to": to, "data": data},
            "latest"
        ],
        "id": 1
    });
    let resp: Value = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .context("eth_call HTTP request failed")?
        .json()
        .await
        .context("eth_call JSON parse failed")?;
    if let Some(err) = resp.get("error") {
        anyhow::bail!("eth_call error: {}", err);
    }
    Ok(resp["result"].as_str().unwrap_or("0x").to_string())
}

/// Check ERC-20 allowance.
/// allowance(address owner, address spender) → uint256
/// Selector: 0xdd62ed3e
pub async fn get_allowance(
    token: &str,
    owner: &str,
    spender: &str,
    rpc_url: &str,
) -> anyhow::Result<u128> {
    let owner_padded = format!("{:0>64}", owner.trim_start_matches("0x"));
    let spender_padded = format!("{:0>64}", spender.trim_start_matches("0x"));
    let data = format!("0xdd62ed3e{}{}", owner_padded, spender_padded);
    let hex = eth_call(token, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");
    let trimmed = if clean.len() > 32 { &clean[clean.len() - 32..] } else { clean };
    Ok(u128::from_str_radix(trimmed, 16).unwrap_or(0))
}

/// Get ERC-20 balance.
/// balanceOf(address) → uint256
/// Selector: 0x70a08231
#[allow(dead_code)]
pub async fn get_balance(token: &str, owner: &str, rpc_url: &str) -> anyhow::Result<u128> {
    let owner_padded = format!("{:0>64}", owner.trim_start_matches("0x"));
    let data = format!("0x70a08231{}", owner_padded);
    let hex = eth_call(token, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");
    let trimmed = if clean.len() > 32 { &clean[clean.len() - 32..] } else { clean };
    Ok(u128::from_str_radix(trimmed, 16).unwrap_or(0))
}

/// CLFactory.getPool(address token0, address token1, int24 tickSpacing) → address
/// Selector: 0x28af8d0b
/// NOTE: Aerodrome Slipstream uses tickSpacing (int24) instead of fee (uint24).
pub async fn factory_get_pool(
    token0: &str,
    token1: &str,
    tick_spacing: i32,
    factory: &str,
    rpc_url: &str,
) -> anyhow::Result<String> {
    let t0 = format!("{:0>64}", token0.trim_start_matches("0x"));
    let t1 = format!("{:0>64}", token1.trim_start_matches("0x"));
    // tickSpacing is int24, encode as 32-byte signed integer
    let ts_hex = if tick_spacing >= 0 {
        format!("{:0>64x}", tick_spacing as u64)
    } else {
        format!(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffff{:08x}",
            tick_spacing as u32
        )
    };
    let data = format!("0x28af8d0b{}{}{}", t0, t1, ts_hex);
    let hex = eth_call(factory, &data, rpc_url).await?;
    // Result is 32-byte padded address — extract last 40 hex chars
    let clean = hex.trim_start_matches("0x");
    let addr = if clean.len() >= 40 {
        format!("0x{}", &clean[clean.len() - 40..])
    } else {
        "0x0000000000000000000000000000000000000000".to_string()
    };
    Ok(addr)
}

/// NonfungiblePositionManager.balanceOf(address) → uint256
/// Selector: 0x70a08231
pub async fn nfpm_balance_of(nfpm: &str, owner: &str, rpc_url: &str) -> anyhow::Result<u64> {
    let owner_padded = format!("{:0>64}", owner.trim_start_matches("0x"));
    let data = format!("0x70a08231{}", owner_padded);
    let hex = eth_call(nfpm, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");
    let trimmed = if clean.len() > 16 { &clean[clean.len() - 16..] } else { clean };
    Ok(u64::from_str_radix(trimmed, 16).unwrap_or(0))
}

/// NonfungiblePositionManager.tokenOfOwnerByIndex(address,uint256) → uint256
/// Selector: 0x2f745c59
pub async fn nfpm_token_of_owner_by_index(
    nfpm: &str,
    owner: &str,
    index: u64,
    rpc_url: &str,
) -> anyhow::Result<u128> {
    let owner_padded = format!("{:0>64}", owner.trim_start_matches("0x"));
    let index_hex = format!("{:0>64x}", index);
    let data = format!("0x2f745c59{}{}", owner_padded, index_hex);
    let hex = eth_call(nfpm, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");
    let trimmed = if clean.len() > 32 { &clean[clean.len() - 32..] } else { clean };
    Ok(u128::from_str_radix(trimmed, 16).unwrap_or(0))
}

/// Decoded position data from NonfungiblePositionManager.positions(tokenId).
/// Aerodrome Slipstream positions() layout:
/// (nonce, operator, token0, token1, tickSpacing, tickLower, tickUpper,
///  liquidity, feeGrowth0, feeGrowth1, tokensOwed0, tokensOwed1)
/// Note: word[4] is tickSpacing (int24), not fee (uint24) — Slipstream-specific.
#[derive(Debug)]
pub struct PositionData {
    #[allow(dead_code)]
    pub token_id: u128,
    pub token0: String,
    pub token1: String,
    pub tick_spacing: i32,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: u128,
    pub tokens_owed0: u128,
    pub tokens_owed1: u128,
}

/// NonfungiblePositionManager.positions(uint256) → (nonce, operator, token0, token1, tickSpacing,
///   tickLower, tickUpper, liquidity, feeGrowth0, feeGrowth1, tokensOwed0, tokensOwed1)
/// Selector: 0x99fbab88
pub async fn nfpm_positions(
    nfpm: &str,
    token_id: u128,
    rpc_url: &str,
) -> anyhow::Result<PositionData> {
    let token_id_hex = format!("{:0>64x}", token_id);
    let data = format!("0x99fbab88{}", token_id_hex);
    let hex = eth_call(nfpm, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");

    // Each ABI word is 64 hex chars (32 bytes).
    // Layout: nonce(0) operator(1) token0(2) token1(3) tickSpacing(4) tickLower(5) tickUpper(6)
    //         liquidity(7) feeGrowth0(8) feeGrowth1(9) tokensOwed0(10) tokensOwed1(11)
    let words: Vec<&str> = (0..12)
        .map(|i| {
            let start = i * 64;
            let end = start + 64;
            if end <= clean.len() { &clean[start..end] } else { "0" }
        })
        .collect();

    let parse_addr = |w: &str| -> String {
        if w.len() >= 40 {
            format!("0x{}", &w[w.len() - 40..])
        } else {
            "0x0000000000000000000000000000000000000000".to_string()
        }
    };

    // Decode int24 / int32 tick: last 8 hex chars, interpret as i32 (two's complement)
    let parse_tick = |w: &str| -> i32 {
        let last8 = if w.len() >= 8 { &w[w.len() - 8..] } else { w };
        u32::from_str_radix(last8, 16).unwrap_or(0) as i32
    };

    let parse_u128 = |w: &str| -> u128 {
        let trimmed = if w.len() > 32 { &w[w.len() - 32..] } else { w };
        u128::from_str_radix(trimmed, 16).unwrap_or(0)
    };

    Ok(PositionData {
        token_id,
        token0: parse_addr(words[2]),
        token1: parse_addr(words[3]),
        tick_spacing: parse_tick(words[4]), // word[4] = tickSpacing (Slipstream-specific)
        tick_lower: parse_tick(words[5]),
        tick_upper: parse_tick(words[6]),
        liquidity: parse_u128(words[7]),
        tokens_owed0: parse_u128(words[10]),
        tokens_owed1: parse_u128(words[11]),
    })
}

/// QuoterV2.quoteExactInputSingle(QuoteExactInputSingleParams)
/// Struct field order (from IQuoterV2.sol source):
///   tokenIn (address), tokenOut (address), amountIn (uint256), tickSpacing (int24), sqrtPriceLimitX96 (uint160)
/// Canonical ABI type: (address,address,uint256,int24,uint160)
/// → (uint256 amountOut, uint160 sqrtPriceX96After, uint32 initializedTicksCrossed, uint256 gasEstimate)
/// Selector: 0x9e7defe6  (keccak256 of "quoteExactInputSingle((address,address,uint256,int24,uint160))")
///
/// NOTE: The design.md selector table lists 0x4c2f129e which corresponds to
///   (address,address,int24,uint256,uint160) — that ordering is wrong.
/// The actual IQuoterV2.sol struct declares amountIn before tickSpacing.
/// Verified against live Base mainnet: 0x9e7defe6 returns correct quotes.
pub async fn quoter_exact_input_single(
    quoter: &str,
    token_in: &str,
    token_out: &str,
    tick_spacing: i32,
    amount_in: u128,
    rpc_url: &str,
) -> anyhow::Result<u128> {
    let t_in = format!("{:0>64}", token_in.trim_start_matches("0x"));
    let t_out = format!("{:0>64}", token_out.trim_start_matches("0x"));
    let amt = format!("{:0>64x}", amount_in);
    // tickSpacing as int24 → 32 bytes signed (after amountIn, matching struct field order)
    let ts_hex = if tick_spacing >= 0 {
        format!("{:0>64x}", tick_spacing as u64)
    } else {
        format!(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffff{:08x}",
            tick_spacing as u32
        )
    };
    let sqrt_limit = "0".repeat(64); // sqrtPriceLimitX96 = 0 (no limit)
    // Struct encoding: tokenIn, tokenOut, amountIn, tickSpacing, sqrtPriceLimitX96
    let data = format!("0x9e7defe6{}{}{}{}{}", t_in, t_out, amt, ts_hex, sqrt_limit);
    let hex = eth_call(quoter, &data, rpc_url).await?;
    let clean = hex.trim_start_matches("0x");
    // First ABI word (64 hex chars = 32 bytes) is amountOut.
    // Take last 32 hex chars of first word to get u128-safe value.
    let first_word = if clean.len() >= 64 { &clean[..64] } else { clean };
    let trimmed = if first_word.len() > 32 { &first_word[first_word.len() - 32..] } else { first_word };
    Ok(u128::from_str_radix(trimmed, 16).unwrap_or(0))
}
