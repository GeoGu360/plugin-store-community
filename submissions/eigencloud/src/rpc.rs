#![allow(dead_code)]

/// ABI encoding helpers — hand-rolled to avoid heavy alloy dependency

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

/// Build calldata for a single-address-param call: selector + address word.
pub fn calldata_single_address(selector: &str, addr: &str) -> String {
    format!("0x{}{}", selector, encode_address(addr))
}

/// Build calldata for a two-address-param call: selector + addr1 + addr2.
pub fn calldata_two_addresses(selector: &str, addr1: &str, addr2: &str) -> String {
    format!("0x{}{}{}", selector, encode_address(addr1), encode_address(addr2))
}

/// Build calldata for `approve(address spender, uint256 amount)`.
pub fn calldata_approve(spender: &str, amount: u128) -> String {
    format!(
        "0x095ea7b3{}{}",
        encode_address(spender),
        encode_uint256_u128(amount)
    )
}

/// Build calldata for `depositIntoStrategy(address strategy, address token, uint256 amount)`.
/// Selector: 0xe7a050aa
pub fn calldata_deposit_into_strategy(strategy: &str, token: &str, amount: u128) -> String {
    format!(
        "0xe7a050aa{}{}{}",
        encode_address(strategy),
        encode_address(token),
        encode_uint256_u128(amount)
    )
}

/// Build calldata for `delegateTo(address operator, (bytes,uint256) approverSig, bytes32 salt)`.
/// Selector: 0xeea9064b
///
/// For open operators (no approver), use empty bytes sig and zero expiry/salt.
/// ABI layout:
///   [0x00] operator (address, padded)
///   [0x20] offset to struct (= 0x60)
///   [0x40] salt (bytes32)
///   [0x60] offset to bytes sig within struct (= 0x40)
///   [0x80] expiry (uint256) = 0
///   [0xa0] bytes length = 0
pub fn calldata_delegate_to(operator: &str) -> String {
    let operator_word = encode_address(operator);
    // Struct offset = 0x60 (3 words from calldata start after selector)
    let struct_offset = encode_uint256_u128(0x60);
    // salt = bytes32(0)
    let salt = "0".repeat(64);
    // Within the struct: bytes sig offset = 0x40 (2 words), expiry = 0
    let sig_offset = encode_uint256_u128(0x40);
    let expiry = encode_uint256_u128(0);
    // bytes sig: length = 0 (no data)
    let sig_len = encode_uint256_u128(0);

    format!(
        "0xeea9064b{}{}{}{}{}{}",
        operator_word, struct_offset, salt, sig_offset, expiry, sig_len
    )
}

/// Build calldata for `undelegate(address staker)`.
/// Selector: 0xda8be864
pub fn calldata_undelegate(staker: &str) -> String {
    format!("0xda8be864{}", encode_address(staker))
}

/// Build calldata for `queueWithdrawals((address[],uint256[],address)[] params)`.
/// Selector: 0x0dd8dd02
///
/// Single-element params array with one strategy and one share amount.
/// ABI layout for queueWithdrawals([(strategies[], shares[], withdrawer)]):
///   This is a complex dynamic type; we encode for a single withdrawal of one strategy.
///
/// Top-level: tuple array with 1 element
/// struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
///
/// Layout:
///   [0x00] offset to array = 0x20
///   [0x20] array length = 1
///   [0x40] offset to element[0] relative to array data start = 0x20
///   [0x60] offset to strategies[] within struct = 0x60
///   [0x80] offset to shares[] within struct = 0xa0 (= 0x60 + 0x20 + 0x20)
///   [0xa0] withdrawer address
///   [0xc0] strategies[] length = 1
///   [0xe0] strategy address
///   [0x100] shares[] length = 1
///   [0x120] share amount
pub fn calldata_queue_withdrawal(strategy: &str, shares: u128, withdrawer: &str) -> String {
    // Top-level dynamic array offset = 0x20
    let arr_offset = encode_uint256_u128(0x20);
    let arr_len = encode_uint256_u128(1);
    // Offset to element[0] from the start of array content = 0x20
    let elem_offset = encode_uint256_u128(0x20);

    // Within the struct:
    // strategies[] is at offset 0x60 from struct start
    // shares[] is at offset 0xa0 from struct start  (= 0x60 + 0x20 (len) + 0x20 (elem))
    // withdrawer is at struct[0x40]
    let strats_offset = encode_uint256_u128(0x60);
    let shares_offset = encode_uint256_u128(0xa0);
    let withdrawer_word = encode_address(withdrawer);

    // strategies[]: length=1, element=strategy
    let strats_len = encode_uint256_u128(1);
    let strategy_word = encode_address(strategy);

    // shares[]: length=1, element=shares
    let shares_len = encode_uint256_u128(1);
    let shares_word = encode_uint256_u128(shares);

    format!(
        "0x0dd8dd02{}{}{}{}{}{}{}{}{}{}",
        arr_offset,
        arr_len,
        elem_offset,
        strats_offset,
        shares_offset,
        withdrawer_word,
        strats_len,
        strategy_word,
        shares_len,
        shares_word
    )
}

/// Decode a single uint256 from ABI-encoded return data (32-byte hex string, optional 0x prefix).
pub fn decode_uint256(hex: &str) -> anyhow::Result<u128> {
    let hex = hex.trim().trim_start_matches("0x");
    if hex.len() < 64 {
        anyhow::bail!("Return data too short for uint256: '{}'", hex);
    }
    let word = &hex[hex.len() - 64..];
    Ok(u128::from_str_radix(word, 16)?)
}

/// Decode a bool (uint256 = 0 or 1) from ABI return data.
pub fn decode_bool(hex: &str) -> bool {
    decode_uint256(hex).unwrap_or(0) != 0
}

/// Decode an address from ABI-encoded return data (last 40 hex chars of a 64-char word).
pub fn decode_address(hex: &str) -> String {
    let hex = hex.trim().trim_start_matches("0x");
    if hex.len() < 64 {
        return "0x0000000000000000000000000000000000000000".to_string();
    }
    // Address is in the last 40 chars of the first 64-char word
    let word = &hex[..64];
    format!("0x{}", &word[24..])
}

/// Decode a (address[], uint256[]) tuple returned by getDeposits(address).
/// ABI layout: offset_arr1(0x40) | offset_arr2 | len1 | addr1..N | len2 | uint256_1..N
pub fn decode_deposits(hex: &str) -> Vec<(String, u128)> {
    let hex = hex.trim().trim_start_matches("0x");
    if hex.len() < 128 {
        return vec![];
    }
    // Read offset to addresses array (first word)
    let addr_offset_hex = &hex[0..64];
    let addr_offset = usize::from_str_radix(addr_offset_hex, 16).unwrap_or(0x40) * 2;

    if addr_offset + 64 > hex.len() {
        return vec![];
    }
    let addr_len = usize::from_str_radix(&hex[addr_offset..addr_offset + 64], 16).unwrap_or(0);

    let mut strategies = Vec::new();
    for i in 0..addr_len {
        let start = addr_offset + 64 + i * 64;
        if start + 64 > hex.len() {
            break;
        }
        let addr_word = &hex[start..start + 64];
        strategies.push(format!("0x{}", &addr_word[24..]));
    }

    // Read offset to shares array (second word)
    let shares_offset_hex = &hex[64..128];
    let shares_offset = usize::from_str_radix(shares_offset_hex, 16).unwrap_or(0) * 2;

    if shares_offset + 64 > hex.len() {
        return strategies.iter().map(|s| (s.clone(), 0u128)).collect();
    }
    let shares_len = usize::from_str_radix(&hex[shares_offset..shares_offset + 64], 16).unwrap_or(0);

    let mut shares_vec = Vec::new();
    for i in 0..shares_len {
        let start = shares_offset + 64 + i * 64;
        if start + 64 > hex.len() {
            break;
        }
        let word = &hex[start..start + 64];
        shares_vec.push(u128::from_str_radix(word, 16).unwrap_or(0));
    }

    strategies
        .into_iter()
        .zip(shares_vec.into_iter().chain(std::iter::repeat(0u128)))
        .collect()
}

/// Extract the raw hex return value from an onchainos/eth_call response.
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
