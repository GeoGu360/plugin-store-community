use crate::api;
use crate::config::get_chain_config;
use crate::onchainos::resolve_wallet;
use anyhow::Result;

pub async fn run(chain_id: u64, account: Option<String>) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;

    let addr = match account {
        Some(a) => a,
        None => {
            let w = resolve_wallet(chain_id)?;
            if w.is_empty() {
                anyhow::bail!("Cannot get wallet address. Ensure onchainos is logged in.");
            }
            w
        }
    };

    println!("Fetching positions for: {}", addr);
    let data = api::get_positions(cfg.api_base_url, &addr).await?;

    let positions = data.as_array().cloned().unwrap_or_default();
    if positions.is_empty() {
        println!("No open positions found.");
        return Ok(());
    }

    println!(
        "\n{:<42} {:<8} {:<18} {:<18} {:<14}",
        "Market", "Dir", "Size (USD)", "Collateral", "PnL"
    );
    println!("{}", "-".repeat(105));

    for p in &positions {
        let market = p["marketAddress"].as_str().unwrap_or("-");
        let is_long = p["isLong"].as_bool().unwrap_or(false);
        let direction = if is_long { "LONG" } else { "SHORT" };
        let size_raw = p["sizeInUsd"].as_str().unwrap_or("0");
        let collateral_raw = p["collateralAmount"].as_str().unwrap_or("0");
        let pnl_raw = p["pnl"].as_str().unwrap_or("0");

        // sizeInUsd is in 30-decimal format
        let size: f64 = size_raw.parse::<u128>().unwrap_or(0) as f64 / 1e30;
        // collateral is in token units (varies), display raw
        let pnl: f64 = pnl_raw.parse::<i128>().unwrap_or(0) as f64 / 1e30;

        println!(
            "{:<42} {:<8} {:<18.2} {:<18} {:<14.4}",
            market, direction, size, collateral_raw, pnl
        );
    }
    println!("\nTotal positions: {}", positions.len());
    Ok(())
}
