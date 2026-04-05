pub struct ChainConfig {
    pub exchange_router: &'static str,
    pub router: &'static str,
    pub order_vault: &'static str,
    pub deposit_vault: &'static str,
    pub withdrawal_vault: &'static str,
    pub api_base_url: &'static str,
}

pub const ARBITRUM: ChainConfig = ChainConfig {
    exchange_router: "0x1C3fa76e6E1088bCE750f23a5BFcffa1efEF6A41",
    router: "0x7452c558d45f8afC8c83dAe62C3f8A5BE19c71f6",
    order_vault: "0x31eF83a530Fde1B38EE9A18093A333D8Bbbc40D5",
    deposit_vault: "0xF89e77e8Dc11691C9e8757e84aaFbCD8A67d7A55",
    withdrawal_vault: "0x0628D46b5D145f183AdB6Ef1f2c97eD1C4701C55",
    api_base_url: "https://arbitrum-api.gmxinfra.io",
};

pub const AVALANCHE: ChainConfig = ChainConfig {
    exchange_router: "0x8f550E53DFe96C055D5Bdb267c21F268fCAF63B2",
    router: "0x820F5FfC5b525cD4d88Cd91aCf2c28F16530Cc68",
    order_vault: "0xD3D60D22d415aD43b7e64b510D86A30f19B1B12C",
    deposit_vault: "0x90c670825d0C62ede1c5ee9571d6d9a17A722DFF",
    withdrawal_vault: "0xf5F30B10141E1F63FC11eD772931A8294a591996",
    api_base_url: "https://avalanche-api.gmxinfra.io",
};

pub fn get_chain_config(chain_id: u64) -> anyhow::Result<&'static ChainConfig> {
    match chain_id {
        42161 => Ok(&ARBITRUM),
        43114 => Ok(&AVALANCHE),
        _ => anyhow::bail!(
            "Unsupported chain ID: {}. Use 42161 (Arbitrum) or 43114 (Avalanche)",
            chain_id
        ),
    }
}
