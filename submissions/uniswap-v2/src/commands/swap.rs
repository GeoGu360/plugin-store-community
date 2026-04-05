use tokio::time::{sleep, Duration};

use crate::config::{
    resolve_token_address, is_native_eth, apply_slippage, deadline,
    pad_address, pad_u256, encode_address_array, build_approve_calldata,
    ROUTER_V2, WETH, ETH_RPC, CHAIN_ID,
};
use crate::onchainos::{resolve_wallet, wallet_contract_call, extract_tx_hash};
use crate::rpc::{get_amounts_out, get_allowance};

/// Swap tokens on Uniswap V2.
///
/// Handles three variants:
///   1. ETH → token  (swapExactETHForTokens, payable, no approve needed)
///   2. token → ETH  (swapExactTokensForETH)
///   3. token → token (swapExactTokensForTokens, routes via WETH)
pub async fn run(
    token_in: &str,
    token_out: &str,
    amount_in: u128,
    dry_run: bool,
) -> anyhow::Result<()> {
    let chain_id = CHAIN_ID;
    let rpc      = ETH_RPC;
    let router   = ROUTER_V2;

    let in_is_eth  = is_native_eth(token_in);
    let out_is_eth = is_native_eth(token_out);

    let addr_in  = resolve_token_address(token_in,  chain_id);
    let addr_out = resolve_token_address(token_out, chain_id);

    // Dry-run uses zero address to avoid onchainos wallet call
    let recipient = if dry_run {
        "0x0000000000000000000000000000000000000000".to_string()
    } else {
        resolve_wallet(chain_id)?
    };
    let dl = deadline();

    if in_is_eth {
        // ---------------------------------------------------------------
        // Variant 1: ETH → token  (swapExactETHForTokens)
        // Selector: 0x7ff36ab5
        // Params: (uint256 amountOutMin, address[] path, address to, uint256 deadline)
        // path starts with WETH
        // ---------------------------------------------------------------
        let path = [WETH, addr_out.as_str()];
        let amounts = get_amounts_out(router, amount_in, &path, rpc).await?;
        let amount_out_min = apply_slippage(*amounts.last().unwrap_or(&0));

        // ABI layout:
        //   word 0: amountOutMin
        //   word 1: offset to path = 0x80 (4 fixed words × 32 = 128 = 0x80)
        //   word 2: to
        //   word 3: deadline
        //   word 4+: path array (length + elements)
        let calldata = format!(
            "0x7ff36ab5{}{}{}{}{}",
            pad_u256(amount_out_min),
            pad_u256(0x80),
            pad_address(&recipient),
            pad_u256(dl as u128),
            encode_address_array(&path)
        );

        println!("Swap: {} wei ETH → {} (swapExactETHForTokens)", amount_in, token_out.to_uppercase());
        println!("  amountOutMin: {}", amount_out_min);
        println!("  to:           {}", recipient);
        println!("  deadline:     {}", dl);

        let result = wallet_contract_call(
            chain_id, router, &calldata, None, Some(amount_in), dry_run,
        ).await?;
        println!("  txHash: {}", extract_tx_hash(&result));

    } else if out_is_eth {
        // ---------------------------------------------------------------
        // Variant 2: token → ETH  (swapExactTokensForETH)
        // Selector: 0x18cbafe5
        // Params: (uint256 amountIn, uint256 amountOutMin, address[] path, address to, uint256 deadline)
        // path ends with WETH
        // ---------------------------------------------------------------
        let path = [addr_in.as_str(), WETH];
        let amounts = get_amounts_out(router, amount_in, &path, rpc).await?;
        let amount_out_min = apply_slippage(*amounts.last().unwrap_or(&0));

        // Approve if needed
        if !dry_run {
            let allowance = get_allowance(&addr_in, &recipient, router, rpc).await?;
            if allowance < amount_in {
                println!("  Approving {} for Router...", token_in.to_uppercase());
                let approve_cd = build_approve_calldata(router, u128::MAX);
                let ar = wallet_contract_call(chain_id, &addr_in, &approve_cd, None, None, false).await?;
                println!("  approve txHash: {}", extract_tx_hash(&ar));
                sleep(Duration::from_secs(3)).await;
            }
        }

        // ABI layout:
        //   word 0: amountIn
        //   word 1: amountOutMin
        //   word 2: offset to path = 0xa0 (5 fixed words × 32 = 160 = 0xa0)
        //   word 3: to
        //   word 4: deadline
        //   word 5+: path array (length + elements)
        let calldata = format!(
            "0x18cbafe5{}{}{}{}{}{}",
            pad_u256(amount_in),
            pad_u256(amount_out_min),
            pad_u256(0xa0),
            pad_address(&recipient),
            pad_u256(dl as u128),
            encode_address_array(&path)
        );

        println!("Swap: {} (token → ETH, swapExactTokensForETH)", token_in.to_uppercase());
        println!("  amountIn:     {}", amount_in);
        println!("  amountOutMin: {} wei ETH", amount_out_min);
        println!("  to:           {}", recipient);

        let result = wallet_contract_call(
            chain_id, router, &calldata, None, None, dry_run,
        ).await?;
        println!("  txHash: {}", extract_tx_hash(&result));

    } else {
        // ---------------------------------------------------------------
        // Variant 3: token → token  (swapExactTokensForTokens)
        // Selector: 0x38ed1739
        // Params: (uint256 amountIn, uint256 amountOutMin, address[] path, address to, uint256 deadline)
        // Route via WETH for maximum liquidity unless one side is already WETH
        // ---------------------------------------------------------------
        let weth_lower = WETH.to_lowercase();
        let ai_lower   = addr_in.to_lowercase();
        let ao_lower   = addr_out.to_lowercase();

        let (path_vec, path_desc): (Vec<String>, String) =
            if ai_lower == weth_lower || ao_lower == weth_lower {
                // Direct path — one side is WETH
                (
                    vec![addr_in.clone(), addr_out.clone()],
                    format!("{} → {}", token_in.to_uppercase(), token_out.to_uppercase()),
                )
            } else {
                // Route via WETH
                (
                    vec![addr_in.clone(), WETH.to_string(), addr_out.clone()],
                    format!("{} → WETH → {}", token_in.to_uppercase(), token_out.to_uppercase()),
                )
            };

        let path: Vec<&str> = path_vec.iter().map(|s| s.as_str()).collect();
        let amounts = get_amounts_out(router, amount_in, &path, rpc).await?;
        let amount_out_min = apply_slippage(*amounts.last().unwrap_or(&0));

        // Approve if needed
        if !dry_run {
            let allowance = get_allowance(&addr_in, &recipient, router, rpc).await?;
            if allowance < amount_in {
                println!("  Approving {} for Router...", token_in.to_uppercase());
                let approve_cd = build_approve_calldata(router, u128::MAX);
                let ar = wallet_contract_call(chain_id, &addr_in, &approve_cd, None, None, false).await?;
                println!("  approve txHash: {}", extract_tx_hash(&ar));
                sleep(Duration::from_secs(3)).await;
            }
        }

        // ABI layout:
        //   word 0: amountIn
        //   word 1: amountOutMin
        //   word 2: offset to path = 0xa0 (5 fixed words × 32 = 160 = 0xa0)
        //   word 3: to
        //   word 4: deadline
        //   word 5+: path array (length + elements)
        let calldata = format!(
            "0x38ed1739{}{}{}{}{}{}",
            pad_u256(amount_in),
            pad_u256(amount_out_min),
            pad_u256(0xa0),
            pad_address(&recipient),
            pad_u256(dl as u128),
            encode_address_array(&path)
        );

        println!("Swap: {} (token → token, swapExactTokensForTokens)", path_desc);
        println!("  amountIn:     {}", amount_in);
        println!("  amountOutMin: {}", amount_out_min);
        println!("  to:           {}", recipient);

        let result = wallet_contract_call(
            chain_id, router, &calldata, None, None, dry_run,
        ).await?;
        println!("  txHash: {}", extract_tx_hash(&result));
    }

    Ok(())
}
