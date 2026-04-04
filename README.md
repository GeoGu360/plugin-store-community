<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/cover-dark.png">
    <source media="(prefers-color-scheme: light)" srcset="assets/cover-light.png">
    <img alt="Plugin Store Community" src="assets/cover-light.png" width="100%">
  </picture>
</p>

# Plugin Store Community

Submit your plugin to the [Plugin Store](https://github.com/okx/plugin-store) ecosystem in 5 minutes. Users install plugins via:

```bash
npx skills add okx/plugin-store-community --skill <plugin-name>
```

No CLI installation required — works from a blank environment.

## Quick Start (5 steps)

### Step 1: Fork, clone, and create your plugin

1. Go to https://github.com/okx/plugin-store-community and click **Fork**
2. Clone your fork and create a plugin:

```bash
git clone --depth=1 git@github.com:YOUR_USERNAME/plugin-store-community.git
cd plugin-store-community
plugin-store init <your-plugin-name>
```

`init` auto-detects you're in the community repo and creates `submissions/<your-plugin-name>/`:

```
submissions/<your-plugin-name>/
├── plugin.yaml       ← fill in your plugin info
├── skills/
│   └── <your-plugin-name>/
│       └── SKILL.md  ← write what your plugin does (with onchainos demo)
├── LICENSE
└── README.md
```

### Step 2: Edit plugin.yaml and SKILL.md

Fill in `plugin.yaml` with your plugin info:

```yaml
schema_version: 1
name: <your-plugin-name>
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
    dir: skills/<your-plugin-name>

api_calls: []        # external API domains, if any
```

Then edit `SKILL.md` — it teaches the AI agent how to use your plugin. The generated template already includes working onchainos examples.

> **Important:** All on-chain interactions — wallet signing, transaction broadcasting, swap execution, contract calls — **must** use [onchainos CLI](https://github.com/okx/onchainos-skills). You are free to query external data sources (third-party DeFi APIs, market data providers, etc.), but any action that touches the blockchain must go through onchainos. Plugins that bypass onchainos for on-chain operations will be rejected.

**Want to include source code (Python scripts, Rust/Go binaries)?** Three options:

- **Mode A:** Add source files directly to `submissions/<name>/skills/<name>/scripts/` — no external repo needed
- **Mode B:** Keep everything in your own repo, submit just a `plugin.yaml` pointer:
  ```yaml
  components:
    skill:
      repo: "your-username/my-plugin"    # your repo
      commit: "abc123..."                 # pinned commit
  ```
  Your repo can follow [Claude marketplace format](https://code.claude.com/docs/en/plugins-reference) — one repo, two ecosystems.
- **Mode C:** Already have a Claude marketplace repo? Import with one command:
  ```bash
  plugin-store import your-username/my-plugin
  ```

See the [Development Guide](./PLUGIN_DEVELOPMENT_GUIDE.md) for full details on all three submission modes.

### Step 3: Check locally

```bash
plugin-store lint ./submissions/<your-plugin-name>/
```

Fix any errors (❌) it reports, then re-run until you see ✓.

### Step 4: Submit

```bash
git checkout -b submit/<your-plugin-name>
git add submissions/<your-plugin-name>/
git commit -m "[new-plugin] <your-plugin-name> v1.0.0"
git push origin submit/<your-plugin-name>
```

Then go to GitHub and open a **Pull Request** from your fork to `okx/plugin-store-community`.

### Step 5: Wait for review

Your PR automatically gets:

```
✅ Structure check (~30s)     — bot validates plugin.yaml + SKILL.md
📋 AI code review (~2min)     — Claude reads your code and writes a report
🔨 Build check (if binary)    — compiles Rust/Go source; validates TS/Node/Python packages
📝 Summary generation         — maintainer triggers Phase 6: generates SUMMARY.md +
                                SKILL_SUMMARY.md, scans dependencies, injects pre-flight
👤 Human review (1-3 days)    — maintainer reads AI report, clicks Merge
```

Once merged, your plugin is live:
```bash
npx skills add okx/plugin-store-community --skill <your-plugin-name>
```

---

## Reference

- **[Development Guide (English)](./PLUGIN_DEVELOPMENT_GUIDE.md)** — all details, examples, error codes, FAQ
- **[开发指南（中文）](./PLUGIN_DEVELOPMENT_GUIDE_ZH.md)** — 完整的 plugin 开发与提交指南

## Getting Help

- Open an [issue](https://github.com/okx/plugin-store-community/issues)
- See `submissions/_example-plugin/` for a complete reference plugin
- Read the full [Development Guide](./PLUGIN_DEVELOPMENT_GUIDE.md) for troubleshooting

## Contributors

<a href="https://github.com/okx/plugin-store-community/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=okx/plugin-store-community" />
</a>

## License

Each plugin must include its own license. This repository is MIT licensed.
