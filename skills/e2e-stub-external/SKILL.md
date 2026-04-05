---
name: e2e-stub-external
description: "E2E test: external stub with Python demo script"
version: "1.0.0"
author: "yz06276"
tags:
  - e2e-test
  - external
---

# e2e-stub-external

## Overview

An external plugin in Claude marketplace format. Contains a Python demo script.

## Pre-flight Checks

Before using this skill, ensure:

1. Python 3 is installed: `python3 --version`
2. Scripts are available in the skill directory

## Commands

### Run Demo

```bash
python3 scripts/demo.py
```

**When to use**: When the user asks to test the e2e-stub-external plugin.
**Output**: "Hello from stub-external!"
