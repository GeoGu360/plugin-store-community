# ether.fi Liquid Restaking Plugin

Stake ETH with [ether.fi](https://ether.fi) to receive eETH or weETH, wrap/unwrap between them, request withdrawals, and claim finalized ETH — all via the onchainos Plugin Store.

## Commands

| Command | Description |
|---|---|
| `etherfi-stake get-apy` | Get current weETH staking APY from DefiLlama |
| `etherfi-stake stake` | Stake ETH to receive weETH (default) or eETH |
| `etherfi-stake balance` | Check eETH and weETH balances for an address |
| `etherfi-stake wrap` | Wrap eETH into weETH (approve + wrap) |
| `etherfi-stake unwrap` | Unwrap weETH back to eETH |
| `etherfi-stake request-withdrawal` | Request withdrawal of eETH for ETH (mints WithdrawRequestNFT) |
| `etherfi-stake get-withdrawals` | Check finalization status of WithdrawRequestNFT token IDs |
| `etherfi-stake claim-withdrawal` | Claim finalized ETH (burns NFT, sends ETH to wallet) |

## Requirements

- `onchainos` CLI ≥ 2.0.0
- Wallet logged in for write operations (`onchainos wallet login`)

## Token Overview

| Token | Type | Notes |
|---|---|---|
| eETH | Rebasing ERC-20 | Balance grows daily without Transfer events |
| weETH | Non-rebasing ERC-20 | Fixed balance; ETH value increases as exchange rate rises |

## Contracts (Ethereum Mainnet)

| Contract | Address |
|---|---|
| LiquidityPool | `0x308861A430be4cce5502d0A12724771Fc6DaF216` |
| eETH | `0x35fA164735182de50811E8e2E824cFb9B6118ac2` |
| weETH | `0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee` |
| WithdrawRequestNFT | `0x7d5706f6ef3F89B3951E23e557CDFBC3239D4E2c` |
| DepositAdapter | `0xcfc6d9bd7411962bfe7145451a7ef71a24b6a7a2` |

## Build

```bash
cargo build --release
```

## License

MIT
