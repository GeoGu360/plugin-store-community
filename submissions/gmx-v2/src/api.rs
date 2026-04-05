use anyhow::Result;
use serde_json::Value;

#[allow(dead_code)]
pub async fn get_markets(api_base: &str) -> Result<Value> {
    let url = format!("{}/markets", api_base);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(resp)
}

#[allow(dead_code)]
pub async fn get_markets_info(api_base: &str) -> Result<Value> {
    let url = format!("{}/markets/info", api_base);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(resp)
}

pub async fn get_prices(api_base: &str) -> Result<Value> {
    let url = format!("{}/prices/tickers", api_base);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(resp)
}

pub async fn get_positions(api_base: &str, account: &str) -> Result<Value> {
    let url = format!("{}/positions?account={}", api_base, account);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(resp)
}

#[allow(dead_code)]
pub async fn get_tokens(api_base: &str) -> Result<Value> {
    let url = format!("{}/tokens", api_base);
    let resp = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(resp)
}

#[allow(dead_code)]
/// Convert a GMX 30-decimal price to a human-readable USD price.
/// GMX stores prices as: price_usd * 10^(30) / 10^(token_decimals)
/// So: human_price = raw_price / 10^(30 - token_decimals)
pub fn price_to_human(raw_price_str: &str, token_decimals: u32) -> f64 {
    let raw: u128 = raw_price_str.parse().unwrap_or(0);
    let divisor = 10u128.pow(30 - token_decimals);
    raw as f64 / divisor as f64
}
