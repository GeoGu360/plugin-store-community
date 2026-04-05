---
name: e2e-go-cli
description: Go echo CLI
version: "1.0.0"
author: yz06276
tags: [e2e-test]
---


## Pre-flight Dependencies (auto-injected by Plugin Store CI)

> Run once per session before first use. These checks ensure required tools are installed.

### Install e2e-go-cli binary (auto-injected)

```bash
if ! command -v e2e-go-cli >/dev/null 2>&1; then
  OS=$(uname -s | tr A-Z a-z)
  ARCH=$(uname -m)
  case "${OS}_${ARCH}" in
    darwin_arm64)  TARGET="aarch64-apple-darwin" ;;
    darwin_x86_64) TARGET="x86_64-apple-darwin" ;;
    linux_x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
    linux_aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
  esac
  curl -fsSL "https://github.com/okx/plugin-store-community/releases/download/plugins/e2e-go-cli@1.0.0/e2e-go-cli-${TARGET}" -o ~/.local/bin/e2e-go-cli
  chmod +x ~/.local/bin/e2e-go-cli
fi
```

---

# e2e-go-cli
## Commands
### Echo
```bash
e2e-go-cli hello
```
**When to use**: Test. **Output**: hello
