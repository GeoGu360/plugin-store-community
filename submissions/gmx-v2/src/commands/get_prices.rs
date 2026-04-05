use crate::api;
use crate::config::get_chain_config;
use anyhow::Result;

pub async fn run(chain_id: u64) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;
    let data = api::get_prices(cfg.api_base_url).await?;

    let tickers = data.as_array().cloned().unwrap_or_default();
    if tickers.is_empty() {
        println!("No price data available.");
        return Ok(());
    }

    println!(
        "{:<12} {:<20} {:<20} {}",
        "Symbol", "Min Price (USD)", "Max Price (USD)", "Token Address"
    );
    println!("{}", "-".repeat(100));

    for t in &tickers {
        let symbol = t["tokenSymbol"].as_str().unwrap_or("?");
        let min_raw = t["minPrice"].as_str().unwrap_or("0");
        let max_raw = t["maxPrice"].as_str().unwrap_or("0");
        let addr = t["tokenAddress"].as_str().unwrap_or("-");

        // Use 18 decimals as default for display (most tokens); stablecoins (~6 dec)
        // will show larger numbers but we avoid needing the tokens list here.
        // For proper display the caller can cross-reference with /tokens.
        let min_human = parse_30dec_price(min_raw);
        let max_human = parse_30dec_price(max_raw);

        println!(
            "{:<12} {:<20.6} {:<20.6} {}",
            symbol, min_human, max_human, addr
        );
    }
    println!("\nNote: Prices shown with 18-decimal token assumption. Stablecoin prices will appear as ~1e-12; multiply by 10^12 for USD.");
    Ok(())
}

/// Parse a GMX 30-decimal price assuming 18-decimal token.
/// human_price = raw / 10^(30 - 18) = raw / 10^12
fn parse_30dec_price(raw: &str) -> f64 {
    let v: u128 = raw.parse().unwrap_or(0);
    v as f64 / 1e12
}
