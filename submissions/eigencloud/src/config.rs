#![allow(dead_code)]

/// Ethereum mainnet chain ID
pub const CHAIN_ID: u64 = 1;

// ─── Core EigenLayer Contracts ───────────────────────────────────────────────

/// StrategyManager — handles LST deposits
pub const STRATEGY_MANAGER: &str = "0x858646372CC42E1A627fcE94aa7A7033e7CF075A";

/// DelegationManager — handles operator delegation + withdrawal queue
pub const DELEGATION_MANAGER: &str = "0x39053D51B77DC0d36036Fc1fCc8Cb819df8Ef37b";

/// EigenPodManager — handles native ETH restaking (not used in this plugin)
#[allow(dead_code)]
pub const EIGEN_POD_MANAGER: &str = "0x91E677b07F7AF907ec9a428aafa9fc14a0d3A338";

// ─── LST Strategy Contracts ────────────────────────────────────────────────

pub const STRATEGY_STETH: &str = "0x93c4b944D05dfe6df7645A86cd2206016c51564D";
pub const STRATEGY_RETH: &str = "0x1BeE69b7dFFfA4E2d53C2a2Df135C388AD25dCD2";
pub const STRATEGY_CBETH: &str = "0x54945180dB7943c0ed0FEE7EdaB2Bd24620256bc";
pub const STRATEGY_ETHX: &str = "0x9d7eD45EE2E8FC5482fa2428f15C971e6369011d";
pub const STRATEGY_ANKRETH: &str = "0x13760F50a9d7377e4F20CB8CF9e4c26586c658ff";

// ─── LST Token Contracts ───────────────────────────────────────────────────

pub const TOKEN_STETH: &str = "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84";
pub const TOKEN_RETH: &str = "0xae78736Cd615f374D3085123A210448E74Fc6393";
pub const TOKEN_CBETH: &str = "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704";

// ─── Known Operators (open delegation — no approver required) ─────────────

pub struct OperatorInfo {
    pub address: &'static str,
    pub name: &'static str,
}

pub const KNOWN_OPERATORS: &[OperatorInfo] = &[
    OperatorInfo {
        address: "0x71c6F7Ed8C2d4925d0bAf16f6A85BB1736D412f",
        name: "P2P.org",
    },
    OperatorInfo {
        address: "0xa4820b796a0C47D4b3291b5E76e04b5e92fd6d72",
        name: "Figment",
    },
    OperatorInfo {
        address: "0xDbEd88D83176316fc46797B43aDeE927Dc2ff2F5",
        name: "Blockdaemon",
    },
    OperatorInfo {
        address: "0x5dD57Da40e6866C9FcC34F4b6DDC89F1BA740DfE",
        name: "Luganodes",
    },
    OperatorInfo {
        address: "0xcf9eF04c0298De47E00a5A4Ac5f96A03cFf41dC7",
        name: "HashKey Cloud",
    },
];

// ─── Function Selectors ────────────────────────────────────────────────────

// StrategyManager
/// depositIntoStrategy(address,address,uint256)
pub const SEL_DEPOSIT_INTO_STRATEGY: &str = "e7a050aa";
/// getDeposits(address)
pub const SEL_GET_DEPOSITS: &str = "94f649dd";

// DelegationManager
/// delegateTo(address,(bytes,uint256),bytes32)
pub const SEL_DELEGATE_TO: &str = "eea9064b";
/// undelegate(address)
pub const SEL_UNDELEGATE: &str = "da8be864";
/// queueWithdrawals((address[],uint256[],address)[])
pub const SEL_QUEUE_WITHDRAWALS: &str = "0dd8dd02";
/// delegatedTo(address)
pub const SEL_DELEGATED_TO: &str = "65da1264";
/// getDelegatableShares(address)
pub const SEL_GET_DELEGATABLE_SHARES: &str = "cf80873e";
/// getOperatorDetails(address)
pub const SEL_GET_OPERATOR_DETAILS: &str = "192ee2ee";
/// isDelegated(address)
pub const SEL_IS_DELEGATED: &str = "3e28391d";

// Strategy (per-strategy reads)
/// underlyingToken()
pub const SEL_UNDERLYING_TOKEN: &str = "2495a599";
/// totalShares()
pub const SEL_TOTAL_SHARES: &str = "3a98ef39";
/// sharesToUnderlyingView(uint256)
pub const SEL_SHARES_TO_UNDERLYING: &str = "7a8b2637";

// ERC20
/// approve(address,uint256)
pub const SEL_APPROVE: &str = "095ea7b3";
/// balanceOf(address)
pub const SEL_BALANCE_OF: &str = "70a08231";
/// allowance(address,address)
pub const SEL_ALLOWANCE: &str = "dd62ed3e";

/// Strategies in display order
pub struct StrategyMeta {
    pub address: &'static str,
    pub token: &'static str,
    pub symbol: &'static str,
}

pub const STRATEGIES: &[StrategyMeta] = &[
    StrategyMeta {
        address: STRATEGY_STETH,
        token: TOKEN_STETH,
        symbol: "stETH",
    },
    StrategyMeta {
        address: STRATEGY_RETH,
        token: TOKEN_RETH,
        symbol: "rETH",
    },
    StrategyMeta {
        address: STRATEGY_CBETH,
        token: TOKEN_CBETH,
        symbol: "cbETH",
    },
    StrategyMeta {
        address: STRATEGY_ETHX,
        token: "0xA35b1B31Ce002FBF2058D22F30f95D405200A15b",
        symbol: "ETHx",
    },
    StrategyMeta {
        address: STRATEGY_ANKRETH,
        token: "0xe95A203B1a91a908F9B9CE46459d101078c2c3cb",
        symbol: "ankrETH",
    },
];
