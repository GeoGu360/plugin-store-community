---
name: rust-text-tool
description: "Text transformation CLI built in Rust — echo and version commands"
version: "1.0.0"
author: "yz06276"
tags:
  - rust
  - text-processing
---

# Rust Text Tool

## Overview

This skill provides a text echo CLI built in Rust. Use it to echo messages back with a Rust identifier prefix.

## Pre-flight Checks

1. The `rust-echo-cli` binary is installed (via `plugin-store install rust-text-tool`)

## Binary Tool Commands

### Echo a message

```bash
rust-echo-cli echo "Hello World"
```

**When to use**: When the user wants to echo text through the Rust tool.
**Output**: `Echo from Rust: Hello World`

### Show version

```bash
rust-echo-cli version
```

**When to use**: When the user wants to check the tool version.
**Output**: `rust-echo-cli 1.0.0`

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "unrecognized subcommand" | Invalid command | Use `echo` or `version` |
| Command not found | Binary not installed | Run `plugin-store install rust-text-tool` |
