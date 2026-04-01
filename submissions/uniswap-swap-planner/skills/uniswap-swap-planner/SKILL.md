---
name: uniswap-swap-planner
description: "Plan token swaps and generate Uniswap deep links across all supported chains"
version: "0.2.1"
author: "Uniswap Labs"
tags:
  - uniswap
  - swap
  - deep-links
---

# Uniswap Swap Planner

Plan and generate deep links for token swaps on Uniswap across all supported chains.

## Overview

This skill helps AI agents plan token swaps by resolving token addresses, selecting the right chain, and generating Uniswap web interface deep links. It supports both known token swaps and exploratory token discovery workflows.

## Pre-flight Checks

1. Know the input and output token symbols or addresses
2. Know which chain to swap on (or let the skill recommend one)
3. A web browser to open the generated deep links

## Commands

### Plan a Known Swap

When the user specifies both tokens:

1. Resolve token contract addresses on the target chain
2. Validate the pair has liquidity on Uniswap
3. Generate the deep link: `https://app.uniswap.org/swap?inputCurrency=<addr>&outputCurrency=<addr>&chain=<chain>`

### Token Discovery

When the user asks "what should I buy" or wants to discover tokens:

1. Use keyword search and web search to find relevant tokens
2. Present options with contract addresses and chain info
3. Generate swap deep links for the user-selected tokens

**Note:** There is no live "trending" feed. Discovery uses search-based workflows.

## Full Skill

For the complete planning logic with multi-chain support, token resolution, and all deep link parameters:

```
npx skills add Uniswap/uniswap-ai
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Token not found | Invalid symbol or wrong chain | Verify token exists on the target chain |
| No liquidity | Pool does not exist | Try a different chain or route through a stablecoin |

## Skill Routing

- For building swap functionality into an app -> use `uniswap-swap-integration`
- For liquidity provision planning -> use `uniswap-liquidity-planner`
