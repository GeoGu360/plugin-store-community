use crate::api;
use crate::config::get_chain_config;
use anyhow::Result;

pub async fn run(chain_id: u64) -> Result<()> {
    let cfg = get_chain_config(chain_id)?;
    let data = api::get_markets(cfg.api_base_url).await?;

    let markets = data["markets"]
        .as_array()
        .cloned()
        .unwrap_or_else(|| data.as_array().cloned().unwrap_or_default());

    if markets.is_empty() {
        println!("No markets found.");
        return Ok(());
    }

    println!("{:<50} {:<12} {:<42}", "Market Name", "Listed", "Market Token");
    println!("{}", "-".repeat(108));
    for m in &markets {
        let name = m["name"].as_str().unwrap_or("Unknown");
        let listed = m["isListed"].as_bool().unwrap_or(false);
        let token = m["marketToken"].as_str().unwrap_or("-");
        println!("{:<50} {:<12} {:<42}", name, listed, token);
    }
    println!("\nTotal: {} markets", markets.len());
    Ok(())
}
