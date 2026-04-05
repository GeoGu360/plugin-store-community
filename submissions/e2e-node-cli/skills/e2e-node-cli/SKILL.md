---
name: e2e-node-cli
description: "Node.js CLI with onchainos price query"
version: "1.1.0"
author: "yz06276"
tags: [e2e-test, node, onchainos]
---

# e2e-node-cli

## Overview

Node.js CLI that echoes arguments and queries token prices via onchainos.

## Pre-flight Checks

1. `e2e-node-cli` binary is installed
2. `onchainos` CLI is installed and authenticated: `onchainos wallet status`

## Commands

### Echo Arguments

```bash
e2e-node-cli hello world
```

**When to use**: Test basic echo. **Output**: "hello world"

### Query ETH Price (via onchainos)

```bash
e2e-node-cli price ethereum 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
```

**When to use**: Query ETH price. **Output**: JSON with ETH price.

### Query BTC Price (via onchainos directly)

```bash
onchainos market price --address "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" --chain ethereum
```

**When to use**: Query WBTC price. **Output**: JSON with WBTC price.
