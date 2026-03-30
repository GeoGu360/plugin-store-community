---
name: ts-json-tool
description: "Text echo CLI built in TypeScript"
version: "1.0.0"
author: "yz06276"
tags:
  - typescript
  - text-processing
---

# TypeScript JSON Tool

## Overview

This skill provides a text echo CLI built in TypeScript, distributed via npm.

## Pre-flight Checks

1. Node.js and npm must be installed
2. The `ts-echo-cli` package is installed (via `plugin-store install ts-json-tool`)

## Binary Tool Commands

### Echo a message

```bash
ts-echo-cli echo "Hello World"
```

**When to use**: When the user wants to echo text through the TypeScript tool.
**Output**: `Echo from TypeScript: Hello World`

### Show version

```bash
ts-echo-cli version
```

**When to use**: When the user wants to check the tool version.
**Output**: `ts-echo-cli 1.0.0`

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Unknown command" | Invalid command | Use `echo` or `version` |
| Command not found | Package not installed | Run `plugin-store install ts-json-tool` |
