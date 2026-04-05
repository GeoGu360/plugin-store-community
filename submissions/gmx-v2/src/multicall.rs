use alloy_primitives::{Address, Bytes, U256, B256, FixedBytes};
use alloy_sol_types::{sol, SolCall};

sol! {
    function multicall(bytes[] calldata data) external returns (bytes[] memory results);
    function sendWnt(address receiver, uint256 amount) external;
    function sendTokens(address token, address receiver, uint256 amount) external;

    struct OrderAddresses {
        address receiver;
        address cancellationReceiver;
        address callbackContract;
        address uiFeeReceiver;
        address market;
        address initialCollateralToken;
        address[] swapPath;
    }

    struct OrderNumbers {
        uint256 sizeDeltaUsd;
        uint256 initialCollateralDeltaAmount;
        uint256 triggerPrice;
        uint256 acceptablePrice;
        uint256 executionFee;
        uint256 callbackGasLimit;
        uint256 minOutputAmount;
        uint256 validFromTime;
    }

    struct CreateOrderParams {
        OrderAddresses addresses;
        OrderNumbers numbers;
        uint8 orderType;
        uint8 decreasePositionSwapType;
        bool isLong;
        bool shouldUnwrapNativeToken;
        bool autoCancel;
        bytes32 referralCode;
        bytes32[] dataList;
    }

    function createOrder(CreateOrderParams params) external returns (bytes32);

    struct DepositAddresses {
        address receiver;
        address callbackContract;
        address uiFeeReceiver;
        address market;
        address initialLongToken;
        address initialShortToken;
        address[] longTokenSwapPath;
        address[] shortTokenSwapPath;
    }

    struct CreateDepositParams {
        DepositAddresses addresses;
        uint256 minMarketTokens;
        bool shouldUnwrapNativeToken;
        uint256 executionFee;
        uint256 callbackGasLimit;
        bytes32[] dataList;
    }

    function createDeposit(CreateDepositParams params) external returns (bytes32);

    struct WithdrawalAddresses {
        address receiver;
        address callbackContract;
        address uiFeeReceiver;
        address market;
        address[] longTokenSwapPath;
        address[] shortTokenSwapPath;
    }

    struct CreateWithdrawalParams {
        WithdrawalAddresses addresses;
        uint256 minLongTokenAmount;
        uint256 minShortTokenAmount;
        bool shouldUnwrapNativeToken;
        uint256 executionFee;
        uint256 callbackGasLimit;
        bytes32[] dataList;
    }

    function createWithdrawal(CreateWithdrawalParams params) external returns (bytes32);
}

/// ABI-encode a multicall(bytes[]) call from a list of inner calldatas.
pub fn build_multicall(calls: Vec<Vec<u8>>) -> Vec<u8> {
    let data: Vec<Bytes> = calls.into_iter().map(Bytes::from).collect();
    let call = multicallCall { data };
    call.abi_encode()
}

pub fn encode_send_wnt(receiver: Address, amount: U256) -> Vec<u8> {
    sendWntCall { receiver, amount }.abi_encode()
}

pub fn encode_send_tokens(token: Address, receiver: Address, amount: U256) -> Vec<u8> {
    sendTokensCall {
        token,
        receiver,
        amount,
    }
    .abi_encode()
}

#[allow(clippy::too_many_arguments)]
pub fn encode_create_order(
    receiver: Address,
    cancellation_receiver: Address,
    market: Address,
    initial_collateral_token: Address,
    swap_path: Vec<Address>,
    size_delta_usd: U256,
    initial_collateral_delta_amount: U256,
    trigger_price: U256,
    acceptable_price: U256,
    execution_fee: U256,
    min_output_amount: U256,
    order_type: u8,
    is_long: bool,
    referral_code: B256,
) -> Vec<u8> {
    let params = CreateOrderParams {
        addresses: OrderAddresses {
            receiver,
            cancellationReceiver: cancellation_receiver,
            callbackContract: Address::ZERO,
            uiFeeReceiver: Address::ZERO,
            market,
            initialCollateralToken: initial_collateral_token,
            swapPath: swap_path,
        },
        numbers: OrderNumbers {
            sizeDeltaUsd: size_delta_usd,
            initialCollateralDeltaAmount: initial_collateral_delta_amount,
            triggerPrice: trigger_price,
            acceptablePrice: acceptable_price,
            executionFee: execution_fee,
            callbackGasLimit: U256::ZERO,
            minOutputAmount: min_output_amount,
            validFromTime: U256::ZERO,
        },
        orderType: order_type,
        decreasePositionSwapType: 0u8,
        isLong: is_long,
        shouldUnwrapNativeToken: false,
        autoCancel: false,
        referralCode: FixedBytes(referral_code.0),
        dataList: vec![],
    };
    createOrderCall { params }.abi_encode()
}

#[allow(clippy::too_many_arguments)]
pub fn encode_create_deposit(
    receiver: Address,
    market: Address,
    initial_long_token: Address,
    initial_short_token: Address,
    min_market_tokens: U256,
    execution_fee: U256,
) -> Vec<u8> {
    let params = CreateDepositParams {
        addresses: DepositAddresses {
            receiver,
            callbackContract: Address::ZERO,
            uiFeeReceiver: Address::ZERO,
            market,
            initialLongToken: initial_long_token,
            initialShortToken: initial_short_token,
            longTokenSwapPath: vec![],
            shortTokenSwapPath: vec![],
        },
        minMarketTokens: min_market_tokens,
        shouldUnwrapNativeToken: false,
        executionFee: execution_fee,
        callbackGasLimit: U256::ZERO,
        dataList: vec![],
    };
    createDepositCall { params }.abi_encode()
}

#[allow(clippy::too_many_arguments)]
pub fn encode_create_withdrawal(
    receiver: Address,
    market: Address,
    min_long_token_amount: U256,
    min_short_token_amount: U256,
    execution_fee: U256,
) -> Vec<u8> {
    let params = CreateWithdrawalParams {
        addresses: WithdrawalAddresses {
            receiver,
            callbackContract: Address::ZERO,
            uiFeeReceiver: Address::ZERO,
            market,
            longTokenSwapPath: vec![],
            shortTokenSwapPath: vec![],
        },
        minLongTokenAmount: min_long_token_amount,
        minShortTokenAmount: min_short_token_amount,
        shouldUnwrapNativeToken: true,
        executionFee: execution_fee,
        callbackGasLimit: U256::ZERO,
        dataList: vec![],
    };
    createWithdrawalCall { params }.abi_encode()
}

/// Parse a hex address string ("0x...") into an alloy Address.
pub fn parse_address(s: &str) -> anyhow::Result<Address> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).map_err(|e| anyhow::anyhow!("invalid address hex: {}", e))?;
    if bytes.len() != 20 {
        anyhow::bail!("address must be 20 bytes, got {}", bytes.len());
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Address::from(arr))
}

/// Encode final multicall calldata to a 0x-prefixed hex string.
pub fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}
