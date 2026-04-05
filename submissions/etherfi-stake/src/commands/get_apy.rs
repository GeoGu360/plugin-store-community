use crate::config;

pub async fn run() -> anyhow::Result<()> {
    // Fetch the latest data point for ether.fi weETH from DefiLlama chart endpoint.
    // This is a read-only HTTP GET — no onchainos command required.
    let url = format!(
        "{}/chart/{}",
        config::DEFILLAMA_YIELDS_URL,
        config::DEFILLAMA_POOL_ID
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "etherfi-stake-plugin/0.1.0")
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Failed to fetch APY from DefiLlama: HTTP {}", resp.status());
    }

    let body: serde_json::Value = resp.json().await?;

    // body.data is an array of data points sorted by timestamp; take the last (latest).
    let data_arr = body["data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Unexpected DefiLlama response shape: {}", body))?;

    if data_arr.is_empty() {
        anyhow::bail!("DefiLlama returned empty data array for pool {}", config::DEFILLAMA_POOL_ID);
    }

    let latest = data_arr.last().unwrap();
    let apy = latest["apy"].as_f64();
    let apy_base_7d = latest["apyBase7d"].as_f64();
    let tvl_usd = latest["tvlUsd"].as_f64();
    let timestamp = latest["timestamp"].as_str().unwrap_or("unknown");

    println!("=== ether.fi weETH APY ===");
    match apy {
        Some(v) => println!("Current APY:        {:.3}%", v),
        None => println!("Current APY:        (unavailable)"),
    }
    match apy_base_7d {
        Some(v) => println!("7-day base APY:     {:.3}%", v),
        None => {}
    }
    match tvl_usd {
        Some(v) => println!("TVL:                ${:.0}", v),
        None => {}
    }
    println!("As of:              {}", timestamp);
    println!();
    println!("Source: DefiLlama pool {}", config::DEFILLAMA_POOL_ID);
    println!("Note: APY includes restaking rewards (EigenLayer / Symbiotic points may vary).");

    Ok(())
}
