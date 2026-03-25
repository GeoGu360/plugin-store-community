# Plugin Development & Submission Guide

> This guide walks you through developing a plugin for the Plugin Store ecosystem and submitting it for review. By the end, you will have a working plugin that integrates with the onchainos CLI.

---

## Table of Contents

1. [What is a Plugin?](#1-what-is-a-plugin)
2. [Before You Start](#2-before-you-start)
3. [Step 1: Scaffold Your Plugin](#3-step-1-scaffold-your-plugin)
4. [Step 2: Write plugin.yaml](#4-step-2-write-pluginyaml)
5. [Step 3: Write SKILL.md](#5-step-3-write-skillmd)
6. [Step 4: Declare Chains and API Calls](#6-step-4-declare-chains-and-api-calls)
7. [Step 5: Local Validation](#7-step-5-local-validation)
8. [Step 6: Submit via Pull Request](#8-step-6-submit-via-pull-request)
9. [What Happens After Submission](#9-what-happens-after-submission)
10. [Updating Your Plugin](#10-updating-your-plugin)
11. [Rules & Restrictions](#11-rules--restrictions)
12. [SKILL.md Writing Guide](#12-skillmd-writing-guide)
13. [Submitting Plugins with Source Code (MCP/Binary)](#13-submitting-plugins-with-source-code-mcpbinary)
14. [onchainos Command Reference](#14-onchainos-command-reference)
15. [FAQ](#15-faq)

---

## 1. What is a Plugin?

A plugin has one required core: **SKILL.md** — a markdown document that teaches AI agents how to perform on-chain tasks. Optionally, it can also include an **MCP Server** or **Binary** (compiled from your source code by our CI).

**SKILL.md is always the entry point.** Even if your plugin includes an MCP server, the Skill tells the AI agent what tools are available and when to use them.

### Two types of plugins

```
Type A: Skill-Only (most common, any developer)
────────────────────────────────────────────────
  SKILL.md → instructs AI → calls onchainos CLI

Type B: Skill + Binary (any developer, source code compiled by our CI)
────────────────────────────────────────────────
  SKILL.md → instructs AI → calls onchainos CLI
                           + calls your MCP tools
                           + runs your binary commands

  Your source code (in your GitHub repo)
    → our CI compiles it
    → users install our compiled artifact
```

Choose your path before starting:

| I want to... | Type |
|---------------|------|
| Create a strategy using onchainos commands | Skill-Only |
| Ship a CLI tool alongside a Skill | Skill + Binary (submit source code, we compile) |

---

## 2. Before You Start

### Prerequisites

- **Git** and a **GitHub account**
- **plugin-store CLI** installed:
  ```bash
  # macOS / Linux
  curl -fsSL https://raw.githubusercontent.com/yz06276/plugin-store/main/install-local.sh | bash
  ```
- **onchainos CLI** installed (for testing your commands):
  ```bash
  curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | bash
  ```
- Basic understanding of the blockchain/DeFi domain your plugin covers

### Key Rule

> **All plugins must use onchainos CLI for on-chain operations.** You cannot implement your own price queries, wallet signing, transaction building, or RPC calls. onchainos provides all of these — your plugin tells the AI agent how and when to use them.

---

## 3. Step 1: Scaffold Your Plugin

```bash
plugin-store init my-awesome-plugin
```

This generates a standard directory for a Skill-only plugin:

```
my-awesome-plugin/
├── plugin.yaml                        # Plugin manifest (you fill this in)
├── skills/
│   └── my-awesome-plugin/
│       ├── SKILL.md                   # Skill definition (you write this)
│       └── references/
│           └── cli-reference.md       # CLI reference docs (you write this)
├── LICENSE                            # MIT license template
├── CHANGELOG.md                       # Version history
└── README.md                          # Plugin description
```

**If you're building a Skill + MCP/Binary plugin**, you also need:
- Source code in your own GitHub repo (we compile it, you don't submit binaries)
- A `build` section in plugin.yaml pointing to your repo + commit SHA

---

## 4. Step 2: Write plugin.yaml

This is your plugin's manifest. It tells the Plugin Store what your plugin is, who wrote it, and what it can do.

### 4A. Skill-Only Example

```yaml
schema_version: 1
name: sol-price-checker              # Lowercase, hyphens only, 2-40 chars
alias: "Solana Price Checker"        # Optional: display name
version: "1.0.0"                     # Semantic versioning (x.y.z)
description: "Query real-time token prices on Solana with market data and trend analysis"
author:
  name: "Your Name"
  github: "your-github-username"     # Must match PR author
  email: "you@example.com"          # Optional: for security notifications
license: MIT
category: analytics                  # See categories below
tags:
  - solana
  - price
  - analytics

components:
  skill:
    dir: skills/sol-price-checker    # Path to your SKILL.md directory

chains:
  - solana

api_calls: []

extra:
  protocols: []                      # e.g. [uniswap-v3, raydium]
  risk_level: low                    # low | medium | high
```

### 4B. Skill + Binary Example (with source code compilation)

If your plugin includes an MCP server or binary, you need a `build` section. Your source code stays in your own GitHub repo — we compile it.

```yaml
schema_version: 2
name: defi-yield-optimizer
version: "1.0.0"
description: "Optimize DeFi yield across protocols with custom analytics"
author:
  name: "DeFi Builder"
  github: "defi-builder"
license: MIT
category: defi-protocol
tags: [defi, yield]

components:
  skill:
    dir: skills/defi-yield-optimizer   # SKILL.md — always required, the entry point
  mcp:
    type: binary
    command: defi-yield-mcp            # Name of compiled binary
    args: ["--stdio"]
    env: [DEFI_API_KEY]

build:
  lang: rust                            # rust | go | typescript | node | python
  source_repo: "defi-builder/yield-mcp" # Your GitHub repo with source code
  source_commit: "a1b2c3d4e5f6..."      # Full 40-char commit SHA (pinned)
  source_dir: "."                       # Path within repo (default: root)
  binary_name: defi-yield-mcp           # Compiled output name

chains:
  - ethereum
  - base

api_calls:
  - "api.defillama.com"

extra:
  protocols: [morpho, aave]
  risk_level: medium
```

**Key differences from Skill-Only:**
- `schema_version: 2` (not 1)
- `components.mcp` or `components.binary` declared
- `build` section with `source_repo` + `source_commit`
- Our CI clones your repo at the exact commit, compiles, and publishes

**How to get the commit SHA:**
```bash
cd your-source-repo
git rev-parse HEAD
# Output: a1b2c3d4e5f6789012345678901234567890abcd
```

### Field Reference

| Field | Required | Rules |
|-------|----------|-------|
| `name` | Yes | Lowercase, `[a-z0-9-]`, 2-40 chars, no consecutive hyphens |
| `version` | Yes | Semantic versioning: `x.y.z` |
| `description` | Yes | One line, under 200 characters recommended |
| `author.name` | Yes | Your name or organization |
| `author.github` | Yes | Your GitHub username (must match PR author) |
| `license` | Yes | SPDX identifier: MIT, Apache-2.0, GPL-3.0, etc. |
| `category` | Yes | One of: `trading-strategy`, `defi-protocol`, `analytics`, `utility`, `security`, `wallet`, `nft` |
| `tags` | No | Keywords for search |
| `components.skill.dir` | Yes | Relative path to the directory containing SKILL.md |
| `chains` | No | List of blockchains the plugin operates on (informational) |
| `api_calls` | No | List of external API domains the plugin calls (reviewer reference; lint checks against this) |
| `extra.risk_level` | No | `low`, `medium`, or `high` |

### Naming Rules

- Allowed: `solana-price-checker`, `defi-yield-optimizer`, `nft-tracker`
- Forbidden: `OKX-Plugin` (reserved prefix), `my_plugin` (underscores), `a` (too short)
- Reserved prefixes: `okx-`, `official-`, `plugin-store-`

---

## 5. Step 3: Write SKILL.md

SKILL.md is the **single entry point** of your plugin. It teaches the AI agent what your plugin does and how to use it. For Skill-only plugins, it orchestrates onchainos commands. For MCP/Binary plugins, it also orchestrates your custom tools.

```
Skill-Only plugin:
  SKILL.md → onchainos commands

MCP/Binary plugin:
  SKILL.md → onchainos commands
           + your MCP tools (calculate_yield, find_route, ...)
           + your binary commands (my-tool start, my-tool status, ...)
```

### 5A. Template (Skill-Only)

```markdown
---
name: my-awesome-plugin
description: "Brief description of what this skill does"
version: "1.0.0"
author: "Your Name"
tags:
  - keyword1
  - keyword2
---

# My Awesome Plugin

## Overview

[2-3 sentences: what does this skill enable the AI agent to do?]

## Pre-flight Checks

Before using this skill, ensure:

1. The `onchainos` CLI is installed and configured
2. [Any other prerequisites]

## Commands

### [Command Name]

\`\`\`bash
onchainos <command> <subcommand> --flag value
\`\`\`

**When to use**: [Describe when the AI should use this command]
**Output**: [Describe what the command returns]
**Example**: [Show a concrete example with real values]

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| "Token not found" | Invalid token symbol | Ask user to verify the token name |
| "Rate limited" | Too many requests | Wait 10 seconds and retry |

## Skill Routing

- For token swaps → use `okx-dex-swap` skill
- For wallet balances → use `okx-wallet-portfolio` skill
- For security scanning → use `okx-security` skill
```

### 5B. Template (MCP/Binary Plugin)

If your plugin includes an MCP server, the SKILL.md must tell the AI agent about **both** onchainos commands and your custom MCP tools:

```markdown
---
name: defi-yield-optimizer
description: "Optimize DeFi yield with custom analytics and onchainos execution"
version: "1.0.0"
author: "DeFi Builder"
tags:
  - defi
  - yield
---

# DeFi Yield Optimizer

## Overview

This plugin combines custom yield analytics (via MCP tools) with
onchainos execution capabilities to find and enter the best DeFi positions.

## Pre-flight Checks

1. The `onchainos` CLI is installed and configured
2. The defi-yield-mcp MCP server is installed via plugin-store
3. A valid DEFI_API_KEY environment variable is set

## MCP Tools (provided by this plugin)

### calculate_yield
Calculate the projected APY for a specific DeFi pool.
**Parameters**: pool_address (string), chain (string)
**Returns**: APY percentage, TVL, risk score

### find_best_route
Find the optimal swap route to enter a DeFi position.
**Parameters**: from_token (string), to_token (string), amount (number)
**Returns**: Route steps, estimated output, price impact

## Commands (using onchainos + MCP tools together)

### Find Best Yield

1. Call MCP tool `calculate_yield` for the target pool
2. Run `onchainos token info --address <pool_token> --chain <chain>`
3. Present yield rate + token info to user

### Enter Position

1. Call MCP tool `find_best_route` for the swap
2. Run `onchainos swap quote --from <token> --to <pool_token> --amount <amount>`
3. **Ask user to confirm** the swap amount and expected yield
4. Run `onchainos swap swap ...` to execute
5. Report result to user

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| MCP connection failed | Server not running | Run `plugin-store install defi-yield-optimizer` |
| "Pool not found" | Invalid pool address | Verify the pool contract address |
| "Insufficient balance" | Not enough tokens | Check balance with `onchainos portfolio all-balances` |

## Skill Routing

- For token swaps only → use `okx-dex-swap` skill
- For security checks → use `okx-security` skill
```

### SKILL.md Best Practices

1. **Be specific** — "Run `onchainos token search --query SOL --chain solana`" is better than "search for tokens"
2. **Always include error handling** — The AI agent needs to know what to do when things fail
3. **Use skill routing** — Tell the AI when to defer to other skills instead of trying to handle everything
4. **Include pre-flight checks** — What conditions must be met before using your skill
5. **Don't duplicate onchainos capabilities** — Your skill should orchestrate onchainos commands, not replace them

---

## 6. Step 4: Declare Chains and API Calls

The only declarations you need are `chains` and `api_calls` — both are top-level fields in plugin.yaml. Actual permissions (wallet access, transaction signing, etc.) are auto-detected by the AI review during submission.

```yaml
chains:
  - solana
  - ethereum

api_calls:
  - "api.defillama.com"
```

- **`chains`** — list of blockchains your plugin operates on (informational).
- **`api_calls`** — list of external API domains your plugin calls. The linter checks that any URLs in your SKILL.md match this list.

---

## 7. Step 5: Local Validation

Before submitting, validate your plugin locally:

```bash
plugin-store lint ./my-awesome-plugin/
```

### If everything passes:

```
Linting ./my-awesome-plugin/...

✓ Plugin 'my-awesome-plugin' passed all checks!
```

### If there are errors:

```
Linting ./my-awesome-plugin/...

  ❌ [E031] name 'My-Plugin' must be lowercase alphanumeric with hyphens only
  ❌ [E065] chains or api_calls field is required
  ⚠️  [W091] SKILL.md frontmatter missing recommended field: description

✗ Plugin 'My-Plugin': 2 error(s), 1 warning(s)
```

Fix all errors (❌) before submitting. Warnings (⚠️) are advisory.

### Common Lint Errors

| Code | Meaning | Fix |
|------|---------|-----|
| E001 | plugin.yaml not found | Ensure plugin.yaml is in the root of your submission directory |
| E031 | Invalid name format | Use lowercase letters, numbers, and hyphens only |
| E033 | Reserved prefix | Don't start your name with `okx-`, `official-`, or `plugin-store-` |
| E035 | Invalid version | Use semantic versioning: `1.0.0`, not `1.0` or `v1.0.0` |
| E041 | Missing LICENSE | Add a LICENSE file to your submission directory |
| E052 | Missing SKILL.md | Ensure SKILL.md exists in the path specified by `components.skill.dir` |
| E065 | Missing chains/api_calls | Add `chains` and/or `api_calls` fields to plugin.yaml |
| E110 | MCP not allowed | Community plugins cannot include MCP components |
| E111 | Binary not allowed | Community plugins cannot include Binary components |

---

## 8. Step 6: Submit via Pull Request

### 1. Clone the community repository

```bash
git clone git@github.com:yz06276/plugin-store-community.git
cd plugin-store-community
```

### 2. Create a branch and add your plugin

```bash
git checkout -b submit/my-awesome-plugin
cp -r /path/to/my-awesome-plugin submissions/my-awesome-plugin
```

### 3. Verify the directory structure

```
submissions/
  my-awesome-plugin/
    plugin.yaml
    skills/
      my-awesome-plugin/
        SKILL.md
        references/
          cli-reference.md
    LICENSE
    CHANGELOG.md
    README.md
```

### 4. Commit and push

```bash
git add submissions/my-awesome-plugin/
git commit -m "[new-plugin] my-awesome-plugin v1.0.0"
git push origin submit/my-awesome-plugin
```

### 5. Open a Pull Request

Go to GitHub and create a Pull Request from your branch. Use this title format:

```
[new-plugin] my-awesome-plugin v1.0.0
```

The PR template will guide you through the checklist.

### Important Rules for PRs

- Each PR should contain **one plugin only**
- Only modify files inside `submissions/your-plugin-name/`
- Do not modify any other files (README.md, workflows, etc.)
- The directory name must match the `name` field in plugin.yaml

---

## 9. What Happens After Submission

### Automated Checks (~5 minutes)

```
Phase 2: Structure Validation (lint)
  → Checks all 15+ rules automatically
  → Posts results as a PR comment
  → If failed: PR is blocked, fix and push again

Phase 3: AI Code Review (Claude)
  → Reads your plugin + latest onchainos source code
  → Generates an 8-section review report
  → Posts report as a PR comment (collapsible sections)
  → Advisory only — does NOT block merge
```

### Human Review (1-3 days)

A maintainer reviews:

- Does the plugin make sense?
- Are chains and api_calls accurate?
- Is the SKILL.md well-written?
- Any security concerns?

### After Merge

Your plugin is automatically:

1. Added to `registry.json` in the main plugin-store repo
2. Tagged with `plugins/my-awesome-plugin@1.0.0`
3. Available to all users via `plugin-store install my-awesome-plugin`

---

## 10. Updating Your Plugin

### Content Update (SKILL.md changes, new commands)

1. Modify files in `submissions/my-awesome-plugin/`
2. Bump `version` in plugin.yaml (e.g., `1.0.0` → `1.1.0`)
3. Update CHANGELOG.md
4. Open a PR with title: `[update] my-awesome-plugin v1.1.0`

### Chain or API Change (requires full review)

If your update changes `chains` or `api_calls`, the review will be more thorough. The AI review report will highlight these changes.

---

## 11. Rules & Restrictions

### What Community Plugins CAN Do

- Define skills using SKILL.md
- Reference any onchainos CLI command
- Include reference documentation
- Declare chains and api_calls

### What Community Plugins CANNOT Do

- Include MCP server components (code execution)
- Include binary components (code execution)
- Use reserved name prefixes (`okx-`, `official-`, `plugin-store-`)
- Bypass onchainos CLI (use direct RPC, external price APIs, web3 libraries)
- Include prompt injection patterns
- Exceed file size limits (100KB per file, 1MB total)

### What Any Developer Can Submit

| Component | How |
|-----------|-----|
| Skill (SKILL.md) | Include in submissions/ directory |
| Binary | Submit source code in your GitHub repo, we compile it (requires `build` section) |

---

## 12. SKILL.md Writing Guide

### Structure Checklist

- [ ] YAML frontmatter with `name` and `description`
- [ ] Overview section (what does this skill do?)
- [ ] Pre-flight checks (what's needed before use?)
- [ ] Commands section (each onchainos command with when/how/output)
- [ ] Error handling table
- [ ] Skill routing (when to defer to other skills)

### Good vs Bad Examples

**Bad: vague instructions**
```
Use onchainos to get the price.
```

**Good: specific and actionable**
```
To get the current price of a Solana token:

\`\`\`bash
onchainos market price --address <TOKEN_ADDRESS> --chain solana
\`\`\`

**When to use**: When the user asks "what's the price of [token]?" on Solana.
**Output**: Current price in USD, 24h change percentage, 24h volume.
**If the token is not found**: Ask the user to verify the contract address or try `onchainos token search --query <NAME> --chain solana` first.
```

---

## 13. Submitting Plugins with Source Code (MCP/Binary)

> **Important:** SKILL.md is always the entry point. Even if your plugin includes an MCP server or binary, the SKILL.md is what tells the AI agent how to orchestrate everything — onchainos commands, your MCP tools, and your binary commands.

### Who Can Submit Source Code?

Any developer can submit source code for binary compilation. Submit your source code in your own GitHub repo, add a `build` section to plugin.yaml, and our CI will compile it.

### How It Works

```
You submit source code → Our CI compiles it → Users install our compiled artifact
You never submit binaries. We never modify your source code.
```

### plugin.yaml with build config

Your source code stays in your own GitHub repo. You provide the repo URL and a pinned commit SHA — our CI clones at that exact commit, compiles, and publishes. The commit SHA is the content fingerprint: same SHA = same code, guaranteed.

```yaml
schema_version: 2
name: my-mcp-server
version: "1.0.0"
description: "My custom MCP server"
author:
  name: "Your Name"
  github: "your-username"
license: MIT
category: defi-protocol
tags: [defi]

components:
  skill:
    dir: skills/my-mcp-server       # SKILL.md is ALWAYS required
  mcp:
    type: binary
    command: my-mcp-server
    args: ["--stdio"]
    env: [API_KEY]

build:
  lang: rust                          # rust | go | typescript | node | python
  source_repo: "your-username/my-mcp-server"  # Your GitHub repo with source code
  source_commit: "abc123def456..."    # Full 40-char commit SHA (pinned)
  source_dir: "."                     # Path within repo (default: root)
  binary_name: my-mcp-server         # Name of the compiled output
  # main: src/index.ts               # Required for typescript/python
  # npm_scope: "@plugin-store"       # Required for node

chains:
  - ethereum

api_calls: []
```

### How to get the commit SHA

```bash
# In your source code repo, after pushing your code:
git rev-parse HEAD
# Output: 342756ee25405b5ec5b375a37c1b36710d5b9cd6
# Copy this full 40-character string into build.source_commit
```

### Directory Structure

Source code lives in your own repo. You only submit metadata + SKILL to the community repo:

```
submissions/my-mcp-server/            ← In community repo (small, ~20KB)
  plugin.yaml                         # With build section pointing to your repo
  skills/my-mcp-server/
    SKILL.md                          # The AI agent's entry point
    references/
  LICENSE
  CHANGELOG.md
  README.md

your-username/my-mcp-server           ← Your own GitHub repo (source code)
  Cargo.toml                          # (Rust example)
  src/
    main.rs
    lib.rs
```

### Supported Languages

| Language | Entry File | Build Tool | Output |
|----------|-----------|------------|--------|
| Rust | `Cargo.toml` | `cargo build --release` | Native binary |
| Go | `go.mod` | `go build` | Native binary |
| TypeScript | `package.json` + `build.main` | `bun build --compile` | Bundled binary |
| Node.js | `package.json` | `npm publish` | npm package |
| Python | `pyproject.toml` + `build.main` | `PyInstaller` | Bundled binary |

### SKILL.md as the Orchestrator

Your SKILL.md tells the AI agent how to use **both** onchainos commands and your custom MCP tools:

```markdown
## Commands

### Check Yield (uses your MCP tool)
Call MCP tool `calculate_yield` with pool address and chain.

### Execute Deposit (uses onchainos + your MCP)
1. Call MCP tool `find_best_route` for the chosen pool
2. Run `onchainos swap quote --from USDC --to POOL_TOKEN`
3. **Ask user to confirm** amount and expected yield
4. Run `onchainos swap swap ...` to execute
5. Call MCP tool `monitor_position` to start tracking
```

### What You Cannot Do

- Submit pre-compiled binaries (.exe, .dll, .so, .wasm) — E130
- Declare MCP/Binary without a build section — E110/E111
- Have source code larger than 10MB — E126
- Include build scripts that download from the internet during compilation

---

## 14. onchainos Command Reference

Your SKILL.md should only use onchainos CLI commands. Here are the available top-level commands:

| Command | Description | Example |
|---------|-------------|---------|
| `onchainos token` | Token search, info, trending, holders | `onchainos token search --query SOL` |
| `onchainos market` | Price, kline charts, portfolio PnL | `onchainos market price --address 0x... --chain ethereum` |
| `onchainos swap` | DEX swap quotes and execution | `onchainos swap quote --from ETH --to USDC --amount 1` |
| `onchainos gateway` | Gas estimation, tx simulation, broadcast | `onchainos gateway gas --chain ethereum` |
| `onchainos portfolio` | Wallet total value and balances | `onchainos portfolio all-balances --address 0x...` |
| `onchainos wallet` | Login, balance, send, history | `onchainos wallet balance --chain solana` |
| `onchainos security` | Token scan, dapp scan, tx scan | `onchainos security token-scan --address 0x...` |
| `onchainos signal` | Smart money / whale signals | `onchainos signal list --chain solana` |
| `onchainos memepump` | Meme token scanning and analysis | `onchainos memepump tokens --chain solana` |
| `onchainos leaderboard` | Top traders by PnL/volume | `onchainos leaderboard list --chain solana` |
| `onchainos payment` | x402 payment protocol | `onchainos payment x402-pay --url ...` |

For the full subcommand list, run `onchainos <command> --help` or see the [onchainos documentation](https://github.com/okx/onchainos-skills).

---

## 15. FAQ

**Q: Can I submit a plugin that calls external APIs directly?**
A: No. All on-chain operations must go through onchainos CLI. If you need a capability that onchainos doesn't provide, open a feature request in the onchainos repo.

**Q: Can I include a binary or MCP server?**
A: Yes. Any developer can submit binary source code. Keep your source in your own GitHub repo and add a `build` section to plugin.yaml with `source_repo` and `source_commit`. Our CI compiles it. See Section 13 for details.

**Q: How long does the review take?**
A: Automated checks complete in ~5 minutes. Human review typically takes 1-3 business days.

**Q: What if the AI review flags something?**
A: The AI review is advisory only — it does not block your PR. However, human reviewers will read the AI report. Address any issues flagged to speed up approval.

**Q: Can I update my plugin after it's published?**
A: Yes. Submit a new PR with updated files and a bumped version number.

**Q: How do I become a Verified Publisher?**
A: After 5+ approved plugin submissions, you can apply for Verified Publisher status. This unlocks elevated permissions and faster review.

**Q: My `plugin-store lint` passes but the GitHub check fails. Why?**
A: Make sure you're running the latest version of the plugin-store CLI. Also ensure your PR only modifies files within `submissions/your-plugin-name/`.

**Q: What does error E122 "source_repo is not valid" mean?**
A: `build.source_repo` must be in `owner/repo` format (e.g. `your-username/my-server`). Don't include `https://github.com/` or `.git`.

**Q: What does error E123 "must be a full 40-character hex SHA" mean?**
A: `build.source_commit` must be the full commit hash, not a short SHA or branch name. Run `git rev-parse HEAD` in your source repo to get the full 40-character hash.

**Q: What does error E120 "must also include a Skill component" mean?**
A: Every plugin with a `build` section must also have a SKILL.md. The Skill is the entry point — it tells the AI agent how to use your MCP server or binary.

**Q: What does error E130 "pre-compiled binary file is not allowed" mean?**
A: You submitted a compiled file (.exe, .dll, .so, .wasm, etc.) in your submission directory. Remove it — we compile from your source code, you don't submit binaries.

**Q: What does error E110/E111 "requires a build section" mean?**
A: You declared an MCP or Binary component but didn't include a `build` section. We need to know where your source code is so we can compile it. Add `build.lang`, `build.source_repo`, and `build.source_commit`.

**Q: The build failed in CI but I can compile locally. Why?**
A: Our CI compiles on Ubuntu Linux. Ensure your code builds on Linux, not just macOS/Windows. Check the build logs in the GitHub Actions run for specific errors.
