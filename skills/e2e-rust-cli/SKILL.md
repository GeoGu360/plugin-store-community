---
name: e2e-rust-cli
description: "Rust echo CLI for E2E testing"
version: "1.0.0"
author: "yz06276"
tags: [e2e-test, rust]
---


## Pre-flight Dependencies (auto-injected by Plugin Store CI)

> Run once per session before first use. These checks ensure required tools are installed.

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
  curl -fsSL "https://github.com/okx/plugin-store-community/releases/download/community/e2e-rust-cli@1.0.0/e2e-rust-cli-${TARGET}" -o ~/.local/bin/e2e-rust-cli
  chmod +x ~/.local/bin/e2e-rust-cli
fi
```

---

# e2e-rust-cli
## Overview
A simple Rust CLI that echoes back its arguments.
## Pre-flight Checks
1. The `e2e-rust-cli` binary is installed
2. Verify: `which e2e-rust-cli`
## Commands
### Echo
```bash
e2e-rust-cli hello
```
**When to use**: Test the e2e-rust-cli plugin.
**Output**: Prints "hello".
