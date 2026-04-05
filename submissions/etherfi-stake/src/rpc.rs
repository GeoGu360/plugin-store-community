#![allow(dead_code)]

/// ABI encoding helpers — hand-rolled to avoid heavy alloy/ethabi dependency.

/// Pad a hex address (with or without 0x) to a 32-byte (64 hex char) left-zero-padded word.
pub fn encode_address(addr: &str) -> String {
    let addr = addr.trim_start_matches("0x").trim_start_matches("0X");
    format!("{:0>64}", addr)
}

/// Encode a u128 as a 32-byte big-endian hex word (no 0x prefix).
pub fn encode_uint256_u128(val: u128) -> String {
    format!("{:064x}", val)
}

/// Encode a u64 as a 32-byte big-endian hex word (no 0x prefix).
pub fn encode_uint256_u64(val: u64) -> String {
    format!("{:064x}", val)
}

/// Encode a dynamic uint256[] array (length word + element words, no offset word, no selector).
pub fn encode_uint256_array(values: &[u128]) -> String {
    let mut out = encode_uint256_u128(values.len() as u128);
    for v in values {
        out.push_str(&encode_uint256_u128(*v));
    }
    out
}

/// Build calldata for `balanceOf(address)` — selector + single address param.
pub fn calldata_single_address(selector: &str, addr: &str) -> String {
    format!("0x{}{}", selector, encode_address(addr))
}

/// Build calldata for `approve(address spender, uint256 amount)`.
pub fn calldata_approve(spender: &str, amount: u128) -> String {
    format!(
        "0x095ea7b3{}{}",
        encode_address(spender),
        encode_uint256_u128(amount)
    )
}

/// Build calldata for `allowance(address owner, address spender)`.
pub fn calldata_allowance(owner: &str, spender: &str) -> String {
    format!(
        "0xdd62ed3e{}{}",
        encode_address(owner),
        encode_address(spender)
    )
}

/// Build calldata for `depositETHForWeETH(address _referral)` — DepositAdapter, payable.
pub fn calldata_deposit_eth_for_weeth(referral: &str) -> String {
    format!("0xef54591d{}", encode_address(referral))
}

/// Build calldata for `deposit()` — LiquidityPool, payable, no params.
pub fn calldata_deposit() -> String {
    "0xd0e30db0".to_string()
}

/// Build calldata for `requestWithdraw(address recipient, uint256 amount)` — LiquidityPool.
pub fn calldata_request_withdraw(recipient: &str, amount_wei: u128) -> String {
    format!(
        "0x397a1b28{}{}",
        encode_address(recipient),
        encode_uint256_u128(amount_wei)
    )
}

/// Build calldata for `wrap(uint256 _eETHAmount)` — weETH contract.
pub fn calldata_wrap(amount_wei: u128) -> String {
    format!("0xea598cb0{}", encode_uint256_u128(amount_wei))
}

/// Build calldata for `unwrap(uint256 _weETHAmount)` — weETH contract.
pub fn calldata_unwrap(amount_wei: u128) -> String {
    format!("0xde0e9a3e{}", encode_uint256_u128(amount_wei))
}

/// Build calldata for `claimWithdraw(uint256 tokenId)` — WithdrawRequestNFT.
pub fn calldata_claim_withdraw(token_id: u128) -> String {
    format!("0xb13acedd{}", encode_uint256_u128(token_id))
}

/// Build calldata for `batchClaimWithdraw(uint256[] tokenIds)` — WithdrawRequestNFT.
/// ABI layout: selector | offset(0x20) | length | tokenIds...
pub fn calldata_batch_claim_withdraw(token_ids: &[u128]) -> String {
    let offset = encode_uint256_u128(0x20);
    let arr = encode_uint256_array(token_ids);
    format!("0x24fccdcf{}{}", offset, arr)
}

/// Build calldata for `getRequest(uint256 tokenId)` — WithdrawRequestNFT.
pub fn calldata_get_request(token_id: u128) -> String {
    format!("0xc58343ef{}", encode_uint256_u128(token_id))
}

/// Build calldata for `isFinalized(uint256 tokenId)` — WithdrawRequestNFT.
pub fn calldata_is_finalized(token_id: u128) -> String {
    format!("0x33727c4d{}", encode_uint256_u128(token_id))
}

/// Build calldata for `getClaimableAmount(uint256 tokenId)` — WithdrawRequestNFT.
pub fn calldata_get_claimable_amount(token_id: u128) -> String {
    format!("0x7d8ca242{}", encode_uint256_u128(token_id))
}

/// Build calldata for `getRate()` — weETH contract.
pub fn calldata_get_rate() -> String {
    "0x679aefce".to_string()
}

/// Build calldata for `getEETHByWeETH(uint256)` — weETH contract.
pub fn calldata_get_eeth_by_weeth(weeth_amount: u128) -> String {
    format!("0x94626044{}", encode_uint256_u128(weeth_amount))
}

/// Build calldata for `getWeETHByeETH(uint256)` — weETH contract.
pub fn calldata_get_weeth_by_eeth(eeth_amount: u128) -> String {
    format!("0xd044fe9b{}", encode_uint256_u128(eeth_amount))
}

/// Decode a single uint256 from ABI-encoded return data (32-byte hex, optional 0x prefix).
pub fn decode_uint256(hex: &str) -> anyhow::Result<u128> {
    let hex = hex.trim().trim_start_matches("0x");
    if hex.len() < 64 {
        anyhow::bail!("Return data too short for uint256: '{}'", hex);
    }
    let word = &hex[hex.len() - 64..];
    Ok(u128::from_str_radix(word, 16)?)
}

/// Decode a bool (uint256 where 0=false, non-zero=true).
pub fn decode_bool(hex: &str) -> anyhow::Result<bool> {
    Ok(decode_uint256(hex)? != 0)
}

/// Extract the raw hex return value from an eth_call response envelope.
pub fn extract_return_data(result: &serde_json::Value) -> anyhow::Result<String> {
    if let Some(s) = result["data"]["result"].as_str() {
        return Ok(s.to_string());
    }
    if let Some(s) = result["data"]["returnData"].as_str() {
        return Ok(s.to_string());
    }
    if let Some(s) = result["result"].as_str() {
        return Ok(s.to_string());
    }
    anyhow::bail!("Could not extract return data from: {}", result)
}

/// Format a wei u128 as a human-readable ETH string (6 decimal places).
pub fn format_eth(wei: u128) -> String {
    let eth = wei as f64 / 1e18;
    format!("{:.6}", eth)
}
