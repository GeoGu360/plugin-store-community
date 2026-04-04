# plugin 开发与提交指南

> 本指南将带你完成 Plugin Store plugin 的开发和提交全流程。读完本文，你将拥有一个可以与 onchainos CLI 集成的完整 plugin。

---

## 目录

1. [什么是 plugin？](#section-1)
2. [开始之前](#section-2)
3. [第一步：Fork、克隆并生成 plugin 脚手架](#section-3)
4. [第二步：编写 plugin.yaml](#section-4)
5. [第三步：编写 SKILL.md](#section-5)
6. [第四步：声明 API 调用](#section-6)
7. [第五步：本地验证](#section-7)
8. [第六步：通过 Pull Request 提交](#section-8)
8a. [替代方式：直接提交源码（模式 A）](#section-8a)
8b. [替代方式：通过外链仓库提交（模式 B）](#section-8b)
8c. [替代方式：一键导入（模式 C）](#section-8c)
9. [提交后会发生什么](#section-9)
10. [更新你的 plugin](#section-10)
11. [规则与限制](#section-11)
12. [SKILL.md 写作指南](#section-12)
13. [提交含源码的 plugin（Binary）](#section-13)
14. [onchainos 命令参考](#section-14)
15. [常见问题](#section-15)
16. [获取帮助](#section-16)

---

<a id="section-1"></a>

## 1. 什么是 plugin？

plugin 有一个必须的核心：**SKILL.md** — 一个 Markdown 文档，教 AI Agent 如何执行链上任务。可选地，plugin 还可以包含一个 **Binary**（由我们的 CI 从你的源码编译）。

**SKILL.md 始终是入口。** 即使你的 plugin 包含 Binary，Skill 也负责告诉 AI Agent 有哪些工具可用、什么时候使用。

### 链上操作：必须使用 onchainOS

所有与区块链交互的 plugin **必须**使用 [onchainOS Agentic Wallet](https://github.com/okx/onchainos-skills) 进行链上操作 — 钱包签名、交易广播、Swap 执行、合约调用等任何写入区块链的操作。

```
✅ 允许 — 自由查询任何数据源：
  第三方 DeFi API（DeFiLlama, Birdeye, DexScreener...）
  行情数据提供商、分析服务
  你自己的后端 API

❌ 必须使用 onchainOS — 所有链上写操作：
  钱包签名          → onchainos wallet send / sign
  交易广播          → onchainos gateway broadcast
  Swap 执行         → onchainos swap swap
  合约调用          → onchainos wallet contract-call
  Token 授权        → onchainos swap approve
```

> 使用第三方钱包（MetaMask、Phantom 等）或直接区块链 RPC 调用（ethers.js、web3.js 等）进行链上操作的 plugin **将被拒绝**。查看 [onchainOS 文档](https://github.com/okx/onchainos-skills) 了解所有可用能力。

### 两种类型的 plugin

```
类型 A：纯 Skill（最常见，任何开发者都可以）
────────────────────────────────────────────
  SKILL.md → 指挥 AI → 调用 onchainos CLI
                      + 查询外部数据（自由）

类型 B：Skill + Binary（任何开发者，源码由我们的 CI 编译）
────────────────────────────────────────────
  SKILL.md → 指挥 AI → 调用 onchainos CLI
                      + 调用你的 binary 工具
                      + 查询外部数据（自由）

  你的源码（在你自己的 GitHub 仓库中）
    → 我们的 CI 编译
    → 用户安装的是我们编译的产物
```

在开始之前选择你的路径：

| 我想要... | 类型 |
|-----------|------|
| 用 onchainos 命令创建策略 | 纯 Skill |
| 提供一个 CLI 工具 + Skill | Skill + Binary（提交源码，我们编译） |

---

<a id="section-2"></a>

## 2. 开始之前

### 前置条件

- **Git** 和 **GitHub 账号**
- 安装 **onchainos CLI**（用于测试命令）：
 ```bash
 curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | bash
 ```
 安装后如果找不到 `onchainos` 命令，添加到 PATH：
 ```bash
 export PATH="$HOME/.local/bin:$PATH"
 ```
- 了解你的 plugin 所涉及的区块链/DeFi 领域

> **注意：** plugin-store CLI 是可选的（用于本地 lint）。用户通过 `npx skills add okx/plugin-store-community --skill <plugin-name>` 安装你发布的 plugin，无需在用户端安装任何 CLI。

### 核心规则

> **所有链上交互操作必须通过 onchainos CLI。** 包括：钱包签名、交易广播、Swap 执行、合约调用等任何写入区块链的操作。你**可以自由**查询外部数据源（第三方 DeFi API、行情数据提供商、分析服务等），但任何链上操作必须走 onchainos。绕过 onchainos 进行链上操作的 plugin 将被拒绝。

---

<a id="section-3"></a>

## 3. 第一步：Fork、克隆并生成 plugin 脚手架

1. 打开 https://github.com/okx/plugin-store-community 点击 **Fork**
2. 克隆你的 fork，然后运行 `init`：

```bash
git clone --depth=1 git@github.com:YOUR_USERNAME/plugin-store-community.git
cd plugin-store-community
plugin-store init <your-plugin-name>
```

`init` 自动检测到 `submissions/` 目录，直接在里面创建你的 plugin：

```
submissions/<your-plugin-name>/
├── plugin.yaml                        # plugin 清单（需要填写）
├── skills/
│   └── <your-plugin-name>/
│       ├── SKILL.md                   # 技能定义（内置 onchainos demo）
│       └── references/
│           └── cli-reference.md       # CLI 参考文档
├── LICENSE                            # MIT 许可证模板
├── CHANGELOG.md                       # 版本变更记录
└── README.md                          # plugin 说明
```

**如果你要构建 Skill + Binary plugin**，你还需要：
- 源码在你自己的 GitHub 仓库中（我们来编译，你不需要提交二进制）
- 在 plugin.yaml 中添加 `build` 配置，指向你的仓库 + commit SHA

---

<a id="section-4"></a>

## 4. 第二步：编写 plugin.yaml

plugin.yaml 是 plugin 的清单文件，描述 plugin 的基本信息和组件。

### 4A. 纯 Skill 示例

```yaml
schema_version: 1
name: sol-price-checker # 小写字母 + 连字符，2-40 个字符
version: "1.0.0" # 语义化版本号 (x.y.z)
description: "Query real-time token prices on Solana with market data and trend analysis"
author:
 name: "你的名字"
 github: "your-github-username" # 必须与 PR 提交者一致
 email: "you@example.com" # 可选：用于安全通知
license: MIT
category: analytics # 见下方分类列表
tags:
 - solana
 - price
 - analytics
type: "community-developer"  # 可选：如 "official"、"dapp-official"、"community-developer"，CI 会提供默认值
link: "https://your-project.com"  # 可选：项目主页 URL，显示在 marketplace 中

components:
 skill:
 dir: skills/sol-price-checker # SKILL.md 所在目录的路径

api_calls: []
```

### 4B. Skill + Binary 示例（含源码编译）

如果你的 plugin 包含 binary 或二进制，需要添加 `build` 配置。源码在你自己的 GitHub 仓库中 — 我们来编译。

```yaml
schema_version: 1
name: defi-yield-optimizer
version: "1.0.0"
description: "跨协议 DeFi 收益优化，含自定义分析"
author:
 name: "DeFi Builder"
 github: "defi-builder"
license: MIT
category: defi-protocol
tags: [defi, yield]

components:
 skill:
 dir: skills/defi-yield-optimizer # SKILL.md — 始终必须，是入口

build:
 lang: rust # rust | go | typescript | node | python
 source_repo: "defi-builder/yield-optimizer" # 你的 GitHub 源码仓库
 source_commit: "a1b2c3d4e5f6..." # 完整的 40 位 commit SHA（锁定版本）
 source_dir: "." # 仓库内路径（默认：根目录）
 binary_name: defi-yield # 编译产物名

 - ethereum
 - base

api_calls:
 - "api.defillama.com"

 protocols: [morpho, aave]
```

**与纯 Skill 的关键区别：**
- 包含 `build` 配置，含 `source_repo` + `source_commit` — 告诉我们的 CI 源码在哪里
- 我们的 CI 在精确的 commit SHA 上克隆你的仓库，编译并发布二进制

**如何获取 commit SHA：**

你的源码必须先推送到 GitHub，然后才能获取有效的 commit SHA。流程如下：

```bash
# 1. 在你的源码仓库中 — 先开发并推送代码
cd your-source-repo
git add . && git commit -m "v1.0.0"
git push origin main

# 2. 获取完整的 40 位 commit SHA
git rev-parse HEAD
# 输出：a1b2c3d4e5f6789012345678901234567890abcd

# 3. 把这个 SHA 复制到 plugin.yaml 的 build.source_commit 字段
```

> commit 必须已经存在于 GitHub 上（不能只在本地）。我们的 CI 会从 GitHub 上的这个精确 SHA 克隆。

### 字段说明

| 字段 | 必填 | 规则 |
|------|------|------|
| `name` | 是 | 小写字母、数字和连字符 `[a-z0-9-]`，2-40 个字符，不能有连续连字符 |
| `version` | 是 | 语义化版本号：`x.y.z` |
| `description` | 是 | 一行描述，建议 200 字符以内 |
| `author.name` | 是 | 你的名字或组织名 |
| `author.github` | 是 | 你的 GitHub 用户名（必须与 PR 提交者一致） |
| `license` | 是 | SPDX 标识符：MIT, Apache-2.0, GPL-3.0 等 |
| `category` | 是 | `trading-strategy`, `defi-protocol`, `analytics`, `utility`, `security`, `wallet`, `nft` |
| `tags` | 否 | 搜索关键词 |
| `type` | 否 | 自由字符串，如 `"official"`、`"dapp-official"`、`"community-developer"`。CI 会提供默认值。 |
| `link` | 否 | 项目主页 URL，显示在 marketplace 中。 |
| `components.skill.dir` | 是 | SKILL.md 所在目录的相对路径 |
| `api_calls` | 否 | plugin 调用的外部 API 域名列表（供审查参考；lint 会据此检查） |

### 命名规则

- 允许：`solana-price-checker`、`defi-yield-optimizer`、`nft-tracker`
- 禁止：`OKX-Plugin`（保留前缀）、`my_plugin`（下划线）、`a`（太短）
- 保留前缀：`okx-`、`official-`、`plugin-store-`

---

<a id="section-5"></a>

## 5. 第三步：编写 SKILL.md

SKILL.md 是 plugin 的**唯一入口**。它教 AI Agent 你的 plugin 做什么以及如何使用。纯 Skill plugin 编排 onchainos 命令；Binary plugin 还额外编排你的自定义工具。

```
纯 Skill plugin：
 SKILL.md → onchainos 命令

Binary plugin：
 SKILL.md → onchainos 命令
 + 你的 binary 工具（calculate_yield, find_route, ...）
 + 你的二进制命令（my-tool start, my-tool status, ...）
```

### 5A. 模板（纯 Skill）

```markdown
---
name: <your-plugin-name>
description: "简要描述这个技能做什么"
version: "1.0.0"
author: "你的名字"
tags:
 - 关键词1
 - 关键词2
---

# My Awesome Plugin

## Overview

[2-3 句话：这个技能让 AI Agent 能做什么？]

## Pre-flight Checks

使用此技能前，请确保：

1. 已安装并配置 `onchainos` CLI
2. [其他前置条件]

## Commands

### [命令名称]

\`\`\`bash
onchainos <命令> <子命令> --参数 值
\`\`\`

**When to use**: [描述 AI 应该在什么场景下使用此命令]
**Output**: [描述命令返回什么内容]
**Example**: [展示一个具体示例]

## Error Handling

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| "Token not found" | 无效的 Token 符号 | 让用户确认 Token 名称 |
| "Rate limited" | 请求过于频繁 | 等待 10 秒后重试 |

## Skill Routing

- 需要代币交换 → 使用 `okx-dex-swap` 技能
- 需要钱包余额 → 使用 `okx-wallet-portfolio` 技能
- 需要安全扫描 → 使用 `okx-security` 技能
```

### 5B. 模板（Binary plugin）

如果你的 plugin 包含 binary，SKILL.md 必须同时描述 onchainos 命令和你的 binary 工具：

```markdown
---
name: defi-yield-optimizer
description: "DeFi 收益优化 — 自定义分析 + onchainos 执行"
version: "1.0.0"
author: "DeFi Builder"
tags:
 - defi
 - yield
---

# DeFi 收益优化器

## Overview

本 plugin 结合自定义收益分析（binary 工具）和 onchainos 执行能力，
帮用户找到并进入最佳的 DeFi 仓位。

## Pre-flight Checks

1. 已安装并配置 `onchainos` CLI
2. 已通过 plugin-store 安装 defi-yield binary
3. 已设置 DEFI_API_KEY 环境变量

## Binary 工具（本 plugin 提供）

### calculate_yield
计算指定 DeFi 池子的预期 APY。
**参数**: pool_address (string), chain (string)
**返回**: APY 百分比、TVL、风险评分

### find_best_route
寻找进入 DeFi 仓位的最优交换路径。
**参数**: from_token (string), to_token (string), amount (number)
**返回**: 路径步骤、预估产出、价格影响

## 命令（onchainos + binary 工具配合使用）

### 查询最佳收益

1. 调用 binary 工具 `calculate_yield` 获取目标池子的收益率
2. 执行 `onchainos token info --address <pool_token> --chain <chain>`
3. 向用户展示收益率 + 代币信息

### 执行存入

1. 调用 binary 工具 `find_best_route` 获取最优路径
2. 执行 `onchainos swap quote --from <token> --to <pool_token> --amount <amount>`
3. **请用户确认** 金额和预期收益
4. 执行 `onchainos swap swap ...`
5. 向用户报告结果

## Error Handling

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| Binary 连接失败 | Server 未运行 | 执行 `npx skills add okx/plugin-store-community --skill defi-yield-optimizer` |
| "Pool not found" | 无效的池子地址 | 确认合约地址 |
| "余额不足" | 代币不够 | 用 `onchainos portfolio all-balances` 查看余额 |

## Skill Routing

- 只需代币交换 → 使用 `okx-dex-swap` 技能
- 需要安全检查 → 使用 `okx-security` 技能
```

### SKILL.md 写作最佳实践

1. **要具体** — "执行 `onchainos token search --query SOL --chain solana`" 比 "搜索代币" 好得多
2. **必须包含错误处理** — AI Agent 需要知道出错时该怎么做
3. **使用技能路由** — 告诉 AI 什么时候应该转交给其他技能，而不是试图处理所有事情
4. **包含前置检查，且必须 npx 兼容** — 列出 onchainos、npm 包、pip 包、binary 等依赖的安装方式，确保 AI Agent 的 pre-flight 能在空白环境中自动处理。Phase 6 CI 会自动注入缺失的依赖安装步骤，但建议开发者自行包含。
5. **不要重复 onchainos 的能力** — 你的技能应该编排 onchainos 命令，而不是替代它们

---

<a id="section-6"></a>

## 6. 第四步：声明 API 调用

你只需要声明 `api_calls` — 两者都是 plugin.yaml 中的顶级字段。实际权限（钱包访问、交易签名等）由提交时的 AI 审查自动检测。

```yaml
 - solana
 - ethereum

api_calls:
 - "api.defillama.com"
```

- **`api_calls`** — plugin 调用的外部 API 域名列表。Linter 会检查你的 SKILL.md 中的 URL 是否与此列表匹配。

---

<a id="section-7"></a>

## 7. 第五步：本地验证

提交前在本地验证你的 plugin：

```bash
plugin-store lint ./<your-plugin-name>/
```

### 全部通过时：

```
Linting ./<your-plugin-name>/...

✓ Plugin '<your-plugin-name>' passed all checks!
```

### 有错误时：

```
Linting ./<your-plugin-name>/...

 ❌ [E031] name 'My-Plugin' must be lowercase alphanumeric with hyphens only
 ❌ [E065] api_calls field is required
 ⚠️ [W091] SKILL.md frontmatter missing recommended field: description

✗ Plugin 'My-Plugin': 2 error(s), 1 warning(s)
```

修复所有错误（❌）后再提交。警告（⚠️）是建议性的。

### 常见 Lint 错误

| 错误码 | 含义 | 修复方法 |
|--------|------|---------|
| E001 | plugin.yaml 未找到 | 确保 plugin.yaml 在提交目录的根目录 |
| E031 | 名称格式无效 | 只使用小写字母、数字和连字符 |
| E033 | 使用了保留前缀 | 名称不要以 `okx-`、`official-` 或 `plugin-store-` 开头 |
| E035 | 版本号无效 | 使用语义化版本号：`1.0.0`，而不是 `1.0` 或 `v1.0.0` |
| E041 | 缺少 LICENSE | 在提交目录中添加 LICENSE 文件 |
| E052 | 缺少 SKILL.md | 确保 SKILL.md 存在于 `components.skill.dir` 指定的路径中 |
| E065 | 缺少 api_calls | 在 plugin.yaml 中添加 `api_calls` 字段 |
| E111 | 不允许 Binary 组件 | 社区 plugin 不能包含 Binary 组件 |

---

<a id="section-8"></a>

## 8. 第六步：通过 Pull Request 提交

你已经在第一步 fork 并克隆了仓库，plugin 在 `submissions/` 里，创建分支并推送到你的 fork：

```bash
git checkout -b submit/<your-plugin-name>
git add submissions/<your-plugin-name>/
git commit -m "[new-plugin] <your-plugin-name> v1.0.0"
git push origin submit/<your-plugin-name>
```

然后在 GitHub 上从你的 fork 向 `okx/plugin-store-community` 创建 Pull Request。

在 GitHub 上从你的分支创建 Pull Request。使用以下标题格式：

```
[new-plugin] <your-plugin-name> v1.0.0
```

PR 模板会引导你完成检查清单。

### PR 重要规则

- 每个 PR 只包含 **一个 plugin**
- 只修改 `submissions/你的 plugin 名/` 目录下的文件
- 不要修改其他文件（README.md、workflows 等）
- 目录名必须与 plugin.yaml 中的 `name` 字段一致

---

<a id="section-8a"></a>

## 8a. 替代方式：直接提交源码（模式 A）

你可以直接在 `submissions/<name>/` 目录中包含源码（Python 脚本、Shell 脚本等），无需创建外部 GitHub 仓库。

```
submissions/<your-plugin-name>/
├── plugin.yaml
├── skills/
│   └── <your-plugin-name>/
│       ├── SKILL.md
│       └── scripts/          ← 直接在这里添加源文件
│           ├── bot.py
│           └── config.py
├── LICENSE
└── README.md
```

直接提交源码时的 plugin.yaml（无需 `build.source_repo`）：
```yaml
schema_version: 1
name: my-plugin
version: "1.0.0"
description: "你的 plugin 做什么"
author:
  name: "你的名字"
  github: "your-username"
license: MIT
category: utility
tags: [关键词]

components:
  skill:
    dir: skills/my-plugin

api_calls: []
```

合并后，CI 会生成 `marketplace.json`，使你的 plugin 可通过 `npx skills add` 被发现。

---

<a id="section-8b"></a>

## 8b. 替代方式：通过外链仓库提交（模式 B）

如果你的 plugin 包含源码（Python 脚本、Rust/Go 二进制），你可以把所有内容放在自己的 GitHub 仓库中，只提交一个 `plugin.yaml` 指针到 community repo。

### 你的仓库结构（兼容 Claude marketplace）

```
your-username/my-plugin/
├── .claude-plugin/
│   └── plugin.json           # 可选：兼容 Claude marketplace
├── skills/
│   └── my-plugin/
│       └── SKILL.md
├── scripts/
│   ├── bot.py
│   └── config.py
├── assets/
│   └── dashboard.html
├── src/                       # Rust/Go 源码（可选）
│   └── main.rs
├── Cargo.toml                 # 仅 Rust（可选）
├── LICENSE
└── README.md
```

### 提交到 community repo

你的提交非常轻量——只需一个 `plugin.yaml` 指针：

```
submissions/my-plugin/
├── plugin.yaml
├── LICENSE
└── README.md
```

Python/脚本类 plugin 的 plugin.yaml：
```yaml
schema_version: 1
name: my-plugin
version: "1.0.0"
description: "你的 plugin 做什么"
author:
  name: "你的名字"
  github: "your-username"
license: MIT
category: utility
tags: [关键词]

components:
  skill:
    repo: "your-username/my-plugin"
    commit: "完整的40位commit-sha"

api_calls: []
```

Rust/Go 编译类 plugin（额外加 build 字段）：
```yaml
# ... 同上，加上：
build:
  lang: rust
  source_repo: "your-username/my-plugin"
  source_commit: "完整的40位commit-sha"
  binary_name: "my-tool"
```

合并后我们的 CI 自动：
1. 拷贝你的 SKILL.md + 脚本到 community 仓库（持久化备份）
2. 编译 Rust/Go 二进制到 community 仓库 Release
3. 更新 registry.json

### 好处

- 一个仓库，同时兼容 Claude marketplace 和 plugin-store
- 你的仓库加上 `.claude-plugin/plugin.json` 就能直接被 Claude marketplace 识别
- community repo 只存指针，不重复存储源码

<a id="section-8c"></a>

## 8c. 替代方式：一键导入（模式 C）

如果你已有一个 Claude marketplace 兼容仓库，一条命令完成上架：

```bash
plugin-store import your-username/my-plugin
```

自动完成：
1. 读取你的 `.claude-plugin/plugin.json` 和 `skills/` 目录
2. 检测编译语言（Rust/Go/Python/Node）
3. 生成 `plugin.yaml`
4. Fork community repo，创建分支，提交 PR

你不需要手写 `plugin.yaml`——CLI 帮你生成。

前置条件：`gh` CLI 已安装并登录（`gh auth login`）。

---

<a id="section-9"></a>

## 9. 提交后会发生什么

### 自动化检查（约 5 分钟）

```
Phase 2：结构验证（lint）
 → 自动检查 15+ 项规则
 → 在 PR 评论中发布检查结果
 → 如果失败：PR 被阻止，修复后重新推送

Phase 3：AI 代码审查（Claude）
 → 读取你的 plugin + 最新的 onchainos 源码
 → 生成 8 个章节的审查报告
 → 在 PR 评论中发布报告（可折叠展开）
 → 仅供参考 — 不会阻止合并

Phase 4：构建检查（如含 binary）
 → 在锁定的 commit SHA 上克隆你的源码仓库
 → 编译 Rust/Go 或验证 TS/Node/Python 包
 → 验证二进制可以运行

Phase 6：生成摘要（需要维护者批准触发）
 → 为你的 plugin 生成 SUMMARY.md 和 SKILL_SUMMARY.md
 → 扫描并记录所有依赖项
 → 自动将缺失的依赖安装步骤注入到 SKILL.md 的 pre-flight 中
 → 由维护者手动触发，不会在每个 PR 上自动运行

Phase 7：发布
 → 更新 marketplace.json（使 npx skills add 可以发现你的 plugin）
 → 创建 release tag 并更新 registry
```

### 人工审核（1-3 天）

维护者会审核：

- plugin 是否有意义？
- api_calls 是否准确？
- SKILL.md 写得好不好？
- 是否存在安全隐患？

### 合并后

你的 plugin 会自动：

1. 添加到 `marketplace.json`（支持 npx 发现）和 `registry.json`
2. 创建 git tag `plugins/<your-plugin-name>@1.0.0`
3. 所有用户可通过以下命令安装：
   ```bash
   npx skills add okx/plugin-store-community --skill <your-plugin-name>
   ```

---

<a id="section-10"></a>

## 10. 更新你的 plugin

### 内容更新（修改 SKILL.md、添加命令）

1. 修改 `submissions/<your-plugin-name>/` 下的文件
2. 在 plugin.yaml 中升级 `version`（例如 `1.0.0` → `1.1.0`）
3. 更新 CHANGELOG.md
4. 创建 PR，标题格式：`[update] <your-plugin-name> v1.1.0`

### 链或 API 变更（需要完整审核）

如果你的更新修改了 `api_calls`，审核会更加严格。AI 审查报告会重点标注这些变化。

---

<a id="section-11"></a>

## 11. 规则与限制

### 你可以做的

- 使用 SKILL.md 定义技能
- 引用任何 onchainos CLI 命令进行链上操作
- 自由查询外部数据源（第三方 DeFi API、行情数据等）
- 包含参考文档
- 提交 Binary 源码（我们通过 `build` 配置编译）
- 声明 api_calls 外部 API 域名

### 你不能做的

- 提交预编译的二进制文件（.exe、.dll、.so 等）— 必须提交源码
- 使用保留名称前缀（`okx-`、`official-`、`plugin-store-`）
- 绕过 onchainos 进行链上写操作（签名、广播、Swap）
- 在 SKILL.md 中包含 prompt injection 模式
- 超过文件大小限制（单文件 200KB，总计 5MB）

---

<a id="section-12"></a>

## 12. SKILL.md 写作指南

### 结构清单

- [ ] YAML frontmatter 包含 `name` 和 `description`
- [ ] Overview 部分（这个技能做什么？）
- [ ] Pre-flight Checks 部分 — **必须 npx 兼容**：包含 onchainos、npm 包、pip 包、binary 等依赖的安装方式，确保 AI Agent 的 pre-flight 能在空白环境中自动处理
- [ ] Commands 部分（每个 onchainos 命令的使用场景/方式/输出）
- [ ] Error Handling 表格
- [ ] Skill Routing（什么时候转交给其他技能）

### 好的 vs 不好的示例

**不好：模糊的指令**
```
用 onchainos 获取价格。
```

**好：具体且可执行**
```
要获取 Solana 代币的当前价格：

\`\`\`bash
onchainos market price --address <TOKEN_ADDRESS> --chain solana
\`\`\`

**使用场景**: 当用户问"[代币]的价格是多少？"且在 Solana 链上时。
**输出**: 当前美元价格、24 小时涨跌幅、24 小时交易量。
**如果找不到代币**: 让用户确认合约地址，或先执行 `onchainos token search --query <名称> --chain solana` 搜索。
```

---

<a id="section-13"></a>

## 13. 提交含源码的 plugin（Binary）

> **核心概念：SKILL.md 是一切的入口。** 即使你的 plugin 包含 binary 或二进制文件，SKILL.md 仍然是 AI Agent 的操作指南 — 它告诉 AI 如何编排 onchainos 命令和你的自定义工具。

### 谁可以提交源码？

任何开发者都可以提交 Binary 源码。将源码放在你自己的 GitHub 仓库中，在 plugin.yaml 中添加 `build` 配置，我们的 CI 会编译。

### 运作方式

```
你提交源码 → 我们的 CI 编译 → 用户安装的是我们编译的产物
你永远不提交二进制文件。我们永远不修改你的源码。
```

### 包含 build 配置的 plugin.yaml

你的源码保留在你自己的 GitHub 仓库中。你提供仓库地址和一个锁定的 commit SHA — 我们的 CI 在这个精确的提交上克隆、编译、发布。commit SHA 就是内容指纹：相同的 SHA = 相同的代码，不可篡改。

```yaml
schema_version: 1
name: <your-plugin-name>
version: "1.0.0"
description: "我的 binary 工具"
author:
 name: "你的名字"
 github: "your-username"
license: MIT
category: defi-protocol
tags: [defi]

components:
 skill:
 dir: skills/<your-plugin-name> # SKILL.md 始终必须

build:
 lang: rust # rust | go | typescript | node | python
 source_repo: "your-username/<your-plugin-name>" # 你的 GitHub 源码仓库
 source_commit: "abc123def456..." # 完整的 40 位 commit SHA（锁定版本）
 source_dir: "." # 仓库内的路径（默认：根目录）
 binary_name: <your-plugin-name> # 编译产物名
 # main: src/index.ts # TypeScript/Python 需要指定

 - ethereum

api_calls: []
```

### 如何获取 commit SHA

源码必须先推送到 GitHub：

```bash
cd your-source-repo
git push origin main              # 确保代码在 GitHub 上
git rev-parse HEAD                # 获取完整 40 位 SHA
# 输出：342756ee25405b5ec5b375a37c1b36710d5b9cd6
# 把这个完整的 40 位字符串复制到 build.source_commit
```

### 目录结构

源码在你自己的仓库中。你只需要把元数据 + SKILL 提交到 community 仓库：

```
submissions/<your-plugin-name>/ ← 在 community 仓库中（很小，约 20KB）
 plugin.yaml # 包含 build 配置，指向你的仓库
 skills/<your-plugin-name>/
 SKILL.md # AI Agent 的入口
 references/
 LICENSE
 CHANGELOG.md
 README.md

your-username/<your-plugin-name> ← 你自己的 GitHub 仓库（源码）
 Cargo.toml # （Rust 示例）
 src/
 main.rs
 lib.rs
```

### 如何提交含二进制的 Plugin（端到端流程）

如果你的 plugin 包含编译的 CLI 工具，你需要**两个仓库**：
1. **你的源码仓库** — 包含你的 CLI 源码（你自己创建）
2. **plugin-store-community** — 包含你的 plugin.yaml + SKILL.md（你 fork 这个仓库）

从零到提交成功的完整流程：

#### Step A：创建你的源码仓库

在 GitHub 上新建一个仓库放你的 CLI 源码。我们的 CI 会 clone 这个仓库并编译。

**你的仓库必须能用单条标准命令编译。** 不要用自定义脚本或多步构建。我们的 CI 对每种语言只执行一条编译命令。

每种语言的目录结构要求：

**Rust：**
```
your-org/your-tool/
├── Cargo.toml          ← 必须包含 [[bin]]，name 要和 binary_name 一致
├── Cargo.lock           ← 提交这个文件（可复现构建）
└── src/
    └── main.rs          ← 你的代码
```

`Cargo.toml` 必须有：
```toml
[package]
name = "your-tool"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "your-tool"      # ← 必须和 plugin.yaml 的 build.binary_name 一致
path = "src/main.rs"
```

**Go：**
```
your-org/your-tool/
├── go.mod               ← 必须有 module 声明
├── go.sum               ← 提交这个文件
└── main.go              ← 必须有 package main + func main()
```

**TypeScript：**
```
your-org/your-tool/
├── package.json         ← 必须有 name、version 和 bin 字段
└── src/
    └── index.js         ← 编译为 JS，首行必须有 #!/usr/bin/env node
```

> **重要：** TypeScript plugin 通过 `npm install -g` 分发，不是编译为二进制。
> 你的 `package.json` 必须包含 `"bin"` 字段指向 JS 入口文件，入口文件首行必须有 `#!/usr/bin/env node`。
> 如果你用 TypeScript 编写，请先编译为 JS 再提交，或者直接用 JS 编写。

`package.json` 示例：
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

**Node.js：**
```
your-org/your-tool/
├── package.json         ← 必须有 name、version 和 bin 字段
└── src/
    └── index.js         ← 首行必须有 #!/usr/bin/env node
```

> **重要：** Node.js plugin 通过 `npm install -g` 分发，不是编译为二进制。
> 你的 `package.json` 必须包含 `"bin"` 字段，入口文件首行必须有 `#!/usr/bin/env node`。

`package.json` 示例：
```json
{
  "name": "your-tool",
  "version": "1.0.0",
  "bin": {
    "your-tool": "src/index.js"
  }
}
```

**Python：**
```
your-org/your-tool/
├── pyproject.toml       ← 必须有 [build-system]、[project]（含 name 和 version）和 [project.scripts]
├── setup.py             ← 推荐：兼容旧版 pip
└── src/
    ├── __init__.py
    └── main.py          ← 这个路径填到 build.main，必须有 def main() 入口
```

> **重要：** Python plugin 通过 `pip install` 分发，不是编译为二进制。
> 你的 `pyproject.toml` 必须包含 `[project.scripts]` 定义 CLI 入口点。推荐同时提供 `setup.py` 以兼容旧版 pip。

`pyproject.toml` 示例：
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

#### Step B：确保本地能编译

提交前用我们 CI 的精确命令验证：

```bash
# Rust
cd your-tool && cargo build --release
# 验证：target/release/your-tool 存在

# Go
cd your-tool && CGO_ENABLED=0 go build -o your-tool -ldflags="-s -w" .
# 验证：./your-tool 存在

# TypeScript / Node.js
cd your-tool && npm install -g .
# 验证：your-tool --help 能运行
# 注意：需要 package.json 有 "bin" 字段，入口文件有 #!/usr/bin/env node

# Python
cd your-tool && pip install .
# 验证：your-tool --help 能运行
# 注意：需要 pyproject.toml 有 [project.scripts]，入口函数是 def main()
```

如果用这些命令安装不成功，我们的 CI 也会失败。

#### Step C：推送并获取 commit SHA

```bash
cd your-tool
git add -A
git commit -m "v1.0.0"
git push origin main

# 获取完整 40 位 SHA — 填入 plugin.yaml
git rev-parse HEAD
# 输出：a1b2c3d4e5f6789012345678901234567890abcd
```

> commit 必须已推送到 GitHub。我们的 CI 从 GitHub 上的这个 SHA 克隆。

#### Step D：创建 plugin 提交

回到 plugin-store-community 创建你的 plugin：

```bash
git clone --depth=1 git@github.com:YOUR_USERNAME/plugin-store-community.git
cd plugin-store-community
plugin-store init <your-plugin-name>
```

编辑 `submissions/<your-plugin-name>/plugin.yaml`：

```yaml
schema_version: 1
name: <your-plugin-name>
version: "1.0.0"
description: "你的 plugin 做什么"
author:
  name: "你的名字"
  github: "your-username"
license: MIT
category: utility
tags: [your-tags]

components:
  skill:
    dir: skills/<your-plugin-name>

build:
  lang: rust                             # rust | go | typescript | node | python
  source_repo: "your-org/your-tool"      # Step A 创建的 GitHub 仓库
  source_commit: "a1b2c3d4e5f6..."       # Step C 获取的 SHA
  source_dir: "."                         # 仓库内路径（通常是根目录）
  binary_name: "your-tool"               # 必须和编译产物名一致

api_calls: []
```

编辑 `submissions/<your-plugin-name>/skills/<your-plugin-name>/SKILL.md`，描述 AI agent 如何使用你的二进制工具和 onchainos 命令。

#### Step E：Lint、推送、创建 PR

```bash
plugin-store lint ./submissions/<your-plugin-name>/
git checkout -b submit/<your-plugin-name>
git add submissions/<your-plugin-name>/
git commit -m "[new-plugin] <your-plugin-name> v1.0.0"
git push origin submit/<your-plugin-name>
```

从你的 fork 向 `okx/plugin-store-community` 创建 PR。我们的 CI 会：
1. Lint 检查 plugin.yaml + SKILL.md
2. AI 审查你的代码（读取你的源码仓库）
3. Clone 你的源码 → 编译 → 验证二进制可用
4. 在 PR 上发布报告

#### 常见编译失败

| 问题 | 原因 | 修复 |
|------|------|------|
| "Binary not found" | `binary_name` 和编译产物名不匹配 | Rust：检查 Cargo.toml 的 `[[bin]] name`。Go：检查 `-o` 参数。 |
| "source_commit is not valid" | 用了短 SHA 或分支名 | 用完整 40 位：`git rev-parse HEAD` |
| "source_repo format invalid" | 格式错误 | 必须是 `owner/repo`，不是 `https://github.com/...` |
| CI 编译失败但本地能编译 | 平台差异 | 我们的 CI 运行在 Ubuntu Linux，确保你的代码在 Linux 上能编译 |
| 找不到 Cargo.lock | 没有提交 | 运行 `cargo generate-lockfile` 并提交 `Cargo.lock` |
| Python import 错误 | 缺少依赖 | 确保所有依赖在 `pyproject.toml` 或 `requirements.txt` 中 |

---

### 支持的语言

| 语言 | 入口文件 | 分发方式 | 用户安装方式 |
|------|---------|---------|-------------|
| Rust | `Cargo.toml` | GitHub Release 二进制 | 自动下载（~5-20MB） |
| Go | `go.mod` | GitHub Release 二进制 | 自动下载（~5-15MB） |
| TypeScript | `package.json` + `bin` | npm 源码包 | `npm install -g`（~KB 级） |
| Node.js | `package.json` + `bin` | npm 源码包 | `npm install -g`（~KB 级） |
| Python | `pyproject.toml` + `[project.scripts]` | pip 源码包 | `pip install`（~KB 级） |

### Build 配置 — 每种语言的完整示例

所有 `build` 字段说明：

| 字段 | 必填 | 说明 |
|------|------|------|
| `lang` | 是 | `rust` \| `go` \| `typescript` \| `node` \| `python` |
| `source_repo` | 是 | GitHub `owner/repo`，你的源码仓库 |
| `source_commit` | 是 | 完整 40 位 commit SHA（通过 `git rev-parse HEAD` 获取） |
| `source_dir` | 否 | 仓库内源码根目录（默认：`.`） |
| `entry` | 否 | 入口文件覆盖（默认：按语言自动检测） |
| `binary_name` | 是 | 编译产物的二进制名 |
| `main` | TS/Node/Python | 入口文件路径（如 `src/index.js`、`src/main.py`） |
| `targets` | 否 | 限定编译平台（默认：全部支持的平台） |

#### Rust

```yaml
build:
  lang: rust
  source_repo: "your-org/your-rust-tool"
  source_commit: "a1b2c3d4e5f6789012345678901234567890abcd"
  source_dir: "."                        # 默认值，可省略
  entry: "Cargo.toml"                    # Rust 默认值，可省略
  binary_name: "your-tool"              # 必须和 Cargo.toml 中的 [[bin]] name 一致
  targets:                               # 可选，省略则编译全平台
    - x86_64-unknown-linux-gnu
    - aarch64-apple-darwin
```

CI 执行：`cargo fetch` → `cargo audit` → `cargo build --release`
产物：原生二进制（约 5-20MB）

#### Go

```yaml
build:
  lang: go
  source_repo: "your-org/your-go-tool"
  source_commit: "b2c3d4e5f6789012345678901234567890abcdef"
  source_dir: "."
  entry: "go.mod"                        # Go 默认值，可省略
  binary_name: "your-tool"
  targets:
    - x86_64-unknown-linux-gnu
    - aarch64-apple-darwin
```

CI 执行：`go mod download` → `govulncheck` → `CGO_ENABLED=0 go build -ldflags="-s -w"`
产物：静态原生二进制（约 5-15MB）

#### TypeScript

```yaml
build:
  lang: typescript
  source_repo: "your-org/your-ts-tool"
  source_commit: "c3d4e5f6789012345678901234567890abcdef01"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/index.js"                   # 必须是 JS 文件（非 .ts）
```

分发方式：`npm install -g git+https://github.com/your-org/your-ts-tool#commit`
用户需要：Node.js + npm
产物大小：KB 级（源码安装，无需下载大文件）

> **注意：** `package.json` 必须包含 `"bin"` 字段，入口文件首行必须有 `#!/usr/bin/env node`。
> 如果用 TypeScript 编写，请先编译为 JS 再提交到源码仓库。

#### Python

```yaml
build:
  lang: python
  source_repo: "your-org/your-python-tool"
  source_commit: "d4e5f6789012345678901234567890abcdef0123"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/main.py"                    # Python 必填
```

分发方式：`pip install git+https://github.com/your-org/your-python-tool@commit`
用户需要：Python 3.8+ 和 pip/pip3
产物大小：KB 级（源码安装）

> **注意：** `pyproject.toml` 必须包含 `[build-system]`、`[project]` 和 `[project.scripts]`。
> 推荐同时提供 `setup.py` 以兼容旧版 pip。入口函数必须是 `def main()`。

#### Node.js

```yaml
build:
  lang: node
  source_repo: "your-org/your-node-tool"
  source_commit: "e5f6789012345678901234567890abcdef012345"
  source_dir: "."
  binary_name: "your-tool"
  main: "src/index.js"                   # Node.js 必填
```

分发方式：`npm install -g git+https://github.com/your-org/your-node-tool#commit`
用户需要：Node.js + npm
产物大小：KB 级（源码安装）

> **注意：** `package.json` 必须包含 `"bin"` 字段，入口文件首行必须有 `#!/usr/bin/env node`。

> Node.js 和 TypeScript 使用相同的分发方式（npm install）。唯一区别是 TypeScript 需要先编译为 JS。

### SKILL.md 作为编排者

你的 SKILL.md 告诉 AI Agent 如何同时使用 onchainos 命令和你的 binary 工具：

```markdown
## Commands

### 查询收益（使用你的 binary 工具）
调用 binary 工具 `calculate_yield`，传入池子地址和链。

### 执行存入（使用 onchainos + 你的 binary）
1. 调用 binary 工具 `find_best_route` 寻找最优路径
2. 执行 `onchainos swap quote --from USDC --to POOL_TOKEN`
3. **请用户确认** 金额和预期收益
4. 执行 `onchainos swap swap ...`
5. 调用 binary 工具 `monitor_position` 开始监控
```

### 不允许的操作

- 提交预编译的二进制文件（.exe、.dll、.so、.wasm）— E130
- 声明 Binary 但没有 build 配置 — E110/E111
- 源码大于 10MB — E126
- 编译脚本在编译期间从网络下载内容

---

<a id="section-14"></a>

## 14. onchainos 命令参考

你的 SKILL.md 只应使用 onchainos CLI 命令。以下是可用的顶级命令：

| 命令 | 说明 | 示例 |
|------|------|------|
| `onchainos token` | 代币搜索、信息、趋势、持仓者 | `onchainos token search --query SOL` |
| `onchainos market` | 价格、K 线图、组合 PnL | `onchainos market price --address 0x... --chain ethereum` |
| `onchainos swap` | DEX 交换报价和执行 | `onchainos swap quote --from ETH --to USDC --amount 1` |
| `onchainos gateway` | Gas 估算、交易模拟、广播 | `onchainos gateway gas --chain ethereum` |
| `onchainos portfolio` | 钱包总价值和余额 | `onchainos portfolio all-balances --address 0x...` |
| `onchainos wallet` | 登录、余额、转账、历史 | `onchainos wallet balance --chain solana` |
| `onchainos security` | 代币扫描、DApp 扫描、交易扫描 | `onchainos security token-scan --address 0x...` |
| `onchainos signal` | 智能资金 / 鲸鱼信号 | `onchainos signal list --chain solana` |
| `onchainos memepump` | Meme 代币扫描和分析 | `onchainos memepump tokens --chain solana` |
| `onchainos leaderboard` | 按 PnL/交易量排名的顶级交易者 | `onchainos leaderboard list --chain solana` |
| `onchainos payment` | x402 支付协议 | `onchainos payment x402-pay --url ...` |

要查看完整的子命令列表，运行 `onchainos <命令> --help` 或参阅 [onchainos 文档](https://github.com/okx/onchainos-skills)。

---

<a id="section-15"></a>

## 15. 常见问题

**Q: 用户如何安装我发布的 plugin？**
A: 用户运行 `npx skills add okx/plugin-store-community --skill <your-plugin-name>`。这在空白环境中即可工作 — AI Agent 的 pre-flight 会自动处理依赖安装（onchainos、binary、pip 包、npm 包），无需用户预先安装 plugin-store CLI。

**Q: 我可以直接调用外部 API 吗？**
A: 查询外部数据源是允许的（第三方 DeFi API、行情数据等）。但所有链上交互操作（签名、广播、Swap、合约调用）必须通过 onchainos CLI。如果你需要 onchainos 尚未提供的链上能力，请在 onchainos 仓库提交 feature request。

**Q: 我可以包含binary 吗？**
A: 可以。任何开发者都可以提交 Binary 源码。将源码放在你自己的 GitHub 仓库中，在 plugin.yaml 中添加 `build` 配置，包含 `source_repo` 和 `source_commit`。我们的 CI 负责编译。详见第 13 节。

**Q: 审核需要多长时间？**
A: 自动化检查约 5 分钟完成。人工审核通常需要 1-3 个工作日。

**Q: AI 审查标记了问题怎么办？**
A: AI 审查仅供参考 — 不会阻止你的 PR。但人工审核者会阅读 AI 报告。建议解决标记的问题以加快审批速度。

**Q: 发布后可以更新 plugin 吗？**
A: 可以。提交新的 PR，包含更新后的文件和升级的版本号。


**Q: 本地 `plugin-store lint` 通过了，但 GitHub 检查失败？**
A: 确保你使用的是最新版本的 plugin-store CLI。同时确保 PR 只修改了 `submissions/你的 plugin 名/` 目录下的文件。

**Q: 错误 E122 "source_repo is not valid" 是什么意思？**
A: `build.source_repo` 必须是 `owner/repo` 格式（如 `your-username/my-server`）。不要包含 `https://github.com/` 或 `.git`。

**Q: 错误 E123 "must be a full 40-character hex SHA" 是什么意思？**
A: `build.source_commit` 必须是完整的 40 位提交哈希，不能是短 SHA 或分支名。在你的源码仓库中运行 `git rev-parse HEAD` 获取完整哈希。

**Q: 错误 E120 "must also include a Skill component" 是什么意思？**
A: 每个包含 `build` 配置的 plugin 都必须有 SKILL.md。Skill 是入口 — 它告诉 AI Agent 如何使用你的 binary 或二进制。

**Q: 错误 E130 "pre-compiled binary file is not allowed" 是什么意思？**
A: 你在提交目录中包含了编译好的文件（.exe、.dll、.so、.wasm 等）。请删除它 — 我们从你的源码编译，你不需要提交二进制。

**Q: 错误 E110/E111 "requires a build section" 是什么意思？**
A: 你声明了 Binary 组件但没有 `build` 配置。我们需要知道你的源码在哪里才能编译。添加 `build.lang`、`build.source_repo` 和 `build.source_commit`。

**Q: CI 中编译失败了，但我本地可以编译。为什么？**
A: 我们的 CI 在 Ubuntu Linux 上编译。确保你的代码能在 Linux 上编译，而不仅仅是 macOS/Windows。查看 GitHub Actions 运行日志获取具体错误信息。

---

<a id="section-16"></a>

## 16. 获取帮助

- 在 GitHub 上提交 [issue](https://github.com/okx/plugin-store-community/issues)
- 查看 `submissions/_example-plugin/` 获取完整的参考 plugin
- 提交前在本地运行 `plugin-store lint` — 它能发现大部分问题
- 如果 PR 检查失败，查看 [GitHub Actions 日志](https://github.com/okx/plugin-store-community/actions)
