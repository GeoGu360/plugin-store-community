# Plugin Development & Submission Guide

> This guide walks you through developing a plugin for the Plugin Store ecosystem and submitting it for review. By the end, you will have a working plugin that integrates with the onchainos CLI.

---

## Table of Contents

1. [What is a Plugin?](#1-what-is-a-plugin)
2. [Before You Start](#2-before-you-start)
3. [Step 1: Fork, Clone, and Scaffold Your Plugin](#3-step-1-scaffold-your-plugin)
4. [Step 2: Write plugin.yaml](#4-step-2-write-pluginyaml)
5. [Step 3: Write SKILL.md](#5-step-3-write-skillmd)
6. [Step 4: Declare API Calls](#6-step-4-declare-api-calls)
7. [Step 5: Local Validation](#7-step-5-local-validation)
8. [Step 6: Submit via Pull Request](#8-step-6-submit-via-pull-request)
8a. [Alternative: Submit Source Code Directly (Mode A)](#8a-alternative-submit-source-code-directly-mode-a)
8b. [Alternative: Submit via External Repository (Mode B)](#8b-alternative-submit-via-external-repository-mode-b)
8c. [Alternative: One-Click Import (Mode C)](#8c-alternative-one-click-import-mode-c)
9. [What Happens After Submission](#9-what-happens-after-submission)
10. [Updating Your Plugin](#10-updating-your-plugin)
11. [Rules & Restrictions](#11-rules--restrictions)
12. [SKILL.md Writing Guide](#12-skillmd-writing-guide)
13. [Submitting Plugins with Source Code (Binary)](#13-submitting-plugins-with-source-code-binary)
14. [onchainos Command Reference](#14-onchainos-command-reference)
15. [FAQ](#15-faq)
16. [Getting Help](#16-getting-help)

---

## 1. What is a Plugin?

A plugin has one required core: **SKILL.md** — a markdown document that teaches AI agents how to perform on-chain tasks. Optionally, it can also include a **Binary** (compiled from your source code by our CI).

**SKILL.md is always the entry point.** Even if your plugin includes a binary, the Skill tells the AI agent what tools are available and when to use them.

### On-chain operations: use onchainOS

All plugins that interact with the blockchain **must** use the [onchainOS Agentic Wallet](https://github.com/okx/onchainos-skills) for on-chain operations — wallet signing, transaction broadcasting, swap execution, contract calls, and any action that writes to the blockchain.

```
✅ Allowed — query any data source you want:
  Third-party DeFi APIs (DeFiLlama, Birdeye, DexScreener...)
  Market data providers, analytics services
  Your own backend APIs

❌ Must use onchainOS — all on-chain write operations:
  Wallet signing        → onchainos wallet send / sign
  Transaction broadcast → onchainos gateway broadcast
  Swap execution        → onchainos swap swap
  Contract calls        → onchainos wallet contract-call
  Token approvals       → onchainos swap approve
```

> Plugins that use third-party wallets (MetaMask, Phantom, etc.) or direct blockchain RPC calls (ethers.js, web3.js, etc.) for on-chain operations **will be rejected**. See the [onchainOS documentation](https://github.com/okx/onchainos-skills) for all available capabilities.

### Two types of plugins

```
Type A: Skill-Only (most common, any developer)
────────────────────────────────────────────────
  SKILL.md → instructs AI → calls onchainos CLI
                           + queries external data (free)

Type B: Skill + Binary (any developer, source code compiled by our CI)
────────────────────────────────────────────────
  SKILL.md → instructs AI → calls onchainos CLI
                           + calls your binary tools
                           + queries external data (free)

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
- **onchainos CLI** installed (for testing your commands):
  ```bash
  curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | bash
  ```
  After installation, if `onchainos` is not found, add it to your PATH:
  ```bash
  export PATH="$HOME/.local/bin:$PATH"
  ```
- Basic understanding of the blockchain/DeFi domain your plugin covers

> **Note:** The plugin-store CLI is optional for local linting. Users install your finished plugin via `npx skills add okx/plugin-store-community --skill <plugin-name>` — no CLI installation required on their end.

### Key Rule

> **All on-chain interactions must use onchainos CLI.** This includes: wallet signing, transaction broadcasting, swap execution, contract calls, and any action that writes to the blockchain. You **are free** to query external data sources (third-party DeFi APIs, market data providers, analytics services, etc.) — but any on-chain action must go through onchainos. Plugins that bypass onchainos for on-chain operations will be rejected.

---

## 3. Step 1: Fork, Clone, and Scaffold Your Plugin

1. Go to https://github.com/okx/plugin-store-community and click **Fork**
2. Clone your fork, then run `init` inside it:

```bash
git clone --depth=1 git@github.com:YOUR_USERNAME/plugin-store-community.git
cd plugin-store-community
plugin-store init <your-plugin-name>
```

`init` auto-detects the `submissions/` directory and creates your plugin there:

```
submissions/<your-plugin-name>/
├── plugin.yaml                        # Plugin manifest (you fill this in)
├── skills/
│   └── <your-plugin-name>/
│       ├── SKILL.md                   # Skill definition (with onchainos demo built-in)
│       └── references/
│           └── cli-reference.md       # CLI reference docs (you write this)
├── LICENSE                            # MIT license template
├── CHANGELOG.md                       # Version history
└── README.md                          # Plugin description
```

**If you're building a Skill + Binary plugin**, you also need:
- Source code in your own GitHub repo (we compile it, you don't submit binaries)
- A `build` section in plugin.yaml pointing to your repo + commit SHA

---

## 4. Step 2: Write plugin.yaml

This is your plugin's manifest. It tells the Plugin Store what your plugin is, who wrote it, and what it can do.

### 4A. Skill-Only Example

```yaml
schema_version: 1
name: sol-price-checker              # Lowercase, hyphens only, 2-40 chars
version: "1.0.0"                     # Semantic versioning (x.y.z)
description: "Query real-time token prices on Solana with market data and trend analysis"
author:
  name: "Your Name"
  github: "your-github-username"     # Must match PR author
  email: "you@example.com"          # Optional: for security notifications
license: MIT
category: analytics                  # See categories below
tags:
  - price
  - analytics
type: "community-developer"          # Optional: e.g. "official", "dapp-official", "community-developer"
link: "https://your-project.com"     # Optional: project homepage URL

components:
  skill:
    dir: skills/sol-price-checker    # Path to your SKILL.md directory


api_calls: []

```

### 4B. Skill + Binary Example (with source code compilation)

If your plugin includes a binary, you need a `build` section. Your source code stays in your own GitHub repo — we compile it.

```yaml
schema_version: 1
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

build:
  lang: rust                            # rust | go | typescript | node | python
  source_repo: "defi-builder/yield-optimizer" # Your GitHub repo with source code
  source_commit: "a1b2c3d4e5f6..."      # Full 40-char commit SHA (pinned)
  source_dir: "."                       # Path within repo (default: root)
  binary_name: defi-yield           # Compiled output name


api_calls:
  - "api.defillama.com"

```

**Key differences from Skill-Only:**
- A `build` section with `source_repo` + `source_commit` — tells our CI where your source code is
- Our CI clones your repo at the exact commit SHA, compiles, and publishes the binary

**How to get the commit SHA:**

Your source code must be pushed to GitHub **before** you can get a valid commit SHA. The workflow is:

```bash
# 1. In your source code repo — develop and push your code first
cd your-source-repo
git add . && git commit -m "v1.0.0"
git push origin main

# 2. Get the full 40-character commit SHA
git rev-parse HEAD
# Output: a1b2c3d4e5f6789012345678901234567890abcd

# 3. Copy this SHA into your plugin.yaml build.source_commit field
```

> The commit must exist on GitHub (not just local). Our CI clones from GitHub at this exact SHA.

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
| `type` | No | Free-form string, e.g. `"official"`, `"dapp-official"`, `"community-developer"`. Defaults provided by CI. |
| `link` | No | Project homepage URL. Displayed in the marketplace. |
| `components.skill.dir` | Yes | Relative path to the directory containing SKILL.md |
| `api_calls` | No | List of external API domains the plugin calls (reviewer reference; lint checks against this) |

### Naming Rules

- Allowed: `solana-price-checker`, `defi-yield-optimizer`, `nft-tracker`
- Forbidden: `OKX-Plugin` (reserved prefix), `my_plugin` (underscores), `a` (too short)
- Reserved prefixes: `okx-`, `official-`, `plugin-store-`

---

## 5. Step 3: Write SKILL.md

SKILL.md is the **single entry point** of your plugin. It teaches the AI agent what your plugin does and how to use it. For Skill-only plugins, it orchestrates onchainos commands. For Binary plugins, it also orchestrates your custom tools.

```
Skill-Only plugin:
  SKILL.md → onchainos commands

Binary plugin:
  SKILL.md → onchainos commands
           + your binary tools (calculate_yield, find_route, ...)
           + your binary commands (my-tool start, my-tool status, ...)
```

### 5A. Template (Skill-Only)

```markdown
---
name: <your-plugin-name>
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

### 5B. Template (Binary Plugin)

If your plugin includes a binary, the SKILL.md must tell the AI agent about **both** onchainos commands and your custom binary tools:

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

This plugin combines custom yield analytics (via binary tools) with
onchainos execution capabilities to find and enter the best DeFi positions.

## Pre-flight Checks

1. The `onchainos` CLI is installed and configured
2. The defi-yield binary is installed via plugin-store
3. A valid DEFI_API_KEY environment variable is set

## Binary Tools (provided by this plugin)

### calculate_yield
Calculate the projected APY for a specific DeFi pool.
**Parameters**: pool_address (string), chain (string)
**Returns**: APY percentage, TVL, risk score

### find_best_route
Find the optimal swap route to enter a DeFi position.
**Parameters**: from_token (string), to_token (string), amount (number)
**Returns**: Route steps, estimated output, price impact

## Commands (using onchainos + binary tools together)

### Find Best Yield

1. Call binary tool `calculate_yield` for the target pool
2. Run `onchainos token info --address <pool_token> --chain <chain>`
3. Present yield rate + token info to user

### Enter Position

1. Call binary tool `find_best_route` for the swap
2. Run `onchainos swap quote --from <token> --to <pool_token> --amount <amount>`
3. **Ask user to confirm** the swap amount and expected yield
4. Run `onchainos swap swap ...` to execute
5. Report result to user

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| Binary connection failed | Server not running | Run `npx skills add okx/plugin-store-community --skill defi-yield-optimizer` |
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
4. **Include pre-flight checks** — Your SKILL.md must be npx-compatible. Include dependency installation checks (onchainos, npm packages, pip packages, binaries) so the AI agent's pre-flight can handle them from a blank environment. Phase 6 CI will auto-inject missing dependency installs, but including your own is recommended.
5. **Don't duplicate onchainos capabilities** — Your skill should orchestrate onchainos commands, not replace them

---

## 6. Step 4: Declare API Calls

The only declarations you need are `api_calls` — both are top-level fields in plugin.yaml. Actual permissions (wallet access, transaction signing, etc.) are auto-detected by the AI review during submission.

```yaml

api_calls:
  - "api.defillama.com"
```

- **`api_calls`** — list of external API domains your plugin calls. The linter checks that any URLs in your SKILL.md match this list.

---

## 7. Step 5: Local Validation

Before submitting, validate your plugin locally:

```bash
plugin-store lint ./<your-plugin-name>/
```

### If everything passes:

```
Linting ./<your-plugin-name>/...

✓ Plugin '<your-plugin-name>' passed all checks!
```

### If there are errors:

```
Linting ./<your-plugin-name>/...

  ❌ [E031] name 'My-Plugin' must be lowercase alphanumeric with hyphens only
  ❌ [E065] api_calls field is required
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
| E065 | Missing api_calls | Add `api_calls` field to plugin.yaml |
| E111 | Binary not allowed | Community plugins cannot include Binary components |

---

## 8. Step 6: Submit via Pull Request

Since you already forked and cloned in Step 1, your plugin is in `submissions/`. Create a branch and push to your fork:

```bash
git checkout -b submit/<your-plugin-name>
git add submissions/<your-plugin-name>/
git commit -m "[new-plugin] <your-plugin-name> v1.0.0"
git push origin submit/<your-plugin-name>
```

Then go to GitHub and open a Pull Request from your fork to `okx/plugin-store-community`. Use this title format:

```
[new-plugin] <your-plugin-name> v1.0.0
```

The PR template will guide you through the checklist.

### Important Rules for PRs

- Each PR should contain **one plugin only**
- Only modify files inside `submissions/your-plugin-name/`
- Do not modify any other files (README.md, workflows, etc.)
- The directory name must match the `name` field in plugin.yaml

---

## 8a. Alternative: Submit Source Code Directly (Mode A)

You can include source code (Python scripts, shell scripts, etc.) directly inside `submissions/<name>/` — no external GitHub repo needed.

```
submissions/<your-plugin-name>/
├── plugin.yaml
├── skills/
│   └── <your-plugin-name>/
│       ├── SKILL.md
│       └── scripts/          ← add your source files here
│           ├── bot.py
│           └── config.py
├── LICENSE
└── README.md
```

plugin.yaml for a directly-submitted plugin (no `build.source_repo` needed):
```yaml
schema_version: 1
name: my-plugin
version: "1.0.0"
description: "What your plugin does"
author:
  name: "Your Name"
  github: "your-username"
license: MIT
category: utility
tags: [keywords]

components:
  skill:
    dir: skills/my-plugin

api_calls: []
```

After merge, CI generates `marketplace.json` which enables `npx skills add` discovery for your plugin.

---

## 8b. Alternative: Submit via External Repository (Mode B)

If your plugin has source code (Python scripts, Rust/Go binaries), you can keep everything in your own GitHub repo and submit just a `plugin.yaml` pointer.

### Your repo structure (Claude marketplace compatible)

```
your-username/my-plugin/
├── .claude-plugin/
│   └── plugin.json           # Optional: makes it Claude marketplace compatible
├── skills/
│   └── my-plugin/
│       └── SKILL.md
├── scripts/
│   ├── bot.py
│   └── config.py
├── assets/
│   └── dashboard.html
├── src/                       # Rust/Go source (optional, for compiled plugins)
│   └── main.rs
├── Cargo.toml                 # Only if Rust (optional)
├── LICENSE
└── README.md
```

### Submit to community repo

Your submission is minimal — just a `plugin.yaml` pointer:

```
submissions/my-plugin/
├── plugin.yaml
├── LICENSE
└── README.md
```

plugin.yaml for Python/script plugins:
```yaml
schema_version: 1
name: my-plugin
version: "1.0.0"
description: "What your plugin does"
author:
  name: "Your Name"
  github: "your-username"
license: MIT
category: utility
tags: [keywords]

components:
  skill:
    repo: "your-username/my-plugin"
    commit: "full-40-char-sha"

api_calls: []
```

plugin.yaml for Rust/Go compiled plugins (add build section):
```yaml
# ... same as above, plus:
build:
  lang: rust
  source_repo: "your-username/my-plugin"
  source_commit: "full-40-char-sha"
  binary_name: "my-tool"
```

After merge, our CI automatically:
1. Copies your SKILL.md + scripts into community repo (persistent backup)
2. Compiles Rust/Go binaries and stores in community repo Release
3. Updates registry.json

---

## 8c. Alternative: One-Click Import (Mode C)

If you already have a Claude marketplace compatible repo, import it with one command:

```bash
plugin-store import your-username/my-plugin
```

This automatically:
1. Reads your `.claude-plugin/plugin.json` and `skills/` directory
2. Detects build language (Rust/Go/Python/Node)
3. Generates `plugin.yaml`
4. Forks community repo, creates branch, opens PR

You don't need to write any `plugin.yaml` — the CLI does it for you.

Pre-requisites: `gh` CLI installed and logged in (`gh auth login`).

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

Phase 4: Build Check (if binary)
  → Clones your source repo at the pinned commit SHA
  → Compiles Rust/Go or validates TS/Node/Python packages
  → Verifies the binary runs

Phase 6: Generate Summary (requires maintainer approval)
  → Generates SUMMARY.md and SKILL_SUMMARY.md for your plugin
  → Scans and records all dependency requirements
  → Auto-injects missing pre-flight dependency installs into SKILL.md
  → Triggered by a maintainer; not automatic on every PR

Phase 7: Publish
  → Updates marketplace.json (enables npx skills add discovery)
  → Tags release and updates registry
```

### Human Review (1-3 days)

A maintainer reviews:

- Does the plugin make sense?
- Are api_calls accurate?
- Is the SKILL.md well-written?
- Any security concerns?

### After Merge

Your plugin is automatically:

1. Added to `marketplace.json` (enables npx discovery) and `registry.json`
2. Tagged with `plugins/<your-plugin-name>@1.0.0`
3. Available to all users via:
   ```bash
   npx skills add okx/plugin-store-community --skill <your-plugin-name>
   ```

---

## 10. Updating Your Plugin

### Content Update (SKILL.md changes, new commands)

1. Modify files in `submissions/<your-plugin-name>/`
2. Bump `version` in plugin.yaml (e.g., `1.0.0` → `1.1.0`)
3. Update CHANGELOG.md
4. Open a PR with title: `[update] <your-plugin-name> v1.1.0`

### Chain or API Change (requires full review)

If your update changes `api_calls`, the review will be more thorough. The AI review report will highlight these changes.

---

## 11. Rules & Restrictions

### What You CAN Do

- Define skills using SKILL.md
- Reference any onchainos CLI command for on-chain operations
- Query external data sources (third-party DeFi APIs, market data, etc.)
- Include reference documentation
- Submit binary source code (we compile it via `build` section)
- Declare api_calls for external API domains

### What You CANNOT Do

- Submit pre-compiled binaries (.exe, .dll, .so, etc.) — must submit source code
- Use reserved name prefixes (`okx-`, `official-`, `plugin-store-`)
- Bypass onchainos for on-chain write operations (signing, broadcasting, swaps)
- Include prompt injection patterns in SKILL.md
- Exceed file size limits (200KB per file, 5MB total)

---

## 12. SKILL.md Writing Guide

### Structure Checklist

- [ ] YAML frontmatter with `name` and `description`
- [ ] Overview section (what does this skill do?)
- [ ] Pre-flight checks — **must be npx-compatible**: include dependency installation instructions for onchainos, npm packages, pip packages, or binaries so the AI agent's pre-flight can run them from a blank environment
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

## 13. Submitting Plugins with Source Code (Binary)

> **Important:** SKILL.md is always the entry point. Even if your plugin includes a binary, the SKILL.md is what tells the AI agent how to orchestrate everything — onchainos commands, your binary tools, and your binary commands.

### Who Can Submit Source Code?

Any developer can submit source code for binary compilation. Submit your source code in your own GitHub repo, add a `build` section to plugin.yaml, and our CI will compile it. Your external repo can also follow the Claude marketplace structure (with `.claude-plugin/plugin.json` and `skills/` directory) — see [Mode B](#8b-alternative-submit-via-external-repository-mode-b) for details on that layout.

### How It Works

```
You submit source code → Our CI compiles it → Users install our compiled artifact
You never submit binaries. We never modify your source code.
```

### plugin.yaml with build config

Your source code stays in your own GitHub repo. You provide the repo URL and a pinned commit SHA — our CI clones at that exact commit, compiles, and publishes. The commit SHA is the content fingerprint: same SHA = same code, guaranteed.

```yaml
schema_version: 1
name: <your-plugin-name>
version: "1.0.0"
description: "My custom binary tool"
author:
  name: "Your Name"
  github: "your-username"
license: MIT
category: defi-protocol
tags: [defi]

components:
  skill:
    dir: skills/<your-plugin-name>       # SKILL.md is ALWAYS required

build:
  lang: rust                          # rust | go | typescript | node | python
  source_repo: "your-username/<your-plugin-name>"  # Your GitHub repo with source code
  source_commit: "abc123def456..."    # Full 40-char commit SHA (pinned)
  source_dir: "."                     # Path within repo (default: root)
  binary_name: <your-plugin-name>         # Name of the compiled output
  # main: src/index.ts               # Required for typescript/python


api_calls: []
```

### How to get the commit SHA

Your source code must be pushed to GitHub first. Then:

```bash
cd your-source-repo
git push origin main            # make sure code is on GitHub
git rev-parse HEAD              # get the full 40-char SHA
# Output: 342756ee25405b5ec5b375a37c1b36710d5b9cd6
# Copy this into build.source_commit
```

### Directory Structure

Source code lives in your own repo. You only submit metadata + SKILL to the community repo:

```
submissions/<your-plugin-name>/            ← In community repo (small, ~20KB)
  plugin.yaml                         # With build section pointing to your repo
  skills/<your-plugin-name>/
    SKILL.md                          # The AI agent's entry point
    references/
  LICENSE
  CHANGELOG.md
  README.md

your-username/<your-plugin-name>           ← Your own GitHub repo (source code)
  Cargo.toml                          # (Rust example)
  src/
    main.rs
    lib.rs
```

### How to Submit a Binary Plugin (End-to-End)

If your plugin includes a compiled CLI tool, you need **two repos**:
1. **Your source code repo** — contains your CLI source code (you create this)
2. **plugin-store-community** — contains your plugin.yaml + SKILL.md (you fork this)

Here's the full workflow from zero to a working binary plugin submission:

#### Step A: Create your source code repo

Create a new GitHub repo for your CLI tool. Our CI will clone this repo and compile it.

**Your repo must compile with a single standard command.** No custom scripts, no multi-step builds. Our CI runs exactly one build command per language.

Required directory structure per language:

**Rust:**
```
your-org/your-tool/
├── Cargo.toml          ← MUST contain [[bin]] with name matching binary_name
├── Cargo.lock           ← commit this (reproducible builds)
└── src/
    └── main.rs          ← your code
```

`Cargo.toml` must have:
```toml
[package]
name = "your-tool"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "your-tool"      # ← this MUST match build.binary_name in plugin.yaml
path = "src/main.rs"
```

**Go:**
```
your-org/your-tool/
├── go.mod               ← MUST have module declaration
├── go.sum               ← commit this
└── main.go              ← must have package main + func main()
```

**TypeScript:**
```
your-org/your-tool/
├── package.json         ← MUST have name, version, and "bin" field
└── src/
    └── index.js         ← Compiled to JS, first line MUST be #!/usr/bin/env node
```

> **Important:** TypeScript plugins are distributed via `npm install -g`, not compiled to binary.
> Your `package.json` must include a `"bin"` field pointing to the JS entry file, and the entry file must start with `#!/usr/bin/env node`.
> If you write in TypeScript, compile to JS before pushing to your source repo.

`package.json` example:
```json
{
  "name": "your-tool",
  "version": "1.0.0",
  "type": "module",
  "bin": {
    "your-tool": "src/index.js"
  }
}
```

**Node.js:**
```
your-org/your-tool/
├── package.json         ← MUST have name, version, and "bin" field
└── src/
    └── index.js         ← First line MUST be #!/usr/bin/env node
```

> **Important:** Node.js plugins are distributed via `npm install -g`, not compiled to binary.
> Your `package.json` must include a `"bin"` field, and the entry file must start with `#!/usr/bin/env node`.

`package.json` example:
```json
{
  "name": "your-tool",
  "version": "1.0.0",
  "bin": {
    "your-tool": "src/index.js"
  }
}
```

**Python:**
```
your-org/your-tool/
├── pyproject.toml       ← MUST have [build-system], [project] (with name, version), and [project.scripts]
├── setup.py             ← Recommended for compatibility with older pip versions
└── src/
    ├── __init__.py
    └── main.py          ← This path goes in build.main; must have def main() entry
```

> **Important:** Python plugins are distributed via `pip install`, not compiled to binary.
> Your `pyproject.toml` must include `[project.scripts]` to define the CLI entry point. We recommend also providing `setup.py` for older pip compatibility.

`pyproject.toml` example:
```toml
[build-system]
requires = ["setuptools", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "your-tool"
version = "1.0.0"
requires-python = ">=3.8"

[project.scripts]
your-tool = "src.main:main"
```

#### Step B: Make sure it compiles locally

Before submitting, verify your code compiles with the exact command our CI uses:

```bash
# Rust
cd your-tool && cargo build --release
# Verify: target/release/your-tool exists

# Go
cd your-tool && CGO_ENABLED=0 go build -o your-tool -ldflags="-s -w" .
# Verify: ./your-tool exists

# TypeScript / Node.js
cd your-tool && npm install -g .
# Verify: your-tool --help runs successfully
# Note: package.json must have "bin" field, entry file must have #!/usr/bin/env node

# Python
cd your-tool && pip install .
# Verify: your-tool --help runs successfully
# Note: pyproject.toml must have [project.scripts], entry function must be def main()
```

If these commands fail locally, our CI will also fail.

#### Step C: Push and get the commit SHA

```bash
cd your-tool
git add -A
git commit -m "v1.0.0"
git push origin main

# Get the full 40-char SHA — this goes in plugin.yaml
git rev-parse HEAD
# Output: a1b2c3d4e5f6789012345678901234567890abcd
```

> The commit must exist on GitHub. Our CI clones from GitHub at this exact SHA.

#### Step D: Create the plugin submission

Now go to plugin-store-community and create your plugin:

```bash
git clone --depth=1 git@github.com:YOUR_USERNAME/plugin-store-community.git
cd plugin-store-community
plugin-store init <your-plugin-name>
```

Edit `submissions/<your-plugin-name>/plugin.yaml`:

```yaml
schema_version: 1
name: <your-plugin-name>
version: "1.0.0"
description: "What your plugin does"
author:
  name: "Your Name"
  github: "your-username"
license: MIT
category: utility
tags: [your-tags]

components:
  skill:
    dir: skills/<your-plugin-name>

build:
  lang: rust                             # rust | go | typescript | node | python
  source_repo: "your-org/your-tool"      # your GitHub repo from Step A
  source_commit: "a1b2c3d4e5f6..."       # SHA from Step C
  source_dir: "."                         # path within repo (usually root)
  binary_name: "your-tool"               # must match what the compiler outputs

api_calls: []
```

Edit `submissions/<your-plugin-name>/skills/<your-plugin-name>/SKILL.md` to describe how the AI agent should use your binary tool alongside onchainos commands.

#### Step E: Lint, push, and PR

```bash
plugin-store lint ./submissions/<your-plugin-name>/
git checkout -b submit/<your-plugin-name>
git add submissions/<your-plugin-name>/
git commit -m "[new-plugin] <your-plugin-name> v1.0.0"
git push origin submit/<your-plugin-name>
```

Open a PR from your fork to `okx/plugin-store-community`. Our CI will:
1. Lint your plugin.yaml + SKILL.md
2. AI review your code (reads your source repo at the pinned SHA)
3. Clone your source repo → compile → verify the binary works
4. Post reports on the PR

#### Common Build Failures

| Problem | Cause | Fix |
|---------|-------|-----|
| "Binary not found" | `binary_name` doesn't match compiled output | Rust: check `[[bin]] name` in Cargo.toml. Go: check `-o` flag. |
| "source_commit is not valid" | Short SHA or branch name used | Use full 40-char: `git rev-parse HEAD` |
| "source_repo format invalid" | Wrong format | Must be `owner/repo`, not `https://github.com/...` |
| Build fails but works locally | Platform difference | Our CI runs Ubuntu Linux. Ensure your code compiles on Linux. |
| Cargo.lock not found | Not committed | Run `cargo generate-lockfile` and commit `Cargo.lock`. |
| Python import error | Missing dependency | Ensure all deps are in `pyproject.toml` or `requirements.txt`. |

---

### Supported Languages

| Language | Entry File | Build Tool | Output |
|----------|-----------|------------|--------|
| Rust | `Cargo.toml` | `cargo build --release` | Native binary |
| Go | `go.mod` | `go build` | Native binary |
| TypeScript | `package.json` + `bin` | `npm install -g` | npm source package (~KB) |
| Node.js | `package.json` + `bin` | `npm install -g` | npm source package (~KB) |
| Python | `pyproject.toml` + `[project.scripts]` | `pip install` | pip source package (~KB) |

### Build Config — Complete Examples for Each Language

Every `build` field explained:

| Field | Required | Description |
|-------|----------|-------------|
| `lang` | Yes | `rust` \| `go` \| `typescript` \| `node` \| `python` |
| `source_repo` | Yes | GitHub `owner/repo` containing your source code |
| `source_commit` | Yes | Full 40-char commit SHA (get via `git rev-parse HEAD`) |
| `source_dir` | No | Path within repo to source root (default: `.`) |
| `entry` | No | Entry file override (default: auto-detected per language) |
| `binary_name` | Yes | Name of the compiled output binary |
| `main` | TS/Node/Python | Entry point file (e.g. `src/index.js`, `src/main.py`) |
| `targets` | No | Limit build platforms (default: all supported) |

#### Rust

```yaml
build:
  lang: rust
  source_repo: "your-org/your-rust-tool"
  source_commit: "a1b2c3d4e5f6789012345678901234567890abcd"
  source_dir: "."                        # default, can omit
  entry: "Cargo.toml"                    # default for Rust, can omit
  binary_name: "your-tool"              # must match [[bin]] name in Cargo.toml
  targets:                               # optional, omit for all platforms
    - x86_64-unknown-linux-gnu
    - aarch64-apple-darwin
```

CI runs: `cargo fetch` → `cargo audit` → `cargo build --release`
Output: native binary (~5-20MB)

#### Go

```yaml
build:
  lang: go
  source_repo: "your-org/your-go-tool"
  source_commit: "b2c3d4e5f6789012345678901234567890abcdef"
  source_dir: "."
  entry: "go.mod"                        # default for Go, can omit
  binary_name: "your-tool"
  targets:
    - x86_64-unknown-linux-gnu
    - aarch64-apple-darwin
```

CI runs: `go mod download` → `govulncheck` → `CGO_ENABLED=0 go build -ldflags="-s -w"`
Output: static native binary (~5-15MB)

#### TypeScript

```yaml
build:
  lang: typescript
  source_repo: "your-org/your-ts-tool"
  source_commit: "c3d4e5f6789012345678901234567890abcdef01"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/index.js"                   # REQUIRED — must be JS (not .ts)
```

Distribution: `npm install -g git+https://github.com/your-org/your-ts-tool#commit`
Requires: Node.js + npm
Output size: ~KB (source install, no large binary download)

> **Note:** `package.json` must include a `"bin"` field, and entry file must start with `#!/usr/bin/env node`.
> If writing in TypeScript, compile to JS before pushing to your source repo.

#### Python

```yaml
build:
  lang: python
  source_repo: "your-org/your-python-tool"
  source_commit: "d4e5f6789012345678901234567890abcdef0123"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/main.py"                    # REQUIRED for Python
```

Distribution: `pip install git+https://github.com/your-org/your-python-tool@commit`
Requires: Python 3.8+ and pip/pip3
Output size: ~KB (source install)

> **Note:** `pyproject.toml` must include `[build-system]`, `[project]`, and `[project.scripts]`.
> We recommend also providing `setup.py` for older pip compatibility. Entry function must be `def main()`.

#### Node.js

```yaml
build:
  lang: node
  source_repo: "your-org/your-node-tool"
  source_commit: "e5f6789012345678901234567890abcdef012345"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/index.js"                   # REQUIRED for Node.js
```

Distribution: `npm install -g git+https://github.com/your-org/your-node-tool#commit`
Requires: Node.js + npm
Output size: ~KB (source install)

> **Note:** `package.json` must include a `"bin"` field, and entry file must start with `#!/usr/bin/env node`.

> Node.js and TypeScript use the same distribution method (npm install). The only difference is that TypeScript must be compiled to JS first.

### SKILL.md as the Orchestrator

Your SKILL.md tells the AI agent how to use **both** onchainos commands and your custom binary tools:

```markdown
## Commands

### Check Yield (uses your binary tool)
Call binary tool `calculate_yield` with pool address and chain.

### Execute Deposit (uses onchainos + your binary)
1. Call binary tool `find_best_route` for the chosen pool
2. Run `onchainos swap quote --from USDC --to POOL_TOKEN`
3. **Ask user to confirm** amount and expected yield
4. Run `onchainos swap swap ...` to execute
5. Call binary tool `monitor_position` to start tracking
```

### What You Cannot Do

- Submit pre-compiled binaries (.exe, .dll, .so, .wasm) — E130
- Declare Binary without a build section — E110/E111
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

**Q: How do users install my plugin after it's published?**
A: Users run `npx skills add okx/plugin-store-community --skill <your-plugin-name>`. This works from a blank environment — the AI agent's pre-flight handles dependency installation (onchainos, binaries, pip packages, npm packages). No plugin-store CLI install is required.

**Q: Can I submit a plugin that calls external APIs directly?**
A: No. All on-chain operations must go through onchainos CLI. If you need a capability that onchainos doesn't provide, open a feature request in the onchainos repo.

**Q: Can I include a binary?**
A: Yes. Any developer can submit binary source code. Keep your source in your own GitHub repo and add a `build` section to plugin.yaml with `source_repo` and `source_commit`. Our CI compiles it. See Section 13 for details.

**Q: How long does the review take?**
A: Automated checks complete in ~5 minutes. Human review typically takes 1-3 business days.

**Q: What if the AI review flags something?**
A: The AI review is advisory only — it does not block your PR. However, human reviewers will read the AI report. Address any issues flagged to speed up approval.

**Q: Can I update my plugin after it's published?**
A: Yes. Submit a new PR with updated files and a bumped version number.


**Q: My `plugin-store lint` passes but the GitHub check fails. Why?**
A: Make sure you're running the latest version of the plugin-store CLI. Also ensure your PR only modifies files within `submissions/your-plugin-name/`.

**Q: What does error E122 "source_repo is not valid" mean?**
A: `build.source_repo` must be in `owner/repo` format (e.g. `your-username/my-server`). Don't include `https://github.com/` or `.git`.

**Q: What does error E123 "must be a full 40-character hex SHA" mean?**
A: `build.source_commit` must be the full commit hash, not a short SHA or branch name. Run `git rev-parse HEAD` in your source repo to get the full 40-character hash.

**Q: What does error E120 "must also include a Skill component" mean?**
A: Every plugin with a `build` section must also have a SKILL.md. The Skill is the entry point — it tells the AI agent how to use your binary.

**Q: What does error E130 "pre-compiled binary file is not allowed" mean?**
A: You submitted a compiled file (.exe, .dll, .so, .wasm, etc.) in your submission directory. Remove it — we compile from your source code, you don't submit binaries.

**Q: What does error E110/E111 "requires a build section" mean?**
A: You declared a Binary component but didn't include a `build` section. We need to know where your source code is so we can compile it. Add `build.lang`, `build.source_repo`, and `build.source_commit`.

**Q: The build failed in CI but I can compile locally. Why?**
A: Our CI compiles on Ubuntu Linux. Ensure your code builds on Linux, not just macOS/Windows. Check the build logs in the GitHub Actions run for specific errors.

---

## 16. Getting Help

- Open an [issue](https://github.com/okx/plugin-store-community/issues) on GitHub
- See `submissions/_example-plugin/` for a complete reference plugin
- Run `plugin-store lint` locally before submitting — it catches most issues
- Check the [GitHub Actions logs](https://github.com/okx/plugin-store-community/actions) if your PR checks fail
