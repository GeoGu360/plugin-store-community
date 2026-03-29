---
name: ts-echo-cli
description: "Text processing CLI — JSON format, CSV parse, template interpolation, regex match, slug, trim"
version: "1.0.0"
author: "yz06276"
tags:
  - typescript
  - json
  - text-processing
---

# ts-echo-cli

## Overview

This skill provides a text processing CLI built in TypeScript. It can format JSON, parse CSV, interpolate templates, match regex patterns, slugify text, and trim whitespace.

## Pre-flight Checks

Before using this skill, ensure:

1. The `ts-echo-cli` binary is installed (via `plugin-store install ts-echo-cli`)

## Commands

### Echo text (plain)

```bash
ts-echo-cli "Hello World"
```

**When to use**: When the user wants to simply echo text back.

### JSON Pretty-Print

```bash
ts-echo-cli --mode json '{"name":"Alice","age":30}'
```

**When to use**: When the user wants to format/pretty-print a JSON string.
**Output**: Formatted JSON with indentation.

### CSV to JSON

```bash
ts-echo-cli --mode csv "name,age\nAlice,30\nBob,25"
```

**When to use**: When the user wants to convert CSV data to JSON objects.
**Output**: Array of objects with headers as keys.

### Template Interpolation

```bash
ts-echo-cli --mode template --vars '{"name":"Alice"}' "Hello {{name}}!"
```

**When to use**: When the user wants to fill in template variables.
**Output**: `Hello Alice!`

### Regex Match

```bash
ts-echo-cli --mode regex --pattern "\d+" "There are 42 cats and 7 dogs"
```

**When to use**: When the user wants to find regex matches in text.
**Output**: List of all matches.

### Slugify

```bash
ts-echo-cli --mode slug "Hello World! This is a Test"
```

**When to use**: When the user wants to convert text to a URL-safe slug.
**Output**: `hello-world-this-is-a-test`

### Trim

```bash
ts-echo-cli --mode trim "  Hello World  "
```

**When to use**: When the user wants to strip leading/trailing whitespace.
**Output**: `Hello World`

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Invalid JSON" | Malformed JSON input | Check JSON syntax |
| "Unknown mode" | Invalid --mode value | Use one of: json, csv, template, regex, slug, trim |
| "No input provided" | Missing text argument | Provide text as a positional argument |
