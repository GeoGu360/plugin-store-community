/// Resolve a token symbol or hex address to a hex address.
/// If the input is already a hex address (starts with 0x), return as-is.
pub fn resolve_token_address(symbol: &str, chain_id: u64) -> String {
    match (symbol.to_uppercase().as_str(), chain_id) {
        // BSC (56)
        ("WBNB", 56) | ("BNB", 56) => "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c",
        ("USDT", 56) => "0x55d398326f99059fF775485246999027B3197955",
        ("USDC", 56) => "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d",
        ("CAKE", 56) => "0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82",
        ("BTCB", 56) => "0x7130d2A12B9BCbFAe4f2634d864A1Ee1Ce3Ead9c",
        ("ETH", 56) => "0x2170Ed0880ac9A755fd29B2688956BD959F933F8",
        // Base (8453)
        ("WETH", 8453) | ("ETH", 8453) => "0x4200000000000000000000000000000000000006",
        ("USDC", 8453) => "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
        ("CBBTC", 8453) => "0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf",
        _ => symbol, // assume already hex address
    }
    .to_string()
}

pub fn rpc_url(chain_id: u64) -> anyhow::Result<String> {
    match chain_id {
        56 => Ok("https://bsc-rpc.publicnode.com".to_string()),
        8453 => Ok("https://base-rpc.publicnode.com".to_string()),
        _ => anyhow::bail!("Unsupported chain_id: {}. Supported: 56 (BSC), 8453 (Base)", chain_id),
    }
}

pub fn smart_router(chain_id: u64) -> anyhow::Result<&'static str> {
    match chain_id {
        56 => Ok("0x13f4EA83D0bd40E75C8222255bc855a974568Dd4"),
        8453 => Ok("0x678Aa4bF4E210cf2166753e054d5b7c31cc7fa86"),
        _ => anyhow::bail!("Unsupported chain_id: {}", chain_id),
    }
}

pub fn nfpm_address(_chain_id: u64) -> &'static str {
    "0x46A15B0b27311cedF172AB29E4f4766fbE7F4364"
}

pub fn quoter_v2_address(_chain_id: u64) -> &'static str {
    "0xB048Bbc1Ee6b733FFfCFb9e9CeF7375518e25997"
}

pub fn factory_address(_chain_id: u64) -> &'static str {
    "0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865"
}

/// Encode an int24 tick as a 32-byte ABI hex string (sign-extended).
pub fn encode_tick(tick: i32) -> String {
    if tick >= 0 {
        format!("{:0>64x}", tick as u64)
    } else {
        // sign-extend negative: fill upper bytes with ff, lower 4 bytes = two's complement
        format!(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffff{:08x}",
            tick as u32
        )
    }
}

/// Decode an ABI int256 hex string back to i32 (tick fits in last 8 hex chars).
#[allow(dead_code)]
pub fn decode_tick(hex: &str) -> i32 {
    let clean = hex.trim_start_matches("0x");
    let last8 = &clean[clean.len().saturating_sub(8)..];
    u32::from_str_radix(last8, 16).unwrap_or(0) as i32
}

/// Build ERC-20 approve calldata: approve(address,uint256).
/// Selector: 0x095ea7b3
pub fn build_approve_calldata(spender: &str, amount: u128) -> String {
    let spender_clean = spender.trim_start_matches("0x");
    let spender_padded = format!("{:0>64}", spender_clean);
    let amount_hex = format!("{:0>64x}", amount);
    format!("0x095ea7b3{}{}", spender_padded, amount_hex)
}

/// Pad an address to 32 bytes (no 0x prefix in output).
pub fn pad_address(addr: &str) -> String {
    let clean = addr.trim_start_matches("0x");
    format!("{:0>64}", clean)
}

/// Pad a u256 value to 32 bytes hex.
pub fn pad_u256(val: u128) -> String {
    format!("{:0>64x}", val)
}

/// Pad a u256 from a decimal string to 32 bytes hex.
#[allow(dead_code)]
pub fn pad_u256_str(val: &str) -> anyhow::Result<String> {
    let n: u128 = val.parse().map_err(|_| anyhow::anyhow!("Invalid u256: {}", val))?;
    Ok(pad_u256(n))
}

/// UINT128_MAX as 32-byte padded hex.
pub fn uint128_max_padded() -> String {
    format!("{:0>64x}", u128::MAX)
}

/// Current unix timestamp in seconds.
pub fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
