---
name: aerodrome-slipstream
description: Swap tokens and manage concentrated liquidity positions on Aerodrome Slipstream CLMM on Base (chain 8453)
version: 0.1.0
author: GeoGu360
tags:
  - dex
  - clmm
  - aerodrome
  - concentrated-liquidity
  - base
---

# Aerodrome Slipstream CLMM

Aerodrome Slipstream is the largest concentrated liquidity DEX on Base, a high-fidelity Uniswap V3 fork using `tickSpacing` (int24) instead of `fee` (uint24) to identify pool levels. LP positions are ERC-721 NFTs with custom tick-ranged liquidity. TVL exceeds $300M (April 2026). This plugin supports swapping, adding/removing liquidity, collecting fees, and querying positions on Base (chain ID 8453).

**Key difference from Uniswap V3 / PancakeSwap V3:** Aerodrome Slipstream uses `tickSpacing` not `fee` — all pool queries, swaps, and mint calls use tick spacing values (1/50/100/200/2000) instead of fee tiers.

**Architecture:** Read-only operations (quote, get-positions, get-pools) use direct `eth_call` via JSON-RPC to `https://base-rpc.publicnode.com`. Write ops use `onchainos wallet contract-call --force` after user confirmation.

---

## Pre-flight Checks

```bash
# Ensure onchainos CLI is installed and wallet is configured
onchainos wallet addresses
```

The binary `aerodrome-slipstream` must be available in your PATH.

---

## Tick Spacing Reference

| Tick Spacing | Approx Fee | Typical Use |
|---|---|---|
| 1 | 0.01% | Highly correlated (wstETH/WETH) |
| 50 | 0.05% | Stablecoins |
| 100 | 0.05% | Stablecoins (alt) |
| 200 | 0.30% | Main volatile pairs (WETH/USDC, WETH/AERO) |
| 2000 | 1.00% | High-volatility tokens |

---

## Commands

### 1. `quote` — Get a Swap Quote

Queries QuoterV2 via `eth_call` (no transaction). Automatically finds the best tick spacing unless `--tick-spacing` is specified.

```bash
aerodrome-slipstream quote \
  --token-in USDC \
  --token-out WETH \
  --amount-in 1000000
```

**Output:**
```json
{"ok":true,"tokenIn":"0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913","tokenOut":"0x4200000000000000000000000000000000000006","amountIn":1000000,"bestTickSpacing":1,"amountOut":486579542286783}
```

**Notes:**
- Validates pool exists via CLFactory before calling QuoterV2 (avoids 0-liquidity false quotes)
- Tick spacings checked: 1, 50, 100, 200, 2000
- Returns best amountOut across all deployed pools
- USDC uses 6 decimals (1 USDC = 1000000), WETH uses 18 decimals

---

### 2. `swap` — Swap Tokens

Executes `exactInputSingle` on the Aerodrome SwapRouter. Quotes first, then asks for confirmation before submitting.

```bash
aerodrome-slipstream swap \
  --token-in USDC \
  --token-out WETH \
  --amount-in 10000000 \
  --slippage 0.5
```

**With dry run (no broadcast):**
```bash
aerodrome-slipstream swap --token-in USDC --token-out WETH --amount-in 10000000 --dry-run
```

**Output:**
```json
{"ok":true,"txHash":"0xabc...","tokenIn":"0x833...","tokenOut":"0x420...","amountIn":10000000,"tickSpacing":1,"amountOutMin":4841464202061528}
```

**Flow:**
1. QuoterV2 eth_call to find best tick spacing and amountOut
2. **Ask user to confirm** the quote (token amounts, tick spacing, slippage)
3. Check ERC-20 allowance; approve SwapRouter if needed (3-second delay after approve)
4. Submit `wallet contract-call --force` to SwapRouter (selector `0xa026383e`)

**Important:** Recipient is always the connected wallet address. Never zero address in non-dry-run mode.

---

### 3. `add-liquidity` — Add Concentrated Liquidity

Mints a new CLMM position via NonfungiblePositionManager. Tick values can be negative.

```bash
aerodrome-slipstream add-liquidity \
  --token0 WETH \
  --token1 USDC \
  --tick-spacing 200 \
  --tick-lower -23027 \
  --tick-upper -20012 \
  --amount0-desired 1000000000000000 \
  --amount1-desired 1800000000
```

**Output:**
```json
{"ok":true,"txHash":"0xdef...","token0":"0x420...","token1":"0x833...","tickSpacing":200,"tickLower":-23027,"tickUpper":-20012}
```

**Flow:**
1. Verify pool exists via CLFactory (`getPool(token0, token1, tickSpacing)`)
2. **Ask user to confirm** position parameters (tick range, token amounts, tickSpacing)
3. Approve token0 to NonfungiblePositionManager if needed (5-second delay)
4. Approve token1 to NonfungiblePositionManager if needed (5-second delay)
5. Submit `wallet contract-call --force` for mint (selector `0xb5007d1f`)

**Notes:**
- Ticks must be multiples of tickSpacing (e.g. for tickSpacing=200, ticks must be multiples of 200)
- `tick_lower` and `tick_upper` accept negative values (use `--tick-lower -23027`)
- Aerodrome MintParams includes `sqrtPriceX96` field (set to 0 when pool already exists)
- `sqrtPriceX96=0` means "use existing pool price" (do NOT set a value unless initializing a new pool)

---

### 4. `remove-liquidity` — Remove Liquidity

Removes liquidity from an existing position via `decreaseLiquidity` then `collect`.

```bash
# Remove all liquidity from position
aerodrome-slipstream remove-liquidity --token-id 13465

# Remove all and burn the empty NFT
aerodrome-slipstream remove-liquidity --token-id 13465 --burn
```

**Output:**
```json
{"ok":true,"tokenId":13465,"decreaseTx":"0x...","collectTx":"0x...","burnTx":""}
```

**Flow:**
1. Fetch current position data (`positions(tokenId)`)
2. **Ask user to confirm** the liquidity amount to remove
3. Submit `wallet contract-call --force` for `decreaseLiquidity` (selector `0x0c49ccbe`)
4. Wait 5 seconds for nonce clearance
5. Submit `wallet contract-call --force` for `collect` (selector `0xfc6f7865`) with uint128::MAX for both token amounts
6. Optionally submit `wallet contract-call --force` for `burn` (selector `0x42966c68`) if `--burn` flag set

---

### 5. `collect-fees` — Collect Accumulated Fees

Collects fees owed to a position without removing liquidity.

```bash
aerodrome-slipstream collect-fees --token-id 13465
```

**Output:**
```json
{"ok":true,"txHash":"0x...","tokenId":13465,"recipient":"0xYourWallet"}
```

**Flow:**
1. Fetch position to check `tokensOwed0` / `tokensOwed1`
2. If both are zero, exit early with no-op message
3. **Ask user to confirm** the fee amounts before collecting
4. Submit `wallet contract-call --force` to `collect(tokenId, recipient, uint128::MAX, uint128::MAX)` (selector `0xfc6f7865`)

---

### 6. `get-positions` — List LP Positions

Lists all CLMM positions for a wallet address (read-only, no transaction).

```bash
# Query connected wallet
aerodrome-slipstream get-positions

# Query a specific address
aerodrome-slipstream get-positions --owner 0xYourWalletAddress
```

**Output:**
```json
{
  "ok": true,
  "owner": "0x...",
  "positions": [
    {
      "tokenId": 13465,
      "token0": "0x4200000000000000000000000000000000000006",
      "token1": "0x940181a94A35A4569E4529A3CDfB74e38FD98631",
      "tickSpacing": 200,
      "tickLower": 78800,
      "tickUpper": 79200,
      "liquidity": "13589538797482293814",
      "tokensOwed0": "0",
      "tokensOwed1": "0"
    }
  ]
}
```

**Notes:**
- `tickSpacing` in output is Aerodrome-specific (replaces `fee` from Uniswap V3)
- `liquidity=0` means the position is empty (can still have fees via `collect`)

---

### 7. `get-pools` — Query Pool Addresses

Queries CLFactory for pool addresses across all tick spacings.

```bash
aerodrome-slipstream get-pools --token0 USDC --token1 WETH

# Query a specific tick spacing
aerodrome-slipstream get-pools --token0 WETH --token1 AERO --tick-spacing 200
```

**Output:**
```json
{
  "ok": true,
  "token0": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  "token1": "0x4200000000000000000000000000000000000006",
  "pools": [
    {"tickSpacing": 1, "address": "0xdbc6998296caa1652a810dc8d3baf4a8294330f1", "deployed": true},
    {"tickSpacing": 50, "address": "0x0000000000000000000000000000000000000000", "deployed": false},
    {"tickSpacing": 100, "address": "0xb2cc224c1c9fee385f8ad6a55b4d94e92359dc59", "deployed": true},
    {"tickSpacing": 200, "address": "0x148bc43946a902258916e580b0e6d92aaa74746f", "deployed": true},
    {"tickSpacing": 2000, "address": "0x0652202c4b2d09cb93aedefadc14b36869483a98", "deployed": true}
  ]
}
```

---

## Supported Token Symbols (Base mainnet)

| Symbol | Address |
|--------|---------|
| WETH / ETH | `0x4200000000000000000000000000000000000006` |
| USDC | `0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913` |
| CBBTC | `0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf` |
| AERO | `0x940181a94A35A4569E4529A3CDfB74e38FD98631` |
| DAI | `0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb` |
| USDT | `0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2` |
| WSTETH | `0xc1CBa3fCea344f92D9239c08C0568f6F2F0ee452` |

For any other token, pass the hex address directly (e.g. `--token-in 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`).

---

## Contract Addresses (Base, chain ID 8453)

| Contract | Address |
|---------|---------|
| SwapRouter (CLSwapRouter) | `0xBE6D8f0d05cC4be24d5167a3eF062215bE6D18a5` |
| NonfungiblePositionManager | `0x827922686190790b37229fd06084350E74485b72` |
| CLFactory | `0x5e7BB104d84c7CB9B682AaC2F3d509f5F406809A` |
| QuoterV2 | `0x254cF9E1E6e233aa1AC962CB9B05b2cfeAaE15b0` |

---

## Error Handling

| Error | Likely Cause | Fix |
|-------|-------------|-----|
| `No valid pool or quote found` | Pool not deployed for tick spacing | Use `get-pools` to verify; try different tick spacing |
| `Pool does not exist for .../tickSpacing=...` | Factory returns zero address | Pool hasn't been deployed; use an existing tick spacing |
| `execution reverted: ID` | Token ID does not exist in NFPM | Verify tokenId with `get-positions` |
| `onchainos: command not found` | onchainos CLI not installed | Install and configure onchainos CLI |
| `txHash: "pending"` | Missing `--force` flag | Internal error — should not occur in this plugin |
| `No fees owed` | Both tokensOwed are 0 | Normal — position has no pending fees to collect |
| Swap reverts | Wrong recipient or insufficient allowance | Plugin auto-approves; check balance with `onchainos wallet balance` |

---

## Skill Routing

- For portfolio tracking across protocols, use `okx-defi-portfolio`
- For cross-DEX aggregated swaps (best price across all DEXes), use `okx-dex-swap`
- For token price data and market info, use `okx-dex-token`
- For Aerodrome V2 (stable/volatile AMM, non-CLMM), use a separate plugin
