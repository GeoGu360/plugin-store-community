use clap::Args;
use crate::config::{factory_address, resolve_token_address, rpc_url, ALL_TICK_SPACINGS};
use crate::rpc::factory_get_pool;

#[derive(Args)]
pub struct GetPoolsArgs {
    /// Token 0 (symbol or hex address, e.g. WETH, USDC, 0x...)
    #[arg(long)]
    pub token0: String,
    /// Token 1 (symbol or hex address)
    #[arg(long)]
    pub token1: String,
    /// Tick spacing (1/50/100/200/2000). If omitted, queries all.
    #[arg(long)]
    pub tick_spacing: Option<i32>,
}

pub async fn run(args: GetPoolsArgs) -> anyhow::Result<()> {
    let rpc = rpc_url();
    let token0 = resolve_token_address(&args.token0);
    let token1 = resolve_token_address(&args.token1);
    let factory = factory_address();

    let spacings_to_check: Vec<i32> = if let Some(ts) = args.tick_spacing {
        vec![ts]
    } else {
        ALL_TICK_SPACINGS.to_vec()
    };

    let mut pools = Vec::new();

    for ts in spacings_to_check {
        let pool_addr = factory_get_pool(&token0, &token1, ts, factory, rpc).await?;
        let deployed = pool_addr != "0x0000000000000000000000000000000000000000";
        println!(
            "  tickSpacing={}: {} {}",
            ts,
            pool_addr,
            if deployed { "(deployed)" } else { "(not deployed)" }
        );
        pools.push(serde_json::json!({
            "tickSpacing": ts,
            "address": pool_addr,
            "deployed": deployed,
        }));
    }

    println!(
        "{{\"ok\":true,\"token0\":\"{}\",\"token1\":\"{}\",\"pools\":{}}}",
        token0,
        token1,
        serde_json::to_string(&pools)?
    );

    Ok(())
}
