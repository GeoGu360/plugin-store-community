---
name: uniswap-v2
description: "Swap tokens and manage liquidity on Uniswap V2 AMM (Ethereum mainnet). Trigger phrases: swap tokens uniswap, add liquidity uniswap v2, remove liquidity uniswap, get price uniswap, quote uniswap v2. 中文：Uniswap V2 兑换代币，添加流动性，移除流动性，查询价格，查询储备金。"
version: "0.1.0"
author: "GeoGu360"
tags:
  - dex
  - amm
  - uniswap
  - ethereum
  - swap
  - liquidity
---

# Uniswap V2 Skill

## Overview

This skill enables interaction with the Uniswap V2 classic xy=k AMM on Ethereum mainnet (chain ID 1). It handles token swaps (ETH→token, token→ETH, token→token), liquidity provisioning (add and remove), and read operations (quotes, pair addresses, reserves, prices). Write operations — after user confirmation — are submitted via `onchainos wallet contract-call` with `--force`. Read operations use direct `eth_call` JSON-RPC to `https://ethereum.publicnode.com`.

## Pre-flight Checks

- `onchainos` CLI must be installed and an Ethereum wallet must be configured (`onchainos wallet addresses`)
- The `uniswap-v2` binary must be built: `cargo build --release` in the plugin directory
- No additional npm or pip packages required

## Commands

### quote — Get expected swap output

```bash
uniswap-v2 quote --token-in ETH --token-out USDC --amount-in 100000000000000000
```

- Calls `getAmountsOut` on the Uniswap V2 Router02
- Routes token→token swaps through WETH for best liquidity
- Returns raw output amount and 0.5% slippage minimum

**Example output:**
```
Uniswap V2 Quote
  Path:            ETH → USDC
  Amount in:       100000000000000000 (raw wei/units)
  Amount out:      185000000 (raw wei/units)
  Slippage (0.5%): 184075000 minimum out
```

---

### swap — Swap tokens

```bash
# ETH → token
uniswap-v2 swap --token-in ETH --token-out USDC --amount-in 100000000000000000

# token → ETH
uniswap-v2 swap --token-in USDC --token-out ETH --amount-in 185000000

# token → token
uniswap-v2 swap --token-in USDT --token-out DAI --amount-in 100000000

# dry run (builds calldata, no broadcast)
uniswap-v2 swap --token-in ETH --token-out USDC --amount-in 100000000000000000 --dry-run
```

**Before submitting: ask the user to confirm the swap details (amount, tokens, estimated output) before proceeding with `onchainos wallet contract-call`.**

Behavior by variant:
- **ETH→token**: calls `swapExactETHForTokens` (selector `0x7ff36ab5`) with `--amt <wei>` and `--force`; no approve needed
- **token→ETH**: checks allowance → approves if needed (wait 3s) → calls `swapExactTokensForETH` (selector `0x18cbafe5`) with `--force`
- **token→token**: checks allowance → approves if needed (wait 3s) → routes via WETH → calls `swapExactTokensForTokens` (selector `0x38ed1739`) with `--force`

**Example output:**
```
Swap: 100000000000000000 wei ETH → USDC (swapExactETHForTokens)
  amountOutMin: 184075000
  to: 0xYourWallet
  deadline: 1712345600
  txHash: 0xabc123...
```

---

### add-liquidity — Add liquidity to a pool

```bash
# token + ETH
uniswap-v2 add-liquidity --token-a USDC --token-b ETH --amount-a 185000000 --amount-b 100000000000000000

# token + token
uniswap-v2 add-liquidity --token-a USDT --token-b DAI --amount-a 100000000 --amount-b 100000000000000000000

# dry run
uniswap-v2 add-liquidity --token-a USDC --token-b ETH --amount-a 185000000 --amount-b 100000000000000000 --dry-run
```

**Before submitting: ask the user to confirm the liquidity amounts before proceeding with `onchainos wallet contract-call`.**

Sequence for ETH pair:
1. Check allowance for token → approve if needed (wait 5s)
2. Submit `addLiquidityETH` (selector `0xf305d719`) with `--amt <ethWei>` and `--force`

Sequence for token+token:
1. Check allowance for tokenA → approve if needed (wait 5s)
2. Check allowance for tokenB → approve if needed (wait 5s)
3. Submit `addLiquidity` (selector `0xe8e33700`) with `--force`

---

### remove-liquidity — Remove liquidity from a pool

```bash
# Remove all LP tokens (omit --liquidity for full balance)
uniswap-v2 remove-liquidity --token-a USDC --token-b ETH

# Remove specific LP amount
uniswap-v2 remove-liquidity --token-a USDC --token-b ETH --liquidity 1000000000000000000

# dry run
uniswap-v2 remove-liquidity --token-a USDC --token-b ETH --dry-run
```

**Before submitting: ask the user to confirm the LP amount to remove before proceeding with `onchainos wallet contract-call`.**

Sequence:
1. Get pair address from factory (`getPair`)
2. Get LP balance (`balanceOf`)
3. Approve LP token to Router if needed (wait 5s)
4. Submit `removeLiquidityETH` (selector `0x02751cec`) or `removeLiquidity` (selector `0xbaa2abde`) with `--force`

---

### get-pair — Get pair contract address

```bash
uniswap-v2 get-pair --token-a ETH --token-b USDC
```

Returns the pair contract address from `UniswapV2Factory.getPair()`. Pair address is also the LP token address.

---

### get-price — Get token price from reserves

```bash
uniswap-v2 get-price --token-a ETH --token-b USDT
```

Computes the decimal-adjusted spot price from `pair.getReserves()`. Handles different token decimals (e.g. WETH=18, USDC/USDT=6, WBTC=8).

---

### get-reserves — Get pair reserves

```bash
uniswap-v2 get-reserves --token-a ETH --token-b USDC
```

Returns raw `reserve0` and `reserve1` from `pair.getReserves()`, mapped to tokenA and tokenB order.

---

## Token Symbols

Built-in symbol resolution for Ethereum mainnet (chain ID 1):

| Symbol | Address | Decimals |
|--------|---------|----------|
| ETH / WETH | `0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2` | 18 |
| USDT | `0xdAC17F958D2ee523a2206206994597C13D831ec7` | 6 |
| USDC | `0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48` | 6 |
| DAI | `0x6B175474E89094C44Da98b954EedeAC495271d0F` | 18 |
| WBTC | `0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599` | 8 |
| UNI | `0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984` | 18 |

For unlisted tokens, pass the full hex address (e.g. `0xABC123...`).

**Important:** USDT and USDC on Ethereum mainnet use **6 decimals** (unlike BSC-Peg USDT which uses 18).

## Error Handling

| Error | Cause | Fix |
|-------|-------|-----|
| `txHash: pending` | Missing `--force` flag | Always use `--force` on DEX calls (built into this plugin) |
| `eth_call error` | Wrong RPC or network issue | Plugin uses `ethereum.publicnode.com` — verify network connectivity |
| `Pair does not exist` | No V2 pool for the token pair | Use `get-pair` to verify; try routing via WETH |
| `No LP balance found` | Wallet has no LP tokens for this pool | Verify with `get-pair` and check wallet LP balance |
| `Replacement transaction underpriced` | Repeated approve without checking allowance | Plugin checks allowance before every approve |
| `ABI encoding error` | Token symbol not resolved to address | Use known symbols or pass full hex address |
| `Reserve for X is zero` | Pool is empty or just created | Check `get-reserves` — pool may have no liquidity |

## Architecture

- Read ops: direct `eth_call` via JSON-RPC to `https://ethereum.publicnode.com`
- Write ops: after user confirmation, submits via `onchainos wallet contract-call` with `--force`
- Token resolution: `config.rs` symbol map → hex address before any ABI call
- Recipient: always fetched via `onchainos wallet addresses` — never zero address in live mode
- Slippage: 0.5% default (995/1000)
- Deadline: `current_timestamp + 1200` (20 minutes)
- Approve wait: 3s after ERC-20 approve before swap; 5s between sequential steps in add/remove liquidity

## Skill Routing

- Use Uniswap V3 skill for concentrated liquidity positions and multi-fee-tier routing
- Use PancakeSwap V2 skill for the same AMM model on BSC (chain ID 56)
