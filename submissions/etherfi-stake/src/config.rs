#![allow(dead_code)]

/// Ethereum mainnet chain ID
pub const CHAIN_ID: u64 = 1;

/// LiquidityPool proxy — ETH deposit + withdrawal requests
pub const LIQUIDITY_POOL_ADDRESS: &str = "0x308861A430be4cce5502d0A12724771Fc6DaF216";

/// eETH token proxy (rebasing)
pub const EETH_ADDRESS: &str = "0x35fA164735182de50811E8e2E824cFb9B6118ac2";

/// weETH token proxy (non-rebasing wrapped)
pub const WEETH_ADDRESS: &str = "0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee";

/// WithdrawRequestNFT proxy (ERC-721, represents pending withdrawal)
pub const WITHDRAW_REQUEST_NFT_ADDRESS: &str = "0x7d5706f6ef3F89B3951E23e557CDFBC3239D4E2c";

/// DepositAdapter — stake ETH and receive weETH in one transaction
pub const DEPOSIT_ADAPTER_ADDRESS: &str = "0xcfc6d9bd7411962bfe7145451a7ef71a24b6a7a2";

/// Zero address used as default referral
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/// Ethereum JSON-RPC endpoint for read-only eth_call
pub const RPC_URL: &str = "https://ethereum.publicnode.com";

/// DefiLlama yields API base URL
pub const DEFILLAMA_YIELDS_URL: &str = "https://yields.llama.fi";

/// DefiLlama pool ID for ether.fi weETH on Ethereum
pub const DEFILLAMA_POOL_ID: &str = "46bd2bdf-6d92-4066-b482-e885ee172264";

// ─── Function selectors ────────────────────────────────────────────────────

// DepositAdapter
pub const SEL_DEPOSIT_ETH_FOR_WEETH: &str = "ef54591d"; // depositETHForWeETH(address)

// LiquidityPool
pub const SEL_DEPOSIT: &str = "d0e30db0"; // deposit()
pub const SEL_REQUEST_WITHDRAW: &str = "397a1b28"; // requestWithdraw(address,uint256)

// weETH
pub const SEL_WRAP: &str = "ea598cb0"; // wrap(uint256)
pub const SEL_UNWRAP: &str = "de0e9a3e"; // unwrap(uint256)
pub const SEL_GET_RATE: &str = "679aefce"; // getRate()
pub const SEL_GET_EETH_BY_WEETH: &str = "94626044"; // getEETHByWeETH(uint256)
pub const SEL_GET_WEETH_BY_EETH: &str = "d044fe9b"; // getWeETHByeETH(uint256)

// eETH / ERC-20 common
pub const SEL_BALANCE_OF: &str = "70a08231"; // balanceOf(address)
pub const SEL_APPROVE: &str = "095ea7b3"; // approve(address,uint256)
pub const SEL_ALLOWANCE: &str = "dd62ed3e"; // allowance(address,address)

// WithdrawRequestNFT
pub const SEL_CLAIM_WITHDRAW: &str = "b13acedd"; // claimWithdraw(uint256)
pub const SEL_BATCH_CLAIM_WITHDRAW: &str = "24fccdcf"; // batchClaimWithdraw(uint256[])
pub const SEL_GET_REQUEST: &str = "c58343ef"; // getRequest(uint256)
pub const SEL_IS_FINALIZED: &str = "33727c4d"; // isFinalized(uint256)
pub const SEL_GET_CLAIMABLE_AMOUNT: &str = "7d8ca242"; // getClaimableAmount(uint256)
