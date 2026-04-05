/// Resolve a token symbol or hex address to a hex address.
/// If the input is already a hex address (starts with 0x and length 42), return as-is.
pub fn resolve_token_address(symbol: &str, chain_id: u64) -> String {
    match (symbol.to_uppercase().as_str(), chain_id) {
        // Ethereum mainnet (1)
        ("ETH", 1) | ("WETH", 1) => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
        ("USDT", 1)               => "0xdAC17F958D2ee523a2206206994597C13D831ec7",
        ("USDC", 1)               => "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        ("DAI", 1)                => "0x6B175474E89094C44Da98b954EedeAC495271d0F",
        ("WBTC", 1)               => "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
        ("UNI", 1)                => "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
        _                         => symbol, // assume already a hex address
    }
    .to_string()
}

/// Returns true if the given symbol is native ETH (not WETH ERC-20).
pub fn is_native_eth(symbol: &str) -> bool {
    symbol.to_uppercase() == "ETH"
}

pub const ROUTER_V2: &str   = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
pub const FACTORY_V2: &str  = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
pub const WETH: &str         = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const ETH_RPC: &str      = "https://ethereum.publicnode.com";
pub const CHAIN_ID: u64      = 1;

/// Deadline: current unix timestamp + 20 minutes.
pub fn deadline() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + 1200
}

/// Apply 0.5% slippage (995/1000).
pub fn apply_slippage(amount: u128) -> u128 {
    amount * 995 / 1000
}

/// Pad an address (with or without 0x) to 32 bytes hex (no 0x prefix).
pub fn pad_address(addr: &str) -> String {
    let clean = addr.trim_start_matches("0x");
    format!("{:0>64}", clean)
}

/// Pad a u128 to 32 bytes hex (no 0x prefix).
pub fn pad_u256(val: u128) -> String {
    format!("{:0>64x}", val)
}

/// Encode an address[] dynamic array for ABI encoding.
/// Returns the raw hex (no 0x) for: length word + each element word.
pub fn encode_address_array(addrs: &[&str]) -> String {
    let mut out = String::new();
    // length
    out.push_str(&format!("{:0>64x}", addrs.len()));
    // elements
    for addr in addrs {
        out.push_str(&pad_address(addr));
    }
    out
}

/// Build ERC-20 approve calldata: approve(address spender, uint256 amount).
/// Selector: 0x095ea7b3
pub fn build_approve_calldata(spender: &str, amount: u128) -> String {
    format!("0x095ea7b3{}{}", pad_address(spender), pad_u256(amount))
}
