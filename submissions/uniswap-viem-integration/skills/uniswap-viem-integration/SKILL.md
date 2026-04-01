---
name: uniswap-viem-integration
description: "Integrate EVM blockchains using viem and wagmi for TypeScript/JavaScript applications"
version: "1.0.0"
author: "Uniswap Labs"
tags:
  - viem
  - wagmi
  - ethereum
  - evm
---

# Viem Integration

Integrate EVM blockchains using viem for TypeScript/JavaScript applications.

## Overview

This skill provides comprehensive guidance for using viem (low-level EVM client) and wagmi (React hooks for Ethereum) to read blockchain data, send transactions, interact with smart contracts, and manage wallets.

## Pre-flight Checks

1. Node.js >= 18 installed
2. A TypeScript/JavaScript project initialized
3. An Ethereum RPC endpoint (Alchemy, Infura, or public)

## Commands

### Install Dependencies

```bash
npm install viem
# For React apps:
npm install wagmi viem @tanstack/react-query
```

### Create a Client

```typescript
import { createPublicClient, http } from "viem";
import { mainnet } from "viem/chains";

const client = createPublicClient({
  chain: mainnet,
  transport: http(),
});
```

### Read Contract Data

```typescript
const balance = await client.readContract({
  address: "0x...",
  abi: erc20Abi,
  functionName: "balanceOf",
  args: ["0x..."],
});
```

## Full Skill

For the complete guide with wallet integration, transaction signing, multi-chain patterns, and wagmi React hooks:

```
npx skills add Uniswap/uniswap-ai
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Could not detect network" | Invalid RPC URL | Verify the RPC endpoint is correct and reachable |
| "Insufficient funds" | Not enough ETH for gas | Fund the wallet with native gas token |

## Skill Routing

- For Uniswap swap integration -> use `uniswap-swap-integration`
- For v4 hook security -> use `uniswap-v4-security-foundations`
