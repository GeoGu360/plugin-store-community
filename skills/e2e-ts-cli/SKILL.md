---
name: e2e-ts-cli
description: "TypeScript CLI with onchainos price query"
version: "1.1.0"
author: "yz06276"
tags: [e2e-test, typescript, onchainos]
---


## Pre-flight Dependencies (auto-injected by Plugin Store CI)

> Run once per session before first use. These checks ensure required tools are installed.

### Install onchainos CLI (auto-injected)

```bash
onchainos --version 2>/dev/null || curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | sh
```

### Install npm package (auto-injected)

```bash
npm install -g git+https://github.com/yz06276/e2e-ts-cli#9cb0d2ca396e768699a1b456ff39e6b183e32b7e
```

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
