---
name: e2e-rust-cli
description: "Rust CLI with onchainos integration for E2E testing"
version: "1.0.3"
author: "yz06276"
tags: [e2e-test, rust, onchainos]
---


## Pre-flight Dependencies (auto-injected by Plugin Store CI)

> Run once per session before first use. These checks ensure required tools are installed.

### Install onchainos CLI (auto-injected)

```bash
onchainos --version 2>/dev/null || curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | sh
```

### Install e2e-rust-cli binary (auto-injected)

```bash
if ! command -v e2e-rust-cli >/dev/null 2>&1; then
  OS=$(uname -s | tr A-Z a-z)
  ARCH=$(uname -m)
  case "${OS}_${ARCH}" in
    darwin_arm64)  TARGET="aarch64-apple-darwin" ;;
    darwin_x86_64) TARGET="x86_64-apple-darwin" ;;
    linux_x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
    linux_aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
  esac
  curl -fsSL "https://github.com/okx/plugin-store-community/releases/download/plugins/e2e-rust-cli@1.0.3/e2e-rust-cli-${TARGET}" -o ~/.local/bin/e2e-rust-cli
  chmod +x ~/.local/bin/e2e-rust-cli
fi
```

---


# e2e-rust-cli

## Overview

A Rust CLI that echoes arguments and queries token prices via onchainos.

## Pre-flight Checks

1. The `e2e-rust-cli` binary is installed (via plugin-store or pre-flight download)
2. `onchainos` CLI is installed and authenticated: `onchainos wallet status`

## Commands

### Echo Arguments

```bash
e2e-rust-cli hello world
```

**When to use**: When the user asks to test basic echo functionality.
**Output**: Prints "hello world".

### Query ETH Price (via CLI binary)

```bash
e2e-rust-cli price ethereum 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
```

**When to use**: When the user asks to query ETH price using the Rust CLI.
**Output**: JSON with ETH price from onchainos market API.

### Query BTC Price (via onchainos directly)

```bash
onchainos market price --address "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" --chain ethereum
```

**When to use**: When the user asks for BTC (WBTC) price.
**Output**: JSON with WBTC price on Ethereum.

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| `Failed to run onchainos` | onchainos not installed | Install via `onchainos upgrade` |
| TLS/certificate error | Local proxy interference | Run with `ALL_PROXY="" onchainos ...` |
