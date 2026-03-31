---
name: smart-money-copy-trade
description: "Monitor smart money, whale, and KOL buy signals across chains, then execute copy trades with security checks and risk controls. Supports Solana, Ethereum, Base, and BSC."
version: "1.0.0"
author: "oker"
tags:
  - smart-money
  - copy-trading
  - whale-tracking
  - trading-strategy
---

# Smart Money Copy Trade

## Overview

An automated copy-trading strategy that monitors smart money, whale, and KOL buy signals on-chain and helps you execute follow-up trades with built-in security screening and risk management. The strategy follows a **Signal → Screen → Size → Execute** pipeline, ensuring every trade is validated before execution.

## Pre-flight Checks

Before using this skill, ensure:

1. The `onchainos` CLI is installed and authenticated
2. A wallet is configured (`onchainos wallet status`)
3. Network connectivity is available
4. Sufficient balance on the target chain for gas + trade amount

## Strategy Pipeline

The copy-trade strategy operates in 4 phases:

```
Phase 1: SIGNAL   — Scan smart money / whale / KOL buy signals
Phase 2: SCREEN   — Security scan + token fundamentals check
Phase 3: SIZE     — Calculate position size based on risk parameters
Phase 4: EXECUTE  — Quote → Approve (EVM) → Swap → Confirm
```

## Commands

### Phase 1: Signal Discovery

#### Check supported chains for signals

```bash
onchainos signal chains
```

**When to use**: First step — confirm which chains support signal tracking.

#### Scan smart money buy signals

```bash
# Smart money signals on Solana
onchainos signal list --chain solana --wallet-type 1

# Whale signals on Ethereum (min $10k trades)
onchainos signal list --chain ethereum --wallet-type 3 --min-amount-usd 10000

# All signal types (smart money + KOL + whale) on Base
onchainos signal list --chain base --wallet-type "1,2,3"

# Filter signals for a specific token
onchainos signal list --chain solana --token-address <address> --wallet-type 1
```

**When to use**: Core signal discovery. Run periodically or on user request.
**Output**: Token symbol, wallet type, amount USD, trigger wallet count, sold ratio.
**Key metric**: `soldRatioPercent` — lower means wallets are still holding (stronger signal).

**Signal strength scoring**:

| Factor | Strong Signal | Weak Signal |
|--------|--------------|-------------|
| Trigger wallet count | >= 3 wallets | 1 wallet |
| Sold ratio | < 20% | > 60% |
| Amount USD | > $50k | < $5k |
| Wallet type overlap | Smart money + Whale | Single type |

### Phase 2: Security Screening

#### Token security scan

```bash
onchainos security token-scan --address <token_address> --chain <chain>
```

**When to use**: MANDATORY before any trade. Check for honeypots, rug-pull risks, tax tokens.
**Decision rules**:
- `action = "block"` → DO NOT TRADE. Inform user of risk details.
- `action = "warn"` → Show risk details, require explicit user confirmation.
- `action = null/empty` → Safe to proceed.
- Scan fails → DO NOT TRADE. Retry once, then abort.

#### Token fundamentals check

```bash
# Market cap, liquidity, holders
onchainos token price-info --address <token_address> --chain <chain>

# Holder concentration risk
onchainos token holders --address <token_address> --chain <chain>
```

**When to use**: After security scan passes. Evaluate if the token is worth trading.
**Minimum thresholds** (configurable by user):

| Metric | Default Minimum | Rationale |
|--------|----------------|-----------|
| Liquidity | $50,000 | Below this, slippage risk too high |
| Market cap | $500,000 | Below this, manipulation risk too high |
| Holder count | 100 | Below this, concentration risk too high |
| Top holder % | < 30% | Above this, whale dump risk |

### Phase 3: Position Sizing

Calculate trade amount based on user's risk parameters:

```
Position Size = min(
  Portfolio Balance × Risk Per Trade %,
  Token Liquidity × Max Liquidity Impact %,
  User Max Position Size
)
```

**Default risk parameters** (user can override):

| Parameter | Default | Range |
|-----------|---------|-------|
| Risk per trade | 2% of portfolio | 1% - 10% |
| Max liquidity impact | 2% of pool | 1% - 5% |
| Max position size | $500 | $50 - $10,000 |
| Stop-loss | -15% | -5% to -30% |
| Take-profit | +50% | +20% to +200% |

#### Check portfolio balance

```bash
onchainos portfolio total --chain <chain>
```

### Phase 4: Trade Execution

#### Get swap quote

```bash
onchainos swap quote \
  --from <native_or_stablecoin_address> \
  --to <signal_token_address> \
  --amount <amount_in_minimal_units> \
  --chain <chain>
```

**Pre-execution checks**:
- `isHoneyPot = true` → BLOCK the trade
- `priceImpact > 5%` → WARN, suggest reducing amount
- `priceImpact > 10%` → BLOCK, suggest splitting into smaller trades
- `taxRate > 0` → Display tax rate to user

#### Execute swap (Solana example)

```bash
onchainos swap swap \
  --from 11111111111111111111111111111111 \
  --to <signal_token_address> \
  --amount <amount_in_lamports> \
  --chain solana \
  --wallet <your_wallet_address>
```

#### Execute swap (EVM — requires approve first)

```bash
# Step 1: Approve (skip for native token)
onchainos swap approve \
  --token <from_token_address> \
  --amount <amount_in_wei> \
  --chain <chain>

# Step 2: Swap
onchainos swap swap \
  --from <from_token_address> \
  --to <signal_token_address> \
  --amount <amount_in_wei> \
  --chain <chain> \
  --wallet <your_wallet_address>
```

#### Sign and broadcast (Agentic Wallet)

```bash
# EVM
onchainos wallet contract-call \
  --to <contract_address> \
  --chain <chain> \
  --value <value_in_UI_units> \
  --input-data <swap_calldata>

# Solana
onchainos wallet contract-call \
  --to <contract_address> \
  --chain sol \
  --unsigned-tx <unsigned_tx_data>
```

## Full Workflow Example

> User: "Copy trade smart money on Solana — budget $200"

```
1. onchainos signal chains                                      → Confirm Solana supports signals
2. onchainos signal list --chain solana --wallet-type "1,2,3"   → Get all signal types
   → Agent scores signals by strength (wallet count, sold ratio, amount)
   → Presents top 3 candidates to user

3. User picks token X from signal list

4. onchainos security token-scan --address <X> --chain solana   → Security check
   → If block: skip, suggest next candidate
   → If warn: show details, ask user

5. onchainos token price-info --address <X> --chain solana      → Check liquidity, mcap
   → If below minimums: warn user

6. Calculate position: min($200, 2% of liquidity, 2% of portfolio)

7. onchainos swap quote --from 111...111 --to <X> --amount <size> --chain solana
   → Display: expected output, gas, price impact, slippage

8. User confirms → onchainos swap swap --from 111...111 --to <X> --amount <size> --chain solana --wallet <addr>

9. onchainos wallet contract-call --to <contract> --chain sol --unsigned-tx <tx>
   → Swap complete

10. Log trade: token, entry price, amount, timestamp
    → Set mental stop-loss at -15%, take-profit at +50%
```

## Monitoring Mode

After executing trades, the strategy supports passive monitoring:

```bash
# Check current price of held token
onchainos market price --address <token_address> --chain <chain>

# Check K-line for momentum
onchainos market kline --address <token_address> --chain <chain> --interval 1h

# Check if smart money is selling (rising soldRatio)
onchainos signal list --chain <chain> --token-address <token_address>

# Check wallet PnL
onchainos market pnl --address <wallet_address> --chain <chain>
```

**Exit signals** (any one triggers a sell recommendation):
- Price drops below stop-loss (-15%)
- Smart money soldRatio rises above 60%
- Security scan status changes to warn/block
- Price hits take-profit (+50%)

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Token not found" | Invalid address or unsupported chain | Verify token address from signal data |
| "Rate limited" | Too many API requests | Wait 10 seconds and retry once |
| "Chain not supported" | Chain doesn't support signals | Run `onchainos signal chains` to check |
| "Insufficient balance" | Not enough tokens for trade | Check balance, reduce trade amount |
| "Security scan failed" | API timeout or error | DO NOT trade — retry scan once, then abort |
| "Honeypot detected" | Token is a honeypot | BLOCK trade, skip to next signal candidate |
| "High price impact" | Low liquidity for trade size | Reduce amount or split into smaller trades |
| "Approval failed" | ERC-20 approve tx reverted | Check if token has special approval logic (e.g., USDT reset) |

## Risk Controls

| Risk | Action | Details |
|------|--------|---------|
| Honeypot token | BLOCK | Never buy honeypot tokens |
| Tax rate > 10% | WARN | Display tax, require confirmation |
| Price impact > 5% | WARN | Suggest reducing size |
| Price impact > 10% | BLOCK | Must split trade |
| Liquidity < $50k | WARN | High slippage risk |
| Token age < 1 hour | WARN | Extremely high risk, likely pump-and-dump |
| Top holder > 30% | WARN | Whale dump risk |
| Security scan fails | BLOCK | Cannot verify safety |
| Signal from 1 wallet only | WARN | Weak signal, could be noise |

## Skill Routing

- For detailed token analytics → use `okx-dex-token`
- For meme/pump.fun token research → use `okx-dex-trenches`
- For price charts and K-line → use `okx-dex-market`
- For transaction broadcasting → use `okx-onchain-gateway`
- For wallet balance and transfers → use `okx-wallet-portfolio`
- For URL/DApp phishing checks → use `okx-security`

## Configuration

Users can customize strategy parameters by stating preferences:

- "Set my max position to $1000" → updates max position size
- "Use 5% risk per trade" → updates risk percentage
- "Only follow whale signals" → filters to wallet-type 3 only
- "Set stop-loss at -10%" → tightens stop-loss
- "Focus on Solana and Base only" → limits chain scope

The agent remembers these preferences for the duration of the conversation.
