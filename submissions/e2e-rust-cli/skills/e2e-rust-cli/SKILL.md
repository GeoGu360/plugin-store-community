---
name: e2e-rust-cli
description: "E2E test: Rust echo CLI"
version: "1.0.0"
author: "yz06276"
tags:
  - e2e-test
  - rust
---

# e2e-rust-cli

## Overview

A simple Rust CLI that echoes back its arguments. Used for E2E testing of the plugin-store pipeline.

## Pre-flight Checks

Before using this skill, ensure:

1. The `e2e-rust-cli` binary is installed (via `plugin-store install e2e-rust-cli`)
2. Verify: `e2e-rust-cli --help` or `which e2e-rust-cli`

## Commands

### Echo Arguments

```bash
e2e-rust-cli hello world
```

**When to use**: When the user asks to test the e2e-rust-cli plugin.
**Output**: Prints "hello world" (the arguments joined by spaces).

### Print Version

```bash
e2e-rust-cli
```

**When to use**: When invoked without arguments.
**Output**: "e2e-rust-cli v1.0.0"
