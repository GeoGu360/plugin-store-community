# Plugin Store Community

Community-submitted plugins for the [Plugin Store](https://github.com/yz06276/plugin-store) ecosystem.

## Documentation

- **[Plugin Development Guide (English)](./docs/PLUGIN_DEVELOPMENT_GUIDE.md)** — Full guide to developing and submitting plugins
- **[plugin 开发指南（中文）](./docs/PLUGIN_DEVELOPMENT_GUIDE_ZH.md)** — 完整的 plugin 开发与提交指南
- **[Contributing Guide](./CONTRIBUTING.md)** — Quick reference for contributors

## How to Submit a Plugin

1. **Scaffold** your plugin locally:
   ```bash
   plugin-store init my-awesome-plugin
   ```

2. **Develop** your SKILL.md and fill in `plugin.yaml`

3. **Validate** locally:
   ```bash
   plugin-store lint submissions/my-awesome-plugin/
   ```

4. **Clone** this repo, copy your plugin into `submissions/`, **push** and **open a PR**

5. Automated checks run (structure, AI review, security). Once all pass, a maintainer reviews and merges.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full guide.

## Core Concept: Skill is the Entry Point

Every plugin must include a SKILL.md. It is the **single entry point** that tells the AI agent how to use your plugin — whether it's a pure Skill or includes a binary. The Skill orchestrates onchainos CLI commands and your custom tools.

```
SKILL.md (orchestrator)
  ├── onchainos CLI commands (platform capabilities)
  └── Your binary commands (your capabilities)
```

Any developer can also include binaries by submitting source code — we compile it. See the [development guide](./docs/PLUGIN_DEVELOPMENT_GUIDE.md) for details.

## Directory Structure

```
submissions/
  my-plugin/
    plugin.yaml              # Plugin manifest (required)
    skills/
      my-plugin/
        SKILL.md             # Skill definition (required) — THE entry point
        references/          # Optional reference docs
    LICENSE                  # Required
    CHANGELOG.md             # Recommended
    README.md                # Recommended
```

## Review Pipeline

```
PR opened
  → Phase 2: Structure validation (bot, ~30s)
  → Phase 3: AI code review (Claude, ~2min)
  → Phase 4: Security audit (auto, ~3min)
  → Phase 5: Sandbox test (auto, ~5min)
  → Phase 6: Human review (maintainer, 1-3 days)
  → Phase 7: Auto-publish to registry
```

## What You Can Submit

| Component | Availability | How |
|-----------|-------------|-----|
| Skill (SKILL.md) | All developers | Include in submissions/ directory |
| Binary (source code) | All developers | Add `build` section to plugin.yaml, we compile |

## License

Each plugin must include its own license. This repository is MIT licensed.
