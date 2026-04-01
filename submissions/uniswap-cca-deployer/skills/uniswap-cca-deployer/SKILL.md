---
name: uniswap-cca-deployer
description: "Deploy CCA (Continuous Clearing Auction) smart contracts using the Factory pattern"
version: "1.0.0"
author: "Uniswap Labs"
tags:
  - uniswap
  - cca
  - deployment
  - create2
---

# CCA Deployment

Deploy Continuous Clearing Auction (CCA) smart contracts using the ContinuousClearingAuctionFactory with CREATE2 for consistent addresses across chains.

## Overview

This skill guides AI agents through deploying CCA contracts via the canonical factory. It covers loading configuration, validating parameters, executing the deployment transaction, and post-deployment steps.

## Pre-flight Checks

1. Foundry (forge/cast) installed
2. A JSON configuration file from the CCA configurator skill
3. Deployer wallet funded with native gas token
4. RPC endpoint for the target chain

## Commands

### Deploy via Factory

The canonical factory address: `0xCCccCcCAE7503Cac057829BF2811De42E16e0bD5`

```bash
# Deploy using forge script
forge script DeployCCA \
  --rpc-url $RPC_URL \
  --account deployer \
  --broadcast
```

### Post-Deployment (CRITICAL)

After deployment, you MUST call `onTokensReceived()` to notify the auction:

```bash
cast send $AUCTION_ADDRESS "onTokensReceived()" \
  --rpc-url $RPC_URL \
  --account deployer
```

This is required before the auction can accept bids.

## Full Skill

For the complete deployment guide with validation checklists, Foundry script examples, and error recovery:

```
npx skills add Uniswap/uniswap-ai
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "CREATE2 collision" | Salt already used | Use a different salt value |
| Transaction reverts | Invalid configuration | Re-validate with configurator skill |
| "onTokensReceived" fails | Tokens not transferred | Transfer tokens to auction address first |

## Skill Routing

- For configuring auction parameters -> use `uniswap-cca-configurator`
- For viem/wagmi blockchain setup -> use `uniswap-viem-integration`
