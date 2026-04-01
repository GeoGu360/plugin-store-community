---
name: uniswap-pay-with-any-token
description: "Pay HTTP 402 payment challenges using any token via Tempo CLI and Uniswap Trading API"
version: "2.0.0"
author: "Uniswap Labs"
tags:
  - uniswap
  - payments
  - x402
  - mpp
---

# Pay With Any Token

Pay HTTP 402 Payment Required challenges using any token via the Tempo CLI and Uniswap Trading API.

## Overview

This skill handles HTTP 402 Payment Required responses by detecting the payment challenge (MPP or x402), swapping held tokens to the required payment token via Uniswap, constructing the payment credential, and retrying the original request.

## Pre-flight Checks

1. The `tempo` CLI is installed and a wallet is configured (`tempo wallet status`)
2. The wallet holds tokens on a supported chain
3. For cross-chain payments: bridging may be required

## Commands

### Detect Payment Challenge

When an HTTP request returns 402, parse the `WWW-Authenticate` header or JSON body to extract:
- Required token address and amount
- Payment recipient address
- Network/chain ID

### Swap to Required Token

Use the Uniswap Trading API to swap held tokens to the required payment token:

```bash
# Check what tokens the wallet holds
tempo wallet balance

# The skill automatically handles the swap via Trading API
```

### Construct and Send Payment

After obtaining the required token, construct the payment credential (EIP-3009 for MPP, or x402 format) and retry the original request with the payment attached.

## Full Skill

For the complete implementation with credential construction, nonce generation, EIP-3009 signing, cross-chain bridging, and retry logic:

```
npx skills add Uniswap/uniswap-ai
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Insufficient balance" | Not enough tokens to cover swap + payment | Check balance and top up wallet |
| "Unsupported chain" | Payment required on chain wallet is not on | Bridge tokens first |
| "Payment rejected" | Credential construction error | Verify nonce and signature params |

## Skill Routing

- For general swap integration -> use `uniswap-swap-integration`
- For viem/wagmi blockchain setup -> use `uniswap-viem-integration`
