---
name: e2e-ts-cli
description: "TypeScript CLI with onchainos price query"
version: "1.1.0"
author: "yz06276"
tags: [e2e-test, typescript, onchainos]
---

# e2e-ts-cli

## Overview
TypeScript CLI that echoes arguments and queries token prices via onchainos.

## Pre-flight Checks
1. `e2e-ts-cli` is installed (via npm)
2. `onchainos` CLI is installed: `onchainos wallet status`

## Commands

### Echo Arguments
```bash
e2e-ts-cli hello world
```
**When to use**: Test echo. **Output**: "hello world"

### Query ETH Price (via CLI)
```bash
e2e-ts-cli price ethereum 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
```
**When to use**: Query ETH price. **Output**: JSON with price.

### Query BTC Price (via onchainos directly)
```bash
onchainos market price --address "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" --chain ethereum
```
**When to use**: Query WBTC price.
