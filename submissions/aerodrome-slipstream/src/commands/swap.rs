use clap::Args;
use tokio::time::{sleep, Duration};
use crate::config::{
    build_approve_calldata, factory_address, pad_address, pad_u256, quoter_v2_address,
    resolve_token_address, rpc_url, swap_router, unix_now, ALL_TICK_SPACINGS,
};
use crate::onchainos::{extract_tx_hash, resolve_wallet, wallet_contract_call};
use crate::rpc::{factory_get_pool, get_allowance, quoter_exact_input_single};

const CHAIN_ID: u64 = 8453;

#[derive(Args)]
pub struct SwapArgs {
    /// Input token (symbol or hex address, e.g. USDC, WETH, 0x...)
    #[arg(long)]
    pub token_in: String,
    /// Output token (symbol or hex address)
    #[arg(long)]
    pub token_out: String,
    /// Amount in (smallest token unit, e.g. 1000000 = 1 USDC)
    #[arg(long)]
    pub amount_in: u128,
    /// Slippage tolerance in percent (e.g. 0.5 = 0.5%)
    #[arg(long, default_value = "0.5")]
    pub slippage: f64,
    /// Tick spacing (1/50/100/200/2000). If omitted, auto-selects best.
    #[arg(long)]
    pub tick_spacing: Option<i32>,
    /// Transaction deadline in minutes from now
    #[arg(long, default_value = "20")]
    pub deadline_minutes: u64,
    /// Dry run — build calldata but do not broadcast
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run(args: SwapArgs) -> anyhow::Result<()> {
    let rpc = rpc_url();
    let token_in = resolve_token_address(&args.token_in);
    let token_out = resolve_token_address(&args.token_out);
    let factory = factory_address();
    let quoter = quoter_v2_address();
    let router = swap_router();

    // --- 1. Find best tick spacing via QuoterV2 ---
    let spacings_to_check: Vec<i32> = if let Some(ts) = args.tick_spacing {
        vec![ts]
    } else {
        ALL_TICK_SPACINGS.to_vec()
    };

    let mut best_amount_out: u128 = 0;
    let mut best_tick_spacing: i32 = 0;

    for ts in &spacings_to_check {
        let pool_addr = factory_get_pool(&token_in, &token_out, *ts, factory, rpc).await?;
        if pool_addr == "0x0000000000000000000000000000000000000000" {
            continue;
        }
        match quoter_exact_input_single(quoter, &token_in, &token_out, *ts, args.amount_in, rpc)
            .await
        {
            Ok(amount_out) if amount_out > best_amount_out => {
                best_amount_out = amount_out;
                best_tick_spacing = *ts;
            }
            _ => {}
        }
    }

    if best_amount_out == 0 {
        anyhow::bail!("No valid pool or quote found. Check token addresses and tick spacings.");
    }

    let slippage_factor = 1.0 - (args.slippage / 100.0);
    let amount_out_minimum = (best_amount_out as f64 * slippage_factor) as u128;

    println!(
        "Quote: tokenIn={} tokenOut={} amountIn={} tickSpacing={} amountOut={} amountOutMin={}",
        token_in, token_out, args.amount_in, best_tick_spacing, best_amount_out, amount_out_minimum
    );
    println!("Please confirm the swap above before proceeding. (Proceeding automatically in non-interactive mode)");

    // --- 2. Resolve recipient ---
    let recipient = if args.dry_run {
        "0x0000000000000000000000000000000000000000".to_string()
    } else {
        resolve_wallet(CHAIN_ID)?
    };

    // --- 3. Check allowance and approve if needed ---
    if !args.dry_run {
        let allowance = get_allowance(&token_in, &recipient, router, rpc).await?;
        if allowance < args.amount_in {
            println!("Approving {} for SwapRouter...", token_in);
            let approve_data = build_approve_calldata(router, u128::MAX);
            let approve_result =
                wallet_contract_call(CHAIN_ID, &token_in, &approve_data, true, false).await?;
            println!("Approve tx: {}", extract_tx_hash(&approve_result));
            // Wait 3s for approve nonce to clear before swap
            sleep(Duration::from_secs(3)).await;
        }
    }

    // --- 4. Build exactInputSingle calldata ---
    // Aerodrome Slipstream exactInputSingle((tokenIn, tokenOut, tickSpacing, recipient,
    //   deadline, amountIn, amountOutMinimum, sqrtPriceLimitX96))
    // Selector: 0xa026383e
    // Note: tickSpacing is int24 (not fee uint24 like Uniswap V3)
    let deadline = unix_now() + args.deadline_minutes * 60;
    let ts_hex = if best_tick_spacing >= 0 {
        format!("{:0>64x}", best_tick_spacing as u64)
    } else {
        format!(
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffff{:08x}",
            best_tick_spacing as u32
        )
    };
    let calldata = format!(
        "0xa026383e{}{}{}{}{}{}{}{}",
        pad_address(&token_in),
        pad_address(&token_out),
        ts_hex,
        pad_address(&recipient),
        pad_u256(deadline as u128),
        pad_u256(args.amount_in),
        pad_u256(amount_out_minimum),
        pad_u256(0), // sqrtPriceLimitX96 = 0 (no limit)
    );

    let result = wallet_contract_call(CHAIN_ID, router, &calldata, true, args.dry_run).await?;

    let tx_hash = extract_tx_hash(&result);
    println!(
        "{{\"ok\":true,\"txHash\":\"{}\",\"tokenIn\":\"{}\",\"tokenOut\":\"{}\",\"amountIn\":{},\"tickSpacing\":{},\"amountOutMin\":{}}}",
        tx_hash, token_in, token_out, args.amount_in, best_tick_spacing, amount_out_minimum
    );

    Ok(())
}
