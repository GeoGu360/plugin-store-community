---
name: etherfi-stake
description: Stake ETH with ether.fi liquid restaking protocol to receive eETH or weETH, wrap/unwrap between the two, request withdrawals, check withdrawal status, and claim finalized ETH — all on Ethereum mainnet.
version: "0.1.0"
author: "GeoGu360"
tags:
  - staking
  - liquid-staking
  - restaking
  - eth
  - eeth
  - weeth
  - etherfi
---

# ether.fi Liquid Restaking Plugin

## Overview

This plugin enables interaction with the ether.fi liquid restaking protocol on Ethereum mainnet (chain ID 1). Users can stake ETH to receive eETH (rebasing) or weETH (non-rebasing), wrap/unwrap between the two tokens, request ETH withdrawals, and claim finalized withdrawals once the ether.fi oracle has processed them.

**Key facts:**
- eETH is a rebasing token: its balance grows daily without emitting Transfer events — always read fresh from chain.
- weETH is non-rebasing: its balance is fixed, but its ETH value increases over time as the exchange rate rises.
- Staking and withdrawals are only supported on Ethereum mainnet (chain ID 1).
- Withdrawal finalization typically takes a few hours but may take longer during high demand.
- All write operations require explicit user confirmation before submission.

## Architecture

- Read ops (balance, APY, withdrawal status, exchange rate) → direct `eth_call` via JSON-RPC to `https://ethereum.publicnode.com`
- APY data → HTTP GET to DefiLlama yields API (`https://yields.llama.fi`)
- Write ops → after user confirmation, submitted via `onchainos wallet contract-call --force`

## Contract Addresses (Ethereum Mainnet)

| Contract | Address |
|---|---|
| LiquidityPool | `0x308861A430be4cce5502d0A12724771Fc6DaF216` |
| eETH | `0x35fA164735182de50811E8e2E824cFb9B6118ac2` |
| weETH | `0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee` |
| WithdrawRequestNFT | `0x7d5706f6ef3F89B3951E23e557CDFBC3239D4E2c` |
| DepositAdapter | `0xcfc6d9bd7411962bfe7145451a7ef71a24b6a7a2` |

## Pre-flight Checks

Before running any command:
1. Verify `onchainos` is installed: `onchainos --version` (requires ≥ 2.0.0)
2. For write operations, verify wallet is logged in: `onchainos wallet addresses`
3. If wallet check fails, prompt: "Please log in with `onchainos wallet login` first."

---

## Commands

### `get-apy` — Get Current weETH APY

Fetch the current ether.fi weETH staking APY from DefiLlama. No wallet required.

**Usage:**
```
etherfi-stake get-apy
```

**Steps:**
1. HTTP GET `https://yields.llama.fi/chart/46bd2bdf-6d92-4066-b482-e885ee172264`
2. Take the latest data point from the `data` array.
3. Display: `"Current weETH APY: X.XXX% (7-day avg: X.XXX%)"`.

**Example output:**
```
=== ether.fi weETH APY ===
Current APY:        2.449%
7-day base APY:     2.743%
TVL:                $5457940962
As of:              2026-04-05T04:01:06.210Z
```

**No onchainos command required** — pure HTTP GET.

---

### `stake` — Stake ETH

Deposit ETH into ether.fi to receive weETH (default) or eETH.

**Usage:**
```
etherfi-stake stake --amount-eth <ETH> [--referral <ADDR>] [--from <ADDR>] [--prefer-eeth] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--amount-eth` | Yes | ETH amount to stake (e.g. `2.0`) |
| `--referral` | No | Referral address (defaults to zero address) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--prefer-eeth` | No | Receive eETH directly via LiquidityPool instead of weETH |
| `--dry-run` | No | Show calldata without broadcasting |

**Default path (weETH via DepositAdapter — recommended):**
1. Fetch APY from DefiLlama for display.
2. Show user: amount, expected token, contract, calldata.
3. **Ask user to confirm** the transaction before submitting.
4. Execute:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0xcfc6d9bd7411962bfe7145451a7ef71a24b6a7a2 \
     --amt <WEI> \
     --input-data 0xef54591d0000000000000000000000000000000000000000000000000000000000000000 \
     --force
   ```

**Alternative path (eETH via LiquidityPool, `--prefer-eeth`):**
```bash
onchainos wallet contract-call \
  --chain 1 \
  --to 0x308861A430be4cce5502d0A12724771Fc6DaF216 \
  --amt <WEI> \
  --input-data 0xd0e30db0 \
  --force
```

**Example:**
```bash
# Stake 2 ETH, receive weETH
etherfi-stake stake --amount-eth 2.0

# Dry run preview
etherfi-stake stake --amount-eth 1.5 --dry-run
```

---

### `balance` — Check eETH and weETH Balance

Query eETH balance, weETH balance, and current exchange rate for an address.

**Usage:**
```
etherfi-stake balance [--address <ADDR>]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--address` | No | Address to query (resolved from onchainos if omitted) |

**Steps:**
1. `eth_call` → `eETH.balanceOf(address)` — read eETH balance.
2. `eth_call` → `weETH.balanceOf(address)` — read weETH balance.
3. `eth_call` → `weETH.getRate()` — read exchange rate (1 weETH = X eETH).
4. Display all three.

**Note:** eETH is rebasing — always read fresh, never cache.

---

### `wrap` — Wrap eETH into weETH

Convert eETH into weETH (non-rebasing, DeFi-friendly). Requires two transactions if allowance is insufficient.

**Usage:**
```
etherfi-stake wrap --amount-eth <ETH> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--amount-eth` | Yes | eETH amount to wrap (e.g. `3.0`) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**

**Step 1 — Check allowance (read-only):**
- `eth_call` → `eETH.allowance(owner, weETH_address)`.
- Skip approve if allowance ≥ amount.

**Step 2 — Approve eETH (if needed):**
1. Show user: eETH contract, spender, amount, calldata.
2. **Ask user to confirm** the approve transaction before submitting.
3. Execute:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0x35fA164735182de50811E8e2E824cFb9B6118ac2 \
     --input-data 0x095ea7b3000000000000000000000000cd5fe23c85820f7b72d0926fc9b05b43e359b7ee<AMOUNT_32B> \
     --force
   ```

**Step 3 — Wrap eETH → weETH:**
1. Show user: weETH contract, amount, calldata.
2. **Ask user to confirm** the wrap transaction before submitting.
3. Execute:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee \
     --input-data 0xea598cb0<AMOUNT_32B> \
     --force
   ```

---

### `unwrap` — Unwrap weETH back to eETH

Convert weETH back to eETH. Single transaction, no approve needed.

**Usage:**
```
etherfi-stake unwrap --amount-eth <ETH> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--amount-eth` | Yes | weETH amount to unwrap (e.g. `2.87`) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**
1. `eth_call` → `weETH.getEETHByWeETH(amount)` to display expected eETH output.
2. Show user: amount, expected eETH, contract, calldata.
3. **Ask user to confirm** the unwrap transaction before submitting.
4. Execute:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee \
     --input-data 0xde0e9a3e<AMOUNT_32B> \
     --force
   ```

---

### `request-withdrawal` — Request eETH Withdrawal

Burn eETH and mint a WithdrawRequestNFT (ERC-721) representing the right to claim ETH after finalization.

**Usage:**
```
etherfi-stake request-withdrawal --amount-eth <ETH> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--amount-eth` | Yes | eETH amount to withdraw (e.g. `1.0`) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**
1. Show user: amount, recipient, contract, calldata.
2. Warn: the NFT must be held until claim time — transferring it transfers the claim right.
3. **Ask user to confirm** the withdrawal request before submitting.
4. Execute:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0x308861A430be4cce5502d0A12724771Fc6DaF216 \
     --input-data 0x397a1b28<RECIPIENT_32B><AMOUNT_32B> \
     --force
   ```

**Notes:**
- No prior `approve()` needed — LiquidityPool calls `eETH.burnShares()` internally.
- The returned `tokenId` (from ERC-721 Transfer event in the receipt) is the NFT ID needed for claim.
- Rewards stop accruing from the moment `requestWithdraw` is called.

---

### `get-withdrawals` — Check Withdrawal Request Status

Query the status of one or more WithdrawRequestNFT token IDs.

**Usage:**
```
etherfi-stake get-withdrawals --token-ids <ID1,ID2,...> [--address <ADDR>]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--token-ids` | Yes | Comma-separated NFT token IDs (e.g. `7842,7843`) |
| `--address` | No | Display-only wallet address |

**Steps (per token ID):**
1. `eth_call` → `WithdrawRequestNFT.getRequest(tokenId)` — fetch eETH amount, validity, fee.
2. `eth_call` → `WithdrawRequestNFT.isFinalized(tokenId)` — check if claimable.
3. `eth_call` → `WithdrawRequestNFT.getClaimableAmount(tokenId)` — ETH available to claim.
4. Display status: PENDING / READY TO CLAIM / INVALIDATED.

**Note:** Typically finalizes within hours, but may take longer during high withdrawal demand.

---

### `claim-withdrawal` — Claim Finalized ETH

Claim ETH from finalized WithdrawRequestNFT(s). Burns the NFT(s) and sends ETH to `msg.sender`.

**Usage:**
```
etherfi-stake claim-withdrawal --token-ids <ID1,ID2,...> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--token-ids` | Yes | Comma-separated NFT token IDs to claim (e.g. `7842`) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**
1. For each token ID: verify `isFinalized == true` (abort if any are not ready).
2. Fetch `getClaimableAmount` for each ID; display total ETH to be received.
3. **Ask user to confirm** the claim transaction before submitting.
4. Single token — execute `claimWithdraw(tokenId)`:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0x7d5706f6ef3F89B3951E23e557CDFBC3239D4E2c \
     --input-data 0xb13acedd<TOKEN_ID_32B> \
     --force
   ```
5. Multiple tokens — execute `batchClaimWithdraw(uint256[])`:
   ```bash
   onchainos wallet contract-call \
     --chain 1 \
     --to 0x7d5706f6ef3F89B3951E23e557CDFBC3239D4E2c \
     --input-data 0x24fccdcf<ABI_ENCODED_ARRAY> \
     --force
   ```

**Prerequisites:**
- All token IDs must be finalized (`isFinalized == true`). Check with `get-withdrawals` first.
- `msg.sender` must be the NFT owner or an approved operator.

---

## Error Handling

| Error | Cause | Resolution |
|---|---|---|
| "Could not resolve wallet address" | Not logged in to onchainos | Run `onchainos wallet login` |
| "Stake amount must be greater than 0" | Zero or negative ETH entered | Enter a positive ETH amount |
| "Wrap amount must be greater than 0" | Zero or negative amount | Enter a positive amount |
| "No token IDs provided" | Missing `--token-ids` argument | Pass `--token-ids <ID1,ID2,...>` |
| "Token IDs not yet finalized" | Withdrawal not processed by oracle | Wait and check again with `get-withdrawals` |
| HTTP error from DefiLlama | API temporarily unavailable | Retry; APY display is non-blocking |
| "eth_call RPC error" | Public RPC issue | Retry or configure a different RPC URL |

---

## Suggested Follow-ups

After **stake**: suggest `etherfi-stake balance` to verify received weETH/eETH, or `etherfi-stake get-apy` for current yield.

After **wrap**: suggest `etherfi-stake balance` to confirm weETH balance.

After **request-withdrawal**: suggest `etherfi-stake get-withdrawals --token-ids <ID>` to monitor finalization.

After **get-withdrawals** (if READY TO CLAIM): suggest `etherfi-stake claim-withdrawal --token-ids <ID>`.

After **claim-withdrawal**: suggest checking ETH balance with `onchainos wallet balance --chain 1`.

---

## Skill Routing

- For SOL liquid staking → use the `jito` skill
- For Lido stETH staking → use the `lido` skill
- For wallet balance queries → use `onchainos wallet balance`
- For general DeFi operations → use the appropriate protocol plugin
