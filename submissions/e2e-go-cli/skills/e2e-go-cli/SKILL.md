---
name: e2e-go-cli
description: "E2E test: Go echo CLI"
version: "1.0.0"
author: "yz06276"
tags:
  - e2e-test
  - go
---

# e2e-go-cli

## Overview

A simple Go CLI that echoes back its arguments. Used for E2E testing of the plugin-store pipeline.

## Pre-flight Checks

Before using this skill, ensure:

1. The `e2e-go-cli` binary is installed (via `plugin-store install e2e-go-cli`)
2. Verify: `e2e-go-cli --help` or `which e2e-go-cli`

## Commands

### Echo Arguments

```bash
e2e-go-cli hello world
```

**When to use**: When the user asks to test the e2e-go-cli plugin.
**Output**: Prints "hello world" (the arguments joined by spaces).

### Print Version

```bash
e2e-go-cli
```

**When to use**: When invoked without arguments.
**Output**: "e2e-go-cli v1.0.0"
