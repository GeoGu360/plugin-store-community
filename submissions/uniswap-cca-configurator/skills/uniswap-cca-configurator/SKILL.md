---
name: uniswap-cca-configurator
description: "Configure CCA (Continuous Clearing Auction) smart contract parameters"
version: "1.0.0"
author: "Uniswap Labs"
tags:
  - uniswap
  - cca
  - auction
  - token-distribution
---

# CCA Configuration

Configure Continuous Clearing Auction (CCA) smart contract parameters for fair and transparent token distribution.

## Overview

This skill guides AI agents through configuring CCA auction parameters via an interactive form flow. CCA is a novel auction mechanism that generalizes the uniform-price auction into continuous time, enabling fair token distribution with transparent pricing.

## Pre-flight Checks

1. Understanding of the CCA mechanism and token distribution goals
2. Token contract deployed and address known
3. Target chain selected (Ethereum, Base, Arbitrum, Unichain)

## Configuration Parameters

Key parameters collected during configuration:

| Parameter | Description |
|-----------|-------------|
| Network | Target chain (Ethereum, Base, Arbitrum, Unichain) |
| Token address | ERC-20 token to distribute |
| Total supply | Amount of tokens for the auction |
| Currency | Payment token (USDC, USDT, ETH) |
| Duration | Auction length in blocks |
| Floor price | Minimum acceptable price |
| Tick spacing | Price granularity |

The configuration outputs a JSON file ready for deployment.

## Full Skill

For the complete interactive configuration flow with supply schedule generation, Q96 price calculations, and validation:

```
npx skills add Uniswap/uniswap-ai
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Invalid token address | Not a valid Ethereum address | Verify 0x + 40 hex characters |
| Floor price not divisible by tick spacing | Pricing constraint violation | Adjust floor price or tick spacing |

## Skill Routing

- For deploying configured auctions -> use `uniswap-cca-deployer`
- For v4 hook security -> use `uniswap-v4-security-foundations`
