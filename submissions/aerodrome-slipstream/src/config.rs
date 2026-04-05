/// Resolve a token symbol or hex address to a hex address on Base (chain 8453).
/// If the input is already a hex address (starts with 0x), return as-is.
pub fn resolve_token_address(symbol: &str) -> String {
    match symbol.to_uppercase().as_str() {
        "WETH" | "ETH" => "0x4200000000000000000000000000000000000006",
        "USDC" => "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
        "CBBTC" => "0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf",
        "AERO" => "0x940181a94A35A4569E4529A3CDfB74e38FD98631",
        "DAI" => "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb",
        "USDT" => "0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2",
        "WSTETH" => "0xc1CBa3fCea344f92D9239c08C0568f6F2F0ee452",
        _ => symbol, // assume already hex address
    }
    .to_string()
}

/// RPC URL for Base (chain 8453).
pub fn rpc_url() -> &'static str {
    "https://base-rpc.publicnode.com"
}

/// Aerodrome Slipstream SwapRouter (CLSwapRouter) on Base.
pub fn swap_router() -> &'static str {
    "0xBE6D8f0d05cC4be24d5167a3eF062215bE6D18a5"
}

/// Aerodrome Slipstream NonfungiblePositionManager on Base.
pub fn nfpm_address() -> &'static str {
    "0x827922686190790b37229fd06084350E74485b72"
}

/// Aerodrome Slipstream CLFactory on Base.
pub fn factory_address() -> &'static str {
    "0x5e7BB104d84c7CB9B682AaC2F3d509f5F406809A"
}

/// Aerodrome Slipstream QuoterV2 on Base.
pub fn quoter_v2_address() -> &'static str {
    "0x254cF9E1E6e233aa1AC962CB9B05b2cfeAaE15b0"
}

/// All supported tick spacings for Aerodrome Slipstream.
/// These replace "fee tiers" — Slipstream uses tickSpacing (int24) to identify pool levels.
pub const ALL_TICK_SPACINGS: &[i32] = &[1, 50, 100, 200, 2000];

/// Encode an int24 tick as a 32-byte ABI hex string (sign-extended).
/// Positive ticks: left-zero-padded.
/// Negative ticks: two's complement sign-extended to 32 bytes (upper bytes = 0xff).
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

/// Pad a u128 value to 32 bytes hex.
pub fn pad_u256(val: u128) -> String {
    format!("{:0>64x}", val)
}

/// UINT128_MAX as 32-byte padded hex.
/// uint128 max = 0xffffffffffffffffffffffffffffffff
/// Padded to 32 bytes: 0000000000000000000000000000000 + ffffffffffffffffffffffffffffffff
pub fn uint128_max_padded() -> String {
    "00000000000000000000000000000000ffffffffffffffffffffffffffffffff".to_string()
}

/// Current unix timestamp in seconds.
pub fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
