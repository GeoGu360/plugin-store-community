# Plugin Store Community

Submit your plugin to the [Plugin Store](https://github.com/yz06276/plugin-store) ecosystem in 5 minutes.

## Quick Start (5 steps)

### Step 1: Clone and create your plugin

```bash
git clone git@github.com:yz06276/plugin-store-community.git
cd plugin-store-community
plugin-store init my-plugin
```

`init` auto-detects you're in the community repo and creates `submissions/my-plugin/`:

```
submissions/my-plugin/
├── plugin.yaml       ← fill in your plugin info
├── skills/
│   └── my-plugin/
│       └── SKILL.md  ← write what your plugin does (with onchainos demo)
├── LICENSE
└── README.md
```

### Step 2: Edit plugin.yaml and SKILL.md

Fill in `plugin.yaml` with your plugin info:

```yaml
schema_version: 1
name: my-plugin
version: "1.0.0"
description: "One-line description of what your plugin does"
author:
  name: "Your Name"
  github: "your-github-username"
license: MIT
category: utility    # trading-strategy | defi-protocol | analytics | utility | security | wallet | nft
tags: [keyword1, keyword2]

components:
  skill:
    dir: skills/my-plugin

api_calls: []        # external API domains, if any
```

Then edit `SKILL.md` — it teaches the AI agent how to use your plugin. The generated template already includes working onchainos examples.

> **Important:** All on-chain interactions — wallet signing, transaction broadcasting, swap execution, contract calls — **must** use [onchainos CLI](https://github.com/okx/onchainos-skills). You are free to query external data sources (third-party DeFi APIs, market data providers, etc.), but any action that touches the blockchain must go through onchainos. Plugins that bypass onchainos for on-chain operations will be rejected.

### Step 3: Check locally

```bash
plugin-store lint ./submissions/my-plugin/
```

Fix any errors (❌) it reports, then re-run until you see ✓.

### Step 4: Submit

```bash
git checkout -b submit/my-plugin
git add submissions/my-plugin/
git commit -m "[new-plugin] my-plugin v1.0.0"
git push origin submit/my-plugin
```

Then open a **Pull Request** on GitHub.

### Step 5: Wait for review

Your PR automatically gets:

```
✅ Structure check (~30s)     — bot validates plugin.yaml + SKILL.md
📋 AI code review (~2min)     — Claude reads your code and writes a report
🔨 Build check (if binary)    — compiles your source code
👤 Human review (1-3 days)    — maintainer reads AI report, clicks Merge
```

Once merged, your plugin is live:
```bash
plugin-store install my-plugin
```

---

## Reference

- **[Development Guide (English)](./PLUGIN_DEVELOPMENT_GUIDE.md)** — all details, examples, error codes, FAQ
- **[开发指南（中文）](./PLUGIN_DEVELOPMENT_GUIDE_ZH.md)** — 完整的 plugin 开发与提交指南

## Getting Help

- Open an [issue](https://github.com/yz06276/plugin-store-community/issues)
- See `submissions/_example-plugin/` for a complete reference plugin
- Read the full [Development Guide](./PLUGIN_DEVELOPMENT_GUIDE.md) for troubleshooting

## License

Each plugin must include its own license. This repository is MIT licensed.
