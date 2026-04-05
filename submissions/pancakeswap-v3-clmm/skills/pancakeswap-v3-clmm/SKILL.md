---
name: pancakeswap-v3-clmm
description: Swap tokens and manage concentrated liquidity positions on PancakeSwap V3 CLMM (BSC and Base)
---

# PancakeSwap V3 CLMM

PancakeSwap V3 is the largest DEX on BSC and a high-fidelity Uniswap V3 fork using a Concentrated Liquidity Market Maker (CLMM) model. LP positions are ERC-721 NFTs with custom tick-ranged liquidity. This plugin supports swapping, adding/removing liquidity, collecting fees, and querying positions on BSC (chain ID 56) and Base (chain ID 8453).

**Architecture:** Read-only operations (quote, get-positions, get-pools) use direct `eth_call` via JSON-RPC. Write ops → after user confirmation, submits via `onchainos wallet contract-call`.

---

## Pre-flight Checks

```bash
# Ensure onchainos CLI is installed and wallet is configured
onchainos wallet balance --chain 56
```

The binary `pancakeswap-v3-clmm` must be available in your PATH. It is built from Rust source and distributed as a plugin binary.

---

## Commands

### 1. `quote` — Get a Swap Quote

Queries QuoterV2 via `eth_call` (no transaction). Automatically finds the best fee tier unless `--fee` is specified.

```bash
pancakeswap-v3-clmm quote \
  --token-in WBNB \
  --token-out USDT \
  --amount-in 10000000000000000 \
  --chain 56
```

**Output:**
```json
{"ok":true,"tokenIn":"0xbb4...","tokenOut":"0x55d...","amountIn":10000000000000000,"bestFee":2500,"amountOut":5123456789}
```

**Notes:**
- Validates pool exists via Factory before calling QuoterV2 (avoids 0-liquidity false quotes)
- Fee tiers: 100 (0.01%), 500 (0.05%), 2500 (0.25%), 10000 (1%)
- PancakeSwap V3 uses 0.25% (2500) instead of Uniswap V3's 0.3% (3000)

---

### 2. `swap` — Swap Tokens

Executes `exactInputSingle` on SmartRouter. Quotes first, then asks for confirmation before submitting.

```bash
pancakeswap-v3-clmm swap \
  --token-in WBNB \
  --token-out USDT \
  --amount-in 10000000000000000 \
  --slippage 0.5 \
  --chain 56
```

**With dry run (no broadcast):**
```bash
pancakeswap-v3-clmm swap --token-in WBNB --token-out USDT --amount-in 10000000000000000 --dry-run
```

**Output:**
```json
{"ok":true,"txHash":"0xabc...","tokenIn":"0xbb4...","tokenOut":"0x55d...","amountIn":10000000000000000,"fee":2500,"amountOutMin":5098000000}
```

**Flow:**
1. QuoterV2 eth_call to get best fee tier and amountOut
2. **Ask user to confirm** the quote (token amounts, fee tier, slippage)
3. Check ERC-20 allowance; approve if needed (3-second delay after approve)
4. Submit `wallet contract-call` with `--force` to SmartRouter

**Important:** The recipient is always resolved from the connected wallet. Never zero address.

---

### 3. `add-liquidity` — Add Concentrated Liquidity

Mints a new CLMM position via NonfungiblePositionManager. Tick values can be negative.

```bash
pancakeswap-v3-clmm add-liquidity \
  --token0 WETH \
  --token1 USDC \
  --fee 500 \
  --tick-lower -23027 \
  --tick-upper -20012 \
  --amount0-desired 1000000000000000 \
  --amount1-desired 1800000000 \
  --chain 8453
```

**Output:**
```json
{"ok":true,"txHash":"0xdef...","token0":"0x420...","token1":"0x833...","fee":500,"tickLower":-23027,"tickUpper":-20012}
```

**Flow:**
1. Verify pool exists via Factory
2. **Ask user to confirm** position parameters (tick range, token amounts)
3. Approve token0 to NonfungiblePositionManager if needed (5-second delay)
4. Approve token1 to NonfungiblePositionManager if needed (5-second delay)
5. Submit `wallet contract-call` with `--force` for mint

**Note:** Ticks must be multiples of the fee tier's tick spacing (500 fee → spacing 10).

---

### 4. `remove-liquidity` — Remove Liquidity

Removes liquidity from an existing position via `decreaseLiquidity` then `collect`.

```bash
pancakeswap-v3-clmm remove-liquidity \
  --token-id 1234 \
  --chain 56

# Remove all liquidity and burn the empty NFT
pancakeswap-v3-clmm remove-liquidity --token-id 1234 --burn --chain 56
```

**Output:**
```json
{"ok":true,"tokenId":1234,"decreaseTx":"0x...","collectTx":"0x...","burnTx":""}
```

**Flow:**
1. Fetch current position data (`positions(tokenId)`)
2. **Ask user to confirm** the liquidity amount to remove
3. Submit `wallet contract-call` with `--force` for `decreaseLiquidity`
4. Wait 5 seconds for nonce clearance
5. Submit `wallet contract-call` with `--force` for `collect` (uint128::MAX for both amounts)
6. Optionally submit `wallet contract-call` with `--force` for `burn` if `--burn` flag set

---

### 5. `collect-fees` — Collect Accumulated Fees

Collects protocol fees owed to a position without removing liquidity.

```bash
pancakeswap-v3-clmm collect-fees --token-id 1234 --chain 56
```

**Output:**
```json
{"ok":true,"txHash":"0x...","tokenId":1234,"recipient":"0xYourWallet"}
```

**Flow:**
1. Fetch position to check `tokensOwed0` / `tokensOwed1`
2. If both are zero, exit early with no-op message
3. **Ask user to confirm** the fee amounts before collecting
4. Submit `wallet contract-call` with `--force` to collect(tokenId, recipient, uint128::MAX, uint128::MAX)

---

### 6. `get-positions` — List LP Positions

Lists all CLMM positions for a wallet address (read-only, no transaction).

```bash
# Query connected wallet
pancakeswap-v3-clmm get-positions --chain 56

# Query a specific address
pancakeswap-v3-clmm get-positions --owner 0xYourWallet --chain 56
```

**Output:**
```json
{
  "ok": true,
  "owner": "0x...",
  "positions": [
    {
      "tokenId": 1234,
      "token0": "0x...",
      "token1": "0x...",
      "fee": 2500,
      "tickLower": -887200,
      "tickUpper": 887200,
      "liquidity": "1000000000000000000",
      "tokensOwed0": "0",
      "tokensOwed1": "123456"
    }
  ]
}
```

---

### 7. `get-pools` — Query Pool Addresses

Queries Factory for pool addresses. Returns deployment status for each fee tier.

```bash
pancakeswap-v3-clmm get-pools --token0 WBNB --token1 USDT --chain 56

# Query a specific fee tier
pancakeswap-v3-clmm get-pools --token0 WETH --token1 USDC --fee 500 --chain 8453
```

**Output:**
```json
{
  "ok": true,
  "token0": "0xbb4...",
  "token1": "0x55d...",
  "pools": [
    {"fee": 100, "address": "0x000...", "deployed": false},
    {"fee": 500, "address": "0xabc...", "deployed": true},
    {"fee": 2500, "address": "0xdef...", "deployed": true},
    {"fee": 10000, "address": "0x000...", "deployed": false}
  ]
}
```

---

## Error Handling

| Error | Likely Cause | Fix |
|-------|-------------|-----|
| `No valid pool or quote found` | Pool not deployed or zero liquidity | Use `get-pools` to verify; check fee tier |
| `Pool does not exist` | Factory returns zero address | Pool hasn't been deployed for that fee tier |
| `onchainos: command not found` | onchainos CLI not installed | Install onchainos CLI |
| `txHash: "pending"` | Missing `--force` flag | Internal — should not occur in this plugin |
| Swap reverts with `TF` | Wrong recipient or 0-liquidity pool | Plugin uses resolved wallet; check pool with `get-pools` |
| `attempt to subtract with overflow` | Tick decode bug | Plugin uses safe last-8-hex decode for ticks |

---

## Contract Addresses

### BSC (chain ID 56)
| Contract | Address |
|---------|---------|
| SmartRouter | `0x13f4EA83D0bd40E75C8222255bc855a974568Dd4` |
| NonfungiblePositionManager | `0x46A15B0b27311cedF172AB29E4f4766fbE7F4364` |
| QuoterV2 | `0xB048Bbc1Ee6b733FFfCFb9e9CeF7375518e25997` |
| Factory | `0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865` |

### Base (chain ID 8453)
| Contract | Address |
|---------|---------|
| SmartRouter | `0x678Aa4bF4E210cf2166753e054d5b7c31cc7fa86` |
| NonfungiblePositionManager | `0x46A15B0b27311cedF172AB29E4f4766fbE7F4364` |
| Factory | `0x0BFbCF9fa4f9C56B0F40a671Ad40E0805A091865` |

---

## Skill Routing

- For portfolio tracking across protocols, use `okx-defi-portfolio`
- For cross-DEX aggregated swaps, use `okx-dex-swap`
- For token price data, use `okx-dex-token`
