use crate::{config, onchainos, rpc};

pub async fn run() -> anyhow::Result<()> {
    println!("=== EigenLayer Available Strategies (Ethereum Mainnet) ===");
    println!();

    for meta in config::STRATEGIES {
        // Read totalShares()
        let total_shares_calldata = format!("0x{}", config::SEL_TOTAL_SHARES);
        let total_shares_result =
            onchainos::eth_call(config::CHAIN_ID, meta.address, &total_shares_calldata);

        let total_shares = if let Ok(result) = total_shares_result {
            if let Ok(data) = rpc::extract_return_data(&result) {
                rpc::decode_uint256(&data).unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };

        // totalShares is in wei (1e18 scale)
        let total_eth_equiv = total_shares as f64 / 1e18;

        println!("  Symbol:    {}", meta.symbol);
        println!("  Strategy:  {}", meta.address);
        println!("  Token:     {}", meta.token);
        println!("  TVL:       {:.4} {} equivalent", total_eth_equiv, meta.symbol);
        println!();
    }

    println!("To stake into a strategy, use: eigencloud stake --symbol <stETH|rETH|cbETH> --amount <AMOUNT>");
    Ok(())
}
