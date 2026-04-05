use crate::config::{resolve_token_address, FACTORY_V2, ETH_RPC, CHAIN_ID};
use crate::rpc::{factory_get_pair, get_reserves, get_token0};

/// Get the spot price of tokenA denominated in tokenB, derived from on-chain reserves.
///
/// For tokens with different decimals (e.g. WETH=18, USDC=6), the raw reserve ratio
/// is adjusted: price = (reserve_b * 10^decimals_a) / (reserve_a * 10^decimals_b).
/// This plugin hard-codes decimals for known tokens and falls back to the raw ratio
/// for unknown tokens — users can adjust externally if needed.
pub async fn run(token_a: &str, token_b: &str) -> anyhow::Result<()> {
    let chain_id = CHAIN_ID;
    let rpc      = ETH_RPC;

    let addr_a = resolve_token_address(token_a, chain_id);
    let addr_b = resolve_token_address(token_b, chain_id);

    let pair = factory_get_pair(FACTORY_V2, &addr_a, &addr_b, rpc).await?;
    if pair == "0x0000000000000000000000000000000000000000" {
        anyhow::bail!("Pair does not exist for {} / {}", token_a, token_b);
    }

    let (r0, r1) = get_reserves(&pair, rpc).await?;
    let token0   = get_token0(&pair, rpc).await?;

    let (reserve_a, reserve_b) = if token0.to_lowercase() == addr_a.to_lowercase() {
        (r0, r1)
    } else {
        (r1, r0)
    };

    if reserve_a == 0 {
        anyhow::bail!("Reserve for {} is zero — pool may be empty", token_a);
    }

    // Decimal-adjusted price calculation for known Ethereum tokens
    let decimals_a = token_decimals(token_a, &addr_a);
    let decimals_b = token_decimals(token_b, &addr_b);

    // Compute price as f64: 1 tokenA = ? tokenB (human units)
    // price = (reserve_b / 10^decimals_b) / (reserve_a / 10^decimals_a)
    //       = reserve_b * 10^decimals_a / (reserve_a * 10^decimals_b)
    let scale_a = 10f64.powi(decimals_a as i32);
    let scale_b = 10f64.powi(decimals_b as i32);
    let price = (reserve_b as f64 / scale_b) / (reserve_a as f64 / scale_a);

    println!("Uniswap V2 Price");
    println!("  pair:              {}", pair);
    println!("  token0:            {}", token0);
    println!("  {} reserve (raw): {}", token_a.to_uppercase(), reserve_a);
    println!("  {} reserve (raw): {}", token_b.to_uppercase(), reserve_b);
    println!(
        "  1 {} = {:.6} {} (decimal-adjusted, {}-dec/{}-dec)",
        token_a.to_uppercase(), price, token_b.to_uppercase(), decimals_a, decimals_b
    );

    Ok(())
}

/// Return the decimal count for known Ethereum tokens.
/// Defaults to 18 for unknown tokens.
fn token_decimals(symbol: &str, addr: &str) -> u8 {
    // Match by symbol first, then by address (lowercase) for robustness
    match symbol.to_uppercase().as_str() {
        "ETH" | "WETH" => return 18,
        "USDT"         => return 6,
        "USDC"         => return 6,
        "DAI"          => return 18,
        "WBTC"         => return 8,
        "UNI"          => return 18,
        _              => {}
    }
    match addr.to_lowercase().as_str() {
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" => 18, // WETH
        "0xdac17f958d2ee523a2206206994597c13d831ec7" => 6,  // USDT
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" => 6,  // USDC
        "0x6b175474e89094c44da98b954eedeac495271d0f" => 18, // DAI
        "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" => 8,  // WBTC
        "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984" => 18, // UNI
        _                                            => 18, // default
    }
}
