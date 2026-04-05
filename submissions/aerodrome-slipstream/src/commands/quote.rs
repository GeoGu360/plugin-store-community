use clap::Args;
use crate::config::{factory_address, quoter_v2_address, resolve_token_address, rpc_url, ALL_TICK_SPACINGS};
use crate::rpc::{factory_get_pool, quoter_exact_input_single};

#[derive(Args)]
pub struct QuoteArgs {
    /// Input token (symbol or hex address, e.g. USDC, WETH, 0x...)
    #[arg(long)]
    pub token_in: String,
    /// Output token (symbol or hex address)
    #[arg(long)]
    pub token_out: String,
    /// Amount in (in token smallest unit, e.g. 1000000 = 1 USDC)
    #[arg(long)]
    pub amount_in: u128,
    /// Tick spacing (1/50/100/200/2000). If omitted, queries all and returns best.
    #[arg(long)]
    pub tick_spacing: Option<i32>,
}

pub async fn run(args: QuoteArgs) -> anyhow::Result<()> {
    let rpc = rpc_url();
    let token_in = resolve_token_address(&args.token_in);
    let token_out = resolve_token_address(&args.token_out);
    let factory = factory_address();
    let quoter = quoter_v2_address();

    let spacings_to_check: Vec<i32> = if let Some(ts) = args.tick_spacing {
        vec![ts]
    } else {
        ALL_TICK_SPACINGS.to_vec()
    };

    let mut best_amount_out: u128 = 0;
    let mut best_tick_spacing: i32 = 0;

    for ts in spacings_to_check {
        // Validate pool exists before quoting (avoids 0-liquidity false quotes)
        let pool_addr = factory_get_pool(&token_in, &token_out, ts, factory, rpc).await?;
        if pool_addr == "0x0000000000000000000000000000000000000000" {
            println!("  tickSpacing={}: pool not deployed, skipping", ts);
            continue;
        }

        match quoter_exact_input_single(quoter, &token_in, &token_out, ts, args.amount_in, rpc)
            .await
        {
            Ok(amount_out) => {
                println!("  tickSpacing={}: amountOut={}", ts, amount_out);
                if amount_out > best_amount_out {
                    best_amount_out = amount_out;
                    best_tick_spacing = ts;
                }
            }
            Err(e) => {
                println!("  tickSpacing={}: quote failed: {}", ts, e);
            }
        }
    }

    if best_amount_out == 0 {
        println!("{{\"ok\":false,\"error\":\"No valid quote found for any tick spacing\"}}");
    } else {
        println!(
            "{{\"ok\":true,\"tokenIn\":\"{}\",\"tokenOut\":\"{}\",\"amountIn\":{},\"bestTickSpacing\":{},\"amountOut\":{}}}",
            token_in, token_out, args.amount_in, best_tick_spacing, best_amount_out
        );
    }

    Ok(())
}
