---
name: signal-tracker-go
description: "Track smart money wallet activity and aggregated buy signals (go implementation)"
version: "1.0.0"
author: "Mingtao"
tags:
  - smart-money
  - signals
  - wallet-tracker
---

# signal-tracker-go

## Overview

signal-tracker-go 让 AI Agent 能够追踪聪明钱钱包的交易动态，并获取聚合买入信号。
它提供 `signal-tracker-go` CLI 工具用于格式化展示，所有链上数据查询通过 `onchainos` CLI 完成。

> 所有链上交互操作（签名、广播、Swap、合约调用）必须通过 onchainos CLI。

## Pre-flight Checks

使用前确认：

1. `onchainos` CLI 已安装并完成身份认证
2. `signal-tracker-go` 二进制已安装（通过 `plugin-store install signal-tracker-go`）
3. 运行 `signal-tracker-go --version` 验证安装
4. 网络连接正常

## Commands

### 追踪聪明钱钱包交易

```bash
signal-tracker-go track --address <WALLET_ADDRESS> --chain ethereum
```

**When to use**: 用户提供钱包地址，想查看该地址的最新 DEX 交易记录。  
**Output**: 该地址最近的买入/卖出记录，包含代币、金额、时间。

底层调用：
```bash
onchainos signal address-tracker --address <WALLET_ADDRESS> --chain ethereum
```

---

### 获取聚合买入信号

```bash
signal-tracker-go signals --chain ethereum
```

**When to use**: 用户想知道聪明钱/KOL 当前集中买入了哪些代币。  
**Output**: 按买入次数排序的代币列表，附带代币地址和参与钱包数。

底层调用：
```bash
onchainos signal buy-signals --chain ethereum
```

---

### 查询代币价格

```bash
signal-tracker-go price --address <TOKEN_ADDRESS> --chain ethereum
```

**When to use**: 用户想确认信号中某个代币的当前价格。  
**Output**: 代币价格（USD）、24h 涨跌幅、成交量。

底层调用：
```bash
onchainos market price --address <TOKEN_ADDRESS> --chain ethereum
```

---

### 执行 Swap

如果用户看到信号后想买入，**必须按以下步骤操作**：

**Step 1 — 获取报价**

```bash
onchainos swap quote --from USDC --to <TOKEN_ADDRESS> --amount 100 --chain ethereum
```

**Step 2 — Require user confirmation before executing**

Show the user the quote details (expected output amount, slippage, fees).
**You MUST explicitly ask the user to confirm before proceeding.**
Do NOT execute the swap unless the user explicitly confirms. If the user does not confirm, abort the operation.

**Step 3 — Only after user confirmation, execute the swap**

```bash
onchainos swap swap --from USDC --to <TOKEN_ADDRESS> --amount 100 --chain ethereum
```

## Error Handling

| Error | Cause | Resolution |
|-------|-------|------------|
| `Failed to run onchainos` | onchainos 未安装或未在 PATH | 运行 `onchainos --version` 检查安装 |
| `address-tracker: not found` | onchainos 版本过旧 | 升级 onchainos |
| `Chain not supported` | 链参数错误 | 支持的链：ethereum, solana, base, bsc |
| `Rate limited` | 请求过于频繁 | 等待 10 秒后重试 |

## Skill Routing

- 执行 Swap → 使用 `okx-dex-swap` skill
- 查钱包余额 → 使用 `okx-wallet-portfolio` skill
- 代币安全扫描 → 使用 `okx-security` skill
- Meme 代币分析 → 使用 `okx-dex-trenches` skill
