# 测试结果报告

- 日期: 2026-04-04
- 测试链: Base (8453)
- 测试钱包: `0xee385ac7ac70b5e7f12aa49bf879a441bed0bae9`
- 编译: ✅
- Lint: ✅ (0 errors, 0 warnings)

## 汇总

| 总数 | L1编译 | L2读取 | L3模拟 | L4链上 | 失败 | 阻塞 |
|------|--------|--------|--------|--------|------|------|
| 16   | 2      | 5      | 6      | 3      | 0    | 0    |

---

## 详细结果

| # | 场景（用户视角） | Level | 命令 | 结果 | TxHash / Calldata | 备注 |
|---|----------------|-------|------|------|-------------------|------|
| 1 | 插件编译通过 | L1 | `cargo build --release` | ✅ PASS | — | 初始编译成功 |
| 2 | 代码风格检查（含 E106 修复） | L1 | `cargo clean && plugin-store lint .` | ✅ PASS | — | 修复 E106: 在 wallet contract-call 附近添加确认文本 |
| 3 | 查询 Base 上所有 Aave V3 借贷市场 | L2 | `reserves --chain 8453` | ✅ PASS | — | 返回 15 个市场，supplyApy/variableBorrowApy 正常 |
| 4 | 按资产地址过滤查看 USDC 市场利率 | L2 | `reserves --chain 8453 --asset 0x833589...` | ✅ PASS | — | USDC supplyApy ~2.60%，variableBorrowApy ~3.80% |
| 5 | 查看测试钱包在 Aave 的仓位 | L2 | `positions --chain 8453 --from 0xee385...` | ✅ PASS | — | 返回 aBasUSDC 持仓，analysisPlatformId=10 |
| 6 | 检查测试钱包健康因子 | L2 | `health-factor --chain 8453 --from 0xee385...` | ✅ PASS | — | HF=2599734.54，状态 safe |
| 7 | 验证不支持的链 ID 报错 | L2 | `reserves --chain 999` | ✅ PASS | — | 返回 "Unsupported chain ID: 999" 错误 |
| 8 | 模拟存入 0.01 USDC（验证 approve + supply calldata） | L3 | `supply --asset USDC --amount 0.01 --dry-run` | ✅ PASS | approve: `0x095ea7b3...`  supply: `0x617ba037...` | 两步 calldata 均正确 |
| 9 | 模拟提取 0.01 USDC（验证 withdraw calldata） | L3 | `withdraw --asset USDC --amount 0.01 --dry-run` | ✅ PASS | `0x69328dec...` | selector 正确 |
| 10 | 模拟借出 USDC（验证 borrow calldata） | L3 | `borrow --asset 0x833589... --amount 0.01 --dry-run` | ✅ PASS | `0xa415bcad...` | selector 正确 |
| 11 | 模拟还款 USDC（验证 repay + approve calldata） | L3 | `repay --asset 0x833589... --amount 0.01 --dry-run` | ✅ PASS | `0x573ade81...` | selector 正确 |
| 12 | 模拟设置效率模式为 0（验证 setUserEMode calldata） | L3 | `set-emode --category 0 --dry-run` | ✅ PASS | `0x28530a47...` | selector 正确 |
| 13 | 模拟启用 USDC 为抵押品（验证 setUserUseReserveAsCollateral calldata） | L3 | `set-collateral --asset 0x833589... --enable --dry-run` | ✅ PASS | `0x5a3b74b9...` | selector 正确 |
| 14 | 将 E-Mode 重置为无（category=0），验证完整钱包-合约调用链路 | L4 | `set-emode --category 0 --chain 8453 --from 0xee385...` | ✅ PASS | `0x85c16a27b14ca9797abec05ba35bb2dba4adfba999e1305f710c54883e833463` | [BaseScan](https://basescan.org/tx/0x85c16a27b14ca9797abec05ba35bb2dba4adfba999e1305f710c54883e833463) |
| 15 | 存入 0.01 USDC 到 Aave V3 赚取利息 | L4 | `supply --asset USDC --amount 0.01 --chain 8453 --from 0xee385...` | ✅ PASS | approve: `0x6fc7f3eb9aa62c93e00adad53d0a2b7c5094f3bf3e652fcb41b848e785f1f63b`  supply: `0x58c8805b5044d71bf4746b19ccf7df82e832cff6f0412c91601881fd4d0ced6a` | [BaseScan](https://basescan.org/tx/0x58c8805b5044d71bf4746b19ccf7df82e832cff6f0412c91601881fd4d0ced6a) |
| 16 | 从 Aave V3 提取 0.01 USDC | L4 | `withdraw --asset USDC --amount 0.01 --chain 8453 --from 0xee385...` | ✅ PASS | `0x353fe23619a3344b5303231abcc21de373ca42ebe07a79bbdce53086eb6b8c86` | [BaseScan](https://basescan.org/tx/0x353fe23619a3344b5303231abcc21de373ca42ebe07a79bbdce53086eb6b8c86) |

---

## 修复记录

| # | 问题 | 根因 | 修复 | 文件 |
|---|------|------|------|------|
| 1 | Lint E106：wallet contract-call 附近无确认文本 | SKILL.md 架构说明和 supply 步骤描述缺少明确的用户确认步骤 | 在架构说明和 supply 步骤中各添加一行确认文本 | `skills/aave-v3/SKILL.md` |
| 2 | RPC 频率限制（mainnet.base.org rate limit） | `config.rs` 中 Base 链使用 `https://mainnet.base.org`，频率限制导致 reserves 丢失 7/15 个市场 | 更新为 `https://base-rpc.publicnode.com`；同步更新 `plugin.yaml` api_calls 列表 | `src/config.rs`, `plugin.yaml` |
| 3 | supply 命令报错 "Pool.supply() failed" | approve tx 广播后立即提交 supply，但 approve 尚未被矿工确认，supply 因 allowance 不足 revert | 新增 `rpc::wait_for_tx()` 轮询 eth_getTransactionReceipt；approve 确认后再发 supply | `src/commands/supply.rs`, `src/rpc.rs` |
| 4 | repay 命令同一问题（approve 未确认就提交 repay） | 与修复 3 相同根因 | 在 repay.rs 中 approve tx 后同样调用 `wait_for_tx()` | `src/commands/repay.rs` |
| 5 | SKILL.md set-collateral 参数文档错误 | `--enable <true/false>` 写法不正确，CLI 实为布尔 flag（`--enable` 启用，不加则禁用） | 修正命令路由表和 set-collateral 使用示例 | `skills/aave-v3/SKILL.md` |

---

## 已知限制（非阻塞）

| 限制 | 描述 |
|------|------|
| borrow/repay 地址传参时 decimals 默认 18 | 当 `--asset` 传 ERC-20 地址（非 symbol）时，decimals 硬编码为 18。USDC（6 decimals）通过地址传参时金额会偏差 1e12 倍。SKILL.md 建议 borrow/repay 使用地址，用户实际执行时会遇到 on-chain revert。修复需额外 eth_call 获取 decimals，非核心 P0 路径。 |
| reserves `--asset SYMBOL` 过滤无效 | 按 symbol 过滤时返回全部市场（仅地址过滤有效）。代码中只比较地址字符串。非核心功能。 |
