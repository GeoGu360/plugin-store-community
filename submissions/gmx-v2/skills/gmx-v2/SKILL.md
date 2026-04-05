---
name: gmx-v2
description: Trade perpetuals and manage GM pool liquidity on GMX V2 (Arbitrum/Avalanche). Supports opening/closing leveraged long and short positions, token swaps, GM pool deposits/withdrawals, and ERC-20 approvals via onchainos.
---

# GMX V2 Plugin

GMX V2 (GMX Synthetics) is a decentralized perpetuals and spot trading protocol on Arbitrum and Avalanche. This skill enables AI agents to trade leveraged positions (up to 100x), swap tokens, and provide/remove liquidity in GM pools.

**Architecture:** Read ops call the GMX REST API directly. Write ops — after user confirmation — submit via `onchainos wallet contract-call` using the GMX multicall pattern (batching sendWnt, sendTokens, and createOrder/createDeposit/createWithdrawal into a single transaction).

---

## Pre-flight Checks

1. Install the GMX V2 plugin binary:
   ```
   npx onchainos plugin install gmx-v2
   ```
2. Ensure onchainos is logged in with a funded wallet:
   ```
   onchainos wallet balance --chain 42161
   ```
3. Verify the wallet has sufficient ETH for execution fees (~0.0005–0.002 ETH per operation).
4. For perp trades and GM deposits, approve your collateral token first using `approve-token`.

---

## Commands

### get-markets

Fetch all available GM pools and tradeable markets.

**Use case:** Discover market token addresses, long/short token pairs, and pool listings before trading.

**Example:**
```
gmx-v2 get-markets --chain 42161
```

**Output:** Table of market names, listing status, and market token addresses.

---

### get-prices

Fetch current oracle prices for all tokens from the GMX price feed.

**Use case:** Check current ETH, BTC, or other token prices before placing orders. Prices use GMX's 30-decimal internal format and are displayed in human-readable USD.

**Example:**
```
gmx-v2 get-prices --chain 42161
```

**Output:** Table of token symbols, min/max USD prices, and token addresses.

---

### get-positions

Fetch all open perpetual positions for a wallet address.

**Use case:** View current leveraged positions including size, collateral, direction, and unrealized PnL.

**Example:**
```
gmx-v2 get-positions --chain 42161
gmx-v2 get-positions --chain 42161 --account 0xYourAddress
```

**Output:** Table of positions with market, direction (LONG/SHORT), USD size, collateral, and PnL.

---

### open-long

Open a leveraged long position on a GMX V2 market.

**Use case:** Enter a long perpetual position when bullish on an asset. Uses the GMX multicall pattern: sendWnt (execution fee) + sendTokens (collateral) + createOrder (MarketIncrease, isLong=true).

**Ask user to confirm** the position parameters before submitting the transaction via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 open-long \
  --chain 42161 \
  --market 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --collateral-token 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 \
  --size-usd 5000 \
  --collateral-amount 500000000 \
  --oracle-price 2000000000000000000000000000000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

**Parameters:**
- `--market`: GM market token address (from `get-markets`)
- `--collateral-token`: ERC-20 collateral token address
- `--size-usd`: Position size in USD (e.g., 5000 for $5000)
- `--collateral-amount`: Collateral in token units (e.g., 500000000 for 500 USDC with 6 decimals)
- `--oracle-price`: Raw 30-decimal price from `get-prices`
- `--execution-fee`: Override execution fee in wei (default: 500000000000000)

---

### open-short

Open a leveraged short position on a GMX V2 market.

**Use case:** Enter a short perpetual position when bearish on an asset. Uses the GMX multicall pattern with isLong=false.

**Ask user to confirm** the position parameters before the transaction is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 open-short \
  --chain 42161 \
  --market 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --collateral-token 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 \
  --size-usd 5000 \
  --collateral-amount 500000000 \
  --oracle-price 2000000000000000000000000000000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

### close-position

Close an open perpetual position (partial or full).

**Use case:** Exit a long or short position by creating a MarketDecrease order. Uses multicall: sendWnt (execution fee) + createOrder (MarketDecrease).

**Ask user to confirm** before the close order is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 close-position \
  --chain 42161 \
  --market 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --collateral-token 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 \
  --size-usd 5000 \
  --is-long true \
  --oracle-price 2000000000000000000000000000000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

### swap

Swap tokens using GMX V2's market swap mechanism.

**Use case:** Route a token swap through one or more GMX markets. Uses multicall: sendTokens (input) + sendWnt (fee) + createOrder (MarketSwap).

**Ask user to confirm** the swap parameters before the transaction is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 swap \
  --chain 42161 \
  --input-token 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1 \
  --input-amount 500000000000000000 \
  --swap-path 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --min-output 990000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

### deposit-gm

Deposit tokens into a GM liquidity pool to receive GM tokens.

**Use case:** Provide liquidity to earn trading fees. Uses multicall: sendWnt (fee) + sendTokens (long) + sendTokens (short) + createDeposit.

**Ask user to confirm** the deposit amounts before the transaction is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 deposit-gm \
  --chain 42161 \
  --market 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --long-token 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1 \
  --short-token 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 \
  --long-amount 100000000000000000 \
  --short-amount 200000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

### withdraw-gm

Burn GM tokens to withdraw underlying long and short tokens from a pool.

**Use case:** Remove liquidity from a GM pool. Uses multicall: sendWnt (fee) + sendTokens (GM tokens → WithdrawalVault) + createWithdrawal.

**Ask user to confirm** the withdrawal before the transaction is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 withdraw-gm \
  --chain 42161 \
  --market 0x70d95587d40A2caf56bd97485aB3Eec10Bee6336 \
  --gm-amount 1000000000000000000 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

### approve-token

Approve an ERC-20 token for the GMX Router (required before first perp trade or deposit).

**Use case:** Set unlimited allowance for the GMX Router contract on a collateral token.

**Ask user to confirm** the approval before the transaction is submitted via `onchainos wallet contract-call`.

**Example:**
```
gmx-v2 approve-token \
  --chain 42161 \
  --token 0xaf88d065e77c8cC2239327C5EDb3A432268e5831 \
  --dry-run
```

Remove `--dry-run` to submit after user confirms.

---

## Price Precision

GMX V2 uses 30-decimal price precision internally:
- `sizeDeltaUsd`: $1000 = `1000 * 10^30`
- API prices from `/prices/tickers`: divide by `10^(30 - token_decimals)` for human-readable USD
- For ETH (18 decimals): `price_usd = raw_price / 10^12`
- For USDC (6 decimals): `price_usd = raw_price / 10^24`

---

## Execution Fees

All write ops require ETH as an execution fee paid to the GMX keeper network:
- Order creation (perp, swap): ~0.0005 ETH (500000000000000 wei)
- Deposit/withdrawal: ~0.001 ETH (1000000000000000 wei)
- Excess fees are refunded automatically by the contracts.

---

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| `Cannot get wallet address` | onchainos not logged in | Run `onchainos wallet login` |
| `Unsupported chain ID` | Invalid chain specified | Use 42161 (Arbitrum) or 43114 (Avalanche) |
| `invalid address hex` | Malformed token/market address | Verify address from `get-markets` output |
| HTTP 429 / timeout | API rate limiting | Retry after a few seconds |
| Order not executed | Keeper delay or price impact too high | Check acceptable price and retry |

---

## Skill Routing

- For wallet balance checks: use `onchainos wallet balance`
- For token approvals on other protocols: use `approve-token` with the relevant spender
- For reading market data without trading: `get-markets`, `get-prices`, `get-positions` require no wallet
- This skill handles only GMX V2 (Synthetics). For GMX V1, use a separate skill.
