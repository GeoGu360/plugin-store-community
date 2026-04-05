---
name: quickswap-dex
description: "Swap tokens and manage liquidity on QuickSwap V2 AMM (Polygon). Trigger phrases: swap tokens quickswap, add liquidity quickswap, remove liquidity quickswap, get price quickswap, quote quickswap v2, quickswap polygon. 中文：QuickSwap V2 兑换代币，添加流动性，移除流动性，查询价格，Polygon DEX。"
license: MIT
metadata:
  author: GeoGu360
  version: "0.1.0"
---

# QuickSwap V2 Skill

## Overview

This skill enables interaction with the QuickSwap V2 classic xy=k AMM on Polygon (chain ID 137). It handles token swaps (MATIC→token, token→MATIC, token→token), liquidity provisioning (add and remove), and read operations (quotes, pair addresses, reserves, prices). Write ops — after user confirmation, submits via `onchainos wallet contract-call` with `--force`. Read ops use direct `eth_call` to `https://polygon-bor-rpc.publicnode.com`.

**Key facts:**
- Chain: Polygon PoS (chain ID 137)
- Protocol: QuickSwap V2 (Uniswap V2 fork, xy=k, 0.30% fee)
- Router: `0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff`
- Factory: `0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32`
- WMATIC/WPOL: `0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270`
- USDC (native) and USDT on Polygon use **6 decimals** (not 18)

## Pre-flight Checks

- `onchainos` CLI must be installed and a Polygon wallet must be configured
- The `quickswap-dex` binary must be built: `cargo build --release` in the plugin directory
- No additional npm or pip packages required

## Commands

### quote — Get expected swap output

```bash
# Quote 10 MATIC → USDC (MATIC has 18 decimals)
quickswap-dex quote --token-in MATIC --token-out USDC --amount-in 10000000000000000000

# Quote 20 USDT → WETH (USDT has 6 decimals on Polygon)
quickswap-dex quote --token-in USDT --token-out WETH --amount-in 20000000

# Quote using raw addresses
quickswap-dex quote --token-in 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270 --token-out 0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359 --amount-in 1000000000000000000
```

- Calls `getAmountsOut` on the QuickSwap V2 Router
- Routes token→token swaps through WMATIC for best liquidity
- Returns raw output amount and 0.5% slippage minimum

**Example output:**
```
QuickSwap V2 Quote
  Path:       MATIC → USDC
  Amount in:  10000000000000000000 (raw units)
  Amount out: 4985000 (raw units)
  Slippage (0.5%): 4960075 minimum out
```

**IMPORTANT — decimal awareness:**
- MATIC/WMATIC, WETH, QUICK: 18 decimals → use `1000000000000000000` for 1 token
- USDC (native), USDC.e, USDT on Polygon: 6 decimals → use `1000000` for 1 token

---

### swap — Swap tokens

```bash
# MATIC → USDC (native MATIC, 10 MATIC = 10e18 wei)
quickswap-dex swap --token-in MATIC --token-out USDC --amount-in 10000000000000000000

# token → MATIC (USDC → MATIC, 5 USDC = 5000000 raw)
quickswap-dex swap --token-in USDC --token-out MATIC --amount-in 5000000

# token → token (USDT → WETH, 20 USDT = 20000000 raw)
quickswap-dex swap --token-in USDT --token-out WETH --amount-in 20000000

# dry run (builds calldata, no broadcast)
quickswap-dex swap --token-in MATIC --token-out USDC --amount-in 10000000000000000000 --dry-run
```

**Before submitting: ask the user to confirm the swap details (amount, tokens, estimated output) before proceeding with `onchainos wallet contract-call`.**

Behavior by variant:
- **MATIC→token**: calls `swapExactETHForTokens` with `--amt <wei>` and `--force`; no approve needed
- **token→MATIC**: checks allowance → approves if needed (wait 3s) → calls `swapExactTokensForETH` with `--force`
- **token→token**: checks allowance → approves if needed (wait 3s) → routes via WMATIC → calls `swapExactTokensForTokens` with `--force`

**Example output:**
```
Swap: 10000000000000000000 wei MATIC → USDC (swapExactETHForTokens)
  amountOutMin: 4960075
  to: 0xYourWallet
  deadline: 1743000000
  txHash: 0xabc123...
```

---

### add-liquidity — Add liquidity to a pool

```bash
# token + MATIC (USDC + 5 MATIC)
quickswap-dex add-liquidity --token-a USDC --token-b MATIC --amount-a 2490000 --amount-b 5000000000000000000

# token + token (USDC + USDT)
quickswap-dex add-liquidity --token-a USDC --token-b USDT --amount-a 1000000 --amount-b 1000000

# dry run
quickswap-dex add-liquidity --token-a USDC --token-b MATIC --amount-a 2490000 --amount-b 5000000000000000000 --dry-run
```

**Before submitting: ask the user to confirm the liquidity amounts before proceeding with `onchainos wallet contract-call`.**

Sequence:
1. Check allowance for tokenA → approve if needed (wait 5s)
2. Check allowance for tokenB → approve if needed (wait 5s)
3. Submit `addLiquidity` or `addLiquidityETH` with `--force`

For MATIC pairs, `--amt <maticWei>` is passed to `onchainos wallet contract-call` automatically.

---

### remove-liquidity — Remove liquidity from a pool

```bash
# Remove all LP tokens (omit --liquidity for full balance)
quickswap-dex remove-liquidity --token-a USDC --token-b MATIC

# Remove specific amount
quickswap-dex remove-liquidity --token-a USDC --token-b MATIC --liquidity 1000000000000000000

# token + token pair
quickswap-dex remove-liquidity --token-a USDC --token-b USDT

# dry run
quickswap-dex remove-liquidity --token-a USDC --token-b MATIC --dry-run
```

**Before submitting: ask the user to confirm the LP amount to remove before proceeding with `onchainos wallet contract-call`.**

Sequence:
1. Get pair address from factory (`factory.getPair`)
2. Get LP balance (`balanceOf`)
3. Compute expected return amounts from reserves + totalSupply
4. Approve LP token to Router if needed (wait 5s)
5. Submit `removeLiquidity` or `removeLiquidityETH` with `--force`

---

### get-pair — Get pair contract address

```bash
quickswap-dex get-pair --token-a MATIC --token-b USDC
quickswap-dex get-pair --token-a USDC --token-b USDT
quickswap-dex get-pair --token-a WETH --token-b USDC
```

Returns the pair contract address from `QuickSwap Factory.getPair()`. Pair address = LP token address for that pool.

---

### get-price — Get token price from reserves

```bash
quickswap-dex get-price --token-a MATIC --token-b USDC
quickswap-dex get-price --token-a WETH --token-b USDC
```

Computes price from `pair.getReserves()`. Correctly accounts for decimal differences between tokens (USDC/USDT = 6 decimals, MATIC/WETH = 18 decimals).

**Example output:**
```
QuickSwap V2 Price
  pair:    0x853ee4b2a13f8a742d64c8f088be7ba2131f670d
  MATIC reserve: 12450000000000000000000 (raw, 18 decimals)
  USDC reserve: 6200000000 (raw, 6 decimals)
  1 MATIC = 0.498000 USDC (from on-chain reserves)
```

---

### get-reserves — Get pair reserves

```bash
quickswap-dex get-reserves --token-a MATIC --token-b USDC
quickswap-dex get-reserves --token-a WETH --token-b USDC
```

Returns raw `reserve0` and `reserve1` from `pair.getReserves()`, mapped to tokenA and tokenB order.

---

## Token Symbols

Built-in symbol resolution for Polygon PoS (chain ID 137):

| Symbol | Address | Decimals |
|--------|---------|----------|
| MATIC / POL / WMATIC / WPOL | `0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270` | 18 |
| USDC (native) | `0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359` | **6** |
| USDC.e / USDCE (bridged) | `0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174` | **6** |
| USDT | `0xc2132D05D31c914a87C6611C10748AEb04B58e8f` | **6** |
| WETH / ETH | `0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619` | 18 |
| QUICK | `0xB5C064F955D8e7F38fE0460C556a72987494eE17` | 18 |

For unlisted tokens, pass the full hex address (e.g. `0xABC123...`).

**Critical:** USDC and USDT on Polygon use **6 decimals**. Always use raw units:
- 1 USDC = `1000000` (not `1000000000000000000`)
- 1 MATIC = `1000000000000000000`

---

## Error Handling

| Error | Cause | Fix |
|-------|-------|-----|
| `txHash: pending` | Missing `--force` flag | Always use `--force` on DEX calls (built into this plugin) |
| `eth_call error` | Wrong RPC URL or network issue | Plugin uses `polygon-bor-rpc.publicnode.com` |
| `Pair does not exist` | No V2 pool for the token pair | Use `get-pair` to verify; try routing via WMATIC |
| `No LP balance found` | Wallet has no LP tokens for this pool | Verify with `get-pair` and check wallet balance |
| `Replacement transaction underpriced` | Repeated approve without checking allowance | Plugin checks allowance before every approve |
| `Reserve is zero` | Pool is empty or newly created | Choose a different pool or wait for liquidity |
| `ABI encoding error` | Token symbol not resolved | Use known symbols or pass full hex address |

---

## Architecture

- Read ops: direct `eth_call` via JSON-RPC to `https://polygon-bor-rpc.publicnode.com`
- Write ops: after user confirmation, submits via `onchainos wallet contract-call` with `--force`
- Token resolution: `config.rs` symbol map → hex address before any ABI call
- Recipient: always fetched via `onchainos wallet addresses` — never zero address in live mode
- Slippage: 0.5% default (995/1000)
- Deadline: `current_timestamp + 1200` (20 minutes)
- Approve guard: checks `allowance` before every approve; skips if sufficient (avoids nonce conflicts)
- Multi-step delays: 3s after token approve before swap; 5s between steps in add/remove liquidity
