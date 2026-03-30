---
name: go-hash-tool
description: "Text echo CLI built in Go"
version: "1.0.0"
author: "yz06276"
tags:
  - go
  - hashing
---

# Go Hash Tool

## Overview

This skill provides a text echo CLI built in Go. Use it to echo messages back with a Go identifier prefix.

## Pre-flight Checks

1. The `go-echo-cli` binary is installed (via `plugin-store install go-hash-tool`)

## Binary Tool Commands

### Echo a message

```bash
go-echo-cli echo "Hello World"
```

**When to use**: When the user wants to echo text through the Go tool.
**Output**: `Echo from Go: Hello World`

### Show version

```bash
go-echo-cli version
```

**When to use**: When the user wants to check the tool version.
**Output**: `go-echo-cli 1.0.0`

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "unknown command" | Invalid command | Use `echo` or `version` |
| Command not found | Binary not installed | Run `plugin-store install go-hash-tool` |
