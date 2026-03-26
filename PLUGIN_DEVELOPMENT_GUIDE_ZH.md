# plugin 开发与提交指南

> 本指南将带你完成 Plugin Store plugin 的开发和提交全流程。读完本文，你将拥有一个可以与 onchainos CLI 集成的完整 plugin。

---

## 目录

1. [什么是 plugin？](#section-1)
2. [开始之前](#section-2)
3. [第一步：克隆仓库并生成 plugin 脚手架](#section-3)
4. [第二步：编写 plugin.yaml](#section-4)
5. [第三步：编写 SKILL.md](#section-5)
6. [第四步：声明 API 调用](#section-6)
7. [第五步：本地验证](#section-7)
8. [第六步：通过 Pull Request 提交](#section-8)
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
- 安装 **plugin-store CLI**：
 ```bash
 # macOS / Linux
 curl -fsSL https://raw.githubusercontent.com/yz06276/plugin-store/main/install-local.sh | bash
 ```
- 安装 **onchainos CLI**（用于测试命令）：
 ```bash
 curl -fsSL https://raw.githubusercontent.com/okx/onchainos-skills/main/install.sh | bash
 ```
- 了解你的 plugin 所涉及的区块链/DeFi 领域

### 核心规则

> **所有链上交互操作必须通过 onchainos CLI。** 包括：钱包签名、交易广播、Swap 执行、合约调用等任何写入区块链的操作。你**可以自由**查询外部数据源（第三方 DeFi API、行情数据提供商、分析服务等），但任何链上操作必须走 onchainos。绕过 onchainos 进行链上操作的 plugin 将被拒绝。

---

<a id="section-3"></a>

## 3. 第一步：克隆仓库并生成 plugin 脚手架

先克隆 community 仓库，再在里面运行 `init`：

```bash
git clone git@github.com:yz06276/plugin-store-community.git
cd plugin-store-community
plugin-store init my-awesome-plugin
```

`init` 自动检测到 `submissions/` 目录，直接在里面创建你的 plugin：

```
submissions/my-awesome-plugin/
├── plugin.yaml                        # plugin 清单（需要填写）
├── skills/
│   └── my-awesome-plugin/
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
alias: "Solana 价格查询" # 可选：展示名称（支持中文）
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

components:
 skill:
 dir: skills/sol-price-checker # SKILL.md 所在目录的路径

 - solana

api_calls: []

 protocols: [] # 例如 [uniswap-v3, raydium]
```

### 4B. Skill + Binary 示例（含源码编译）

如果你的 plugin 包含 binary 或二进制，需要添加 `build` 配置。源码在你自己的 GitHub 仓库中 — 我们来编译。

```yaml
schema_version: 2
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
- `schema_version: 2`（不是 1）
- 声明了 ``components.binary``
- 包含 `build` 配置，含 `source_repo` + `source_commit`
- 我们的 CI 从你的仓库克隆、编译、发布

**如何获取 commit SHA：**
```bash
cd your-source-repo
git rev-parse HEAD
# 输出：a1b2c3d4e5f6789012345678901234567890abcd
```

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
name: my-awesome-plugin
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
| Binary 连接失败 | Server 未运行 | 执行 `plugin-store install defi-yield-optimizer` |
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
4. **包含前置检查** — 使用你的技能前需要满足什么条件
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
plugin-store lint ./my-awesome-plugin/
```

### 全部通过时：

```
Linting ./my-awesome-plugin/...

✓ Plugin 'my-awesome-plugin' passed all checks!
```

### 有错误时：

```
Linting ./my-awesome-plugin/...

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

你已经在第一步克隆了仓库，plugin 也在 `submissions/` 里了，直接创建分支并推送：

```bash
git checkout -b submit/my-awesome-plugin
git add submissions/my-awesome-plugin/
git commit -m "[new-plugin] my-awesome-plugin v1.0.0"
git push origin submit/my-awesome-plugin
```

然后在 GitHub 上创建 Pull Request。

在 GitHub 上从你的分支创建 Pull Request。使用以下标题格式：

```
[new-plugin] my-awesome-plugin v1.0.0
```

PR 模板会引导你完成检查清单。

### PR 重要规则

- 每个 PR 只包含 **一个 plugin**
- 只修改 `submissions/你的 plugin 名/` 目录下的文件
- 不要修改其他文件（README.md、workflows 等）
- 目录名必须与 plugin.yaml 中的 `name` 字段一致

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
```

### 人工审核（1-3 天）

维护者会审核：

- plugin 是否有意义？
- api_calls 是否准确？
- SKILL.md 写得好不好？
- 是否存在安全隐患？

### 合并后

你的 plugin 会自动：

1. 添加到主 plugin-store 仓库的 `registry.json` 中
2. 创建 git tag `plugins/my-awesome-plugin@1.0.0`
3. 所有用户可通过 `plugin-store install my-awesome-plugin` 安装

---

<a id="section-10"></a>

## 10. 更新你的 plugin

### 内容更新（修改 SKILL.md、添加命令）

1. 修改 `submissions/my-awesome-plugin/` 下的文件
2. 在 plugin.yaml 中升级 `version`（例如 `1.0.0` → `1.1.0`）
3. 更新 CHANGELOG.md
4. 创建 PR，标题格式：`[update] my-awesome-plugin v1.1.0`

### 链或 API 变更（需要完整审核）

如果你的更新修改了 `api_calls`，审核会更加严格。AI 审查报告会重点标注这些变化。

---

<a id="section-11"></a>

## 11. 规则与限制

### 社区 plugin 可以做的

- 使用 SKILL.md 定义技能
- 引用任何 onchainos CLI 命令
- 包含参考文档
- 声明 api_calls

### 社区 plugin 不能做的

- 包含 binary 组件（代码执行）
- 包含 Binary 组件（代码执行）
- 使用保留名称前缀（`okx-`、`official-`、`plugin-store-`）
- 绕过 onchainos CLI 进行链上操作（钱包签名、交易广播、合约调用等）
- 包含 prompt injection 模式
- 超过文件大小限制（单文件 200KB，总计 5MB）

### 所有开发者可以提交的内容

| 组件 | 方式 |
|------|------|
| Skill (SKILL.md) | 放入 submissions/ 目录 |
| Binary（源码） | 源码放在你自己的 GitHub 仓库，在 plugin.yaml 中添加 `build` 配置，我们编译 |

---

<a id="section-12"></a>

## 12. SKILL.md 写作指南

### 结构清单

- [ ] YAML frontmatter 包含 `name` 和 `description`
- [ ] Overview 部分（这个技能做什么？）
- [ ] Pre-flight Checks 部分（使用前需要什么？）
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
schema_version: 2
name: my-binary-tool
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
 dir: skills/my-binary-tool # SKILL.md 始终必须

build:
 lang: rust # rust | go | typescript | node | python
 source_repo: "your-username/my-binary-tool" # 你的 GitHub 源码仓库
 source_commit: "abc123def456..." # 完整的 40 位 commit SHA（锁定版本）
 source_dir: "." # 仓库内的路径（默认：根目录）
 binary_name: my-binary-tool # 编译产物名
 # main: src/index.ts # TypeScript/Python 需要指定

 - ethereum

api_calls: []
```

### 如何获取 commit SHA

```bash
# 在你的源码仓库中，推送代码后执行：
git rev-parse HEAD
# 输出：342756ee25405b5ec5b375a37c1b36710d5b9cd6
# 把这个完整的 40 位字符串复制到 build.source_commit
```

### 目录结构

源码在你自己的仓库中。你只需要把元数据 + SKILL 提交到 community 仓库：

```
submissions/my-binary-tool/ ← 在 community 仓库中（很小，约 20KB）
 plugin.yaml # 包含 build 配置，指向你的仓库
 skills/my-binary-tool/
 SKILL.md # AI Agent 的入口
 references/
 LICENSE
 CHANGELOG.md
 README.md

your-username/my-binary-tool ← 你自己的 GitHub 仓库（源码）
 Cargo.toml # （Rust 示例）
 src/
 main.rs
 lib.rs
```

### 支持的语言

| 语言 | 入口文件 | 编译工具 | 产物 |
|------|---------|---------|------|
| Rust | `Cargo.toml` | `cargo build --release` | 原生二进制 |
| Go | `go.mod` | `go build` | 原生二进制 |
| TypeScript | `package.json` + `build.main` | `bun build --compile` | 打包二进制 |
| Node.js | `package.json` | `npm publish` | npm 包 |
| Python | `pyproject.toml` + `build.main` | `PyInstaller` | 打包二进制 |

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

- 在 GitHub 上提交 [issue](https://github.com/yz06276/plugin-store-community/issues)
- 查看 `submissions/_example-plugin/` 获取完整的参考 plugin
- 提交前在本地运行 `plugin-store lint` — 它能发现大部分问题
- 如果 PR 检查失败，查看 [GitHub Actions 日志](https://github.com/yz06276/plugin-store-community/actions)
