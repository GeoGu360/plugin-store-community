---
name: eigencloud
description: EigenLayer AVS restaking plugin. Stake LSTs (stETH, rETH, cbETH) into EigenLayer strategies, delegate to operators for AVS rewards, and manage withdrawal queues on Ethereum mainnet (chain ID 1).
---

# EigenCloud — EigenLayer Restaking Plugin

## Overview

This plugin enables interaction with EigenLayer, the leading Ethereum restaking protocol. Users can:
- Deposit LSTs (stETH, rETH, cbETH, etc.) into EigenLayer strategies to earn restaking yield
- Delegate their stake to operators running AVS (Actively Validated Service) infrastructure
- Manage withdrawals with the EigenLayer queue system

**Supported chains**: Ethereum mainnet (chain ID 1) only
**Protocol**: EigenLayer v2

## Architecture

- Read ops (strategies, operators, positions) → direct eth_call via public Ethereum RPC
- Write ops → after user confirmation, submit via `onchainos wallet contract-call`

## Pre-flight Checks

Before any write operation:
1. Verify `onchainos` is installed: `onchainos --version`
2. For write operations: `onchainos wallet balance --chain 1 --output json`
3. If wallet check fails: "Please log in with `onchainos wallet login` first."

## Contract Addresses (Ethereum Mainnet)

| Contract | Address |
|---|---|
| StrategyManager | `0x858646372CC42E1A627fcE94aa7A7033e7CF075A` |
| DelegationManager | `0x39053D51B77DC0d36036Fc1fCc8Cb819df8Ef37b` |
| EigenPodManager | `0x91E677b07F7AF907ec9a428aafa9fc14a0d3A338` |

## Strategy Addresses

| Symbol | Strategy | Token |
|---|---|---|
| stETH | `0x93c4b944D05dfe6df7645A86cd2206016c51564D` | `0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84` |
| rETH | `0x1BeE69b7dFFfA4E2d53C2a2Df135C388AD25dCD2` | `0xae78736Cd615f374D3085123A210448E74Fc6393` |
| cbETH | `0x54945180dB7943c0ed0FEE7EdaB2Bd24620256bc` | `0xBe9895146f7AF43049ca1c1AE358B0541Ea49704` |

---

## Commands

### `strategies` — List Available Strategies

Display all EigenLayer LST strategies with TVL information.

**Usage:**
```
eigencloud strategies
```

**Example:**
```bash
eigencloud strategies
```

---

### `operators` — List Active Operators

Display known EigenLayer operators available for delegation, including their delegation approval type.

**Usage:**
```
eigencloud operators
```

**Example:**
```bash
eigencloud operators
```

---

### `positions` — Show Restaked Positions

Show all restaked positions and current delegation status for a wallet.

**Usage:**
```
eigencloud positions [--address <ADDR>]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--address` | No | Wallet address (resolved from onchainos if omitted) |

**Example:**
```bash
eigencloud positions
eigencloud positions --address 0x87fb0647faabea33113eaf1d80d67acb1c491b90
```

---

### `stake` — Stake LST into Strategy

Deposit LST tokens into an EigenLayer strategy to earn restaking yield.

**Usage:**
```
eigencloud stake --symbol <SYMBOL> --amount <AMOUNT> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--symbol` | No | LST symbol: stETH, rETH, cbETH, ETHx, ankrETH (default: stETH) |
| `--amount` | Yes | Amount of LST to stake (e.g. `1.5`) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**
1. Check LST balance — abort if insufficient
2. Check StrategyManager allowance — approve if needed
3. Show user: amount, strategy, contract addresses
4. **Ask user to confirm** before each transaction
5. Execute approve (if needed) then depositIntoStrategy

**Calldata:**
- Approve: `0x095ea7b3` + StrategyManager address + amount (uint256)
- Deposit: `0xe7a050aa` + strategy + token + amount

**Example:**
```bash
# Stake 1.0 stETH (dry-run first to preview)
eigencloud stake --symbol stETH --amount 1.0 --dry-run
eigencloud stake --symbol stETH --amount 1.0

# Stake rETH
eigencloud stake --symbol rETH --amount 0.5
```

---

### `delegate` — Delegate to Operator

Delegate your restaked stake to an operator for AVS rewards.

**Usage:**
```
eigencloud delegate --operator <ADDR> [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--operator` | Yes | Operator address to delegate to |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Steps:**
1. Check if already delegated — if so, suggest `undelegate` first
2. Show operator details
3. **Ask user to confirm** before submitting
4. Call `DelegationManager.delegateTo(operator, emptySignature, zeroSalt)`

**Calldata:** `0xeea9064b` + operator + struct_offset(0x60) + salt(0) + sig_offset(0x40) + expiry(0) + sig_len(0)

**Example:**
```bash
# Delegate to P2P.org
eigencloud delegate --operator 0x71c6F7Ed8C2d4925d0bAf16f6A85BB1736D412f

# Preview calldata
eigencloud delegate --operator 0x71c6F7Ed8C2d4925d0bAf16f6A85BB1736D412f --dry-run
```

---

### `undelegate` — Undelegate from Operator

Undelegate from current operator. This automatically queues a withdrawal for all restaked shares.

**Usage:**
```
eigencloud undelegate [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Important:**
- Undelegating queues a withdrawal for ALL shares (~7 day delay)
- **Ask user to confirm** — this is irreversible until withdrawal completes

**Calldata:** `0xda8be864` + staker address

**Example:**
```bash
eigencloud undelegate --dry-run
eigencloud undelegate
```

---

### `queue-withdrawal` — Queue Withdrawal of Shares

Queue a withdrawal for specific shares in a strategy (without undelegating).

**Usage:**
```
eigencloud queue-withdrawal [--strategy <ADDR> | --symbol <SYMBOL>] [--shares <AMOUNT>] [--withdrawer <ADDR>] [--from <ADDR>] [--dry-run]
```

**Parameters:**
| Parameter | Required | Description |
|---|---|---|
| `--strategy` | Yes* | Strategy address (*or use --symbol) |
| `--symbol` | Yes* | LST symbol shortcut (*or use --strategy) |
| `--shares` | No | Shares to withdraw in wei (0 = all shares) |
| `--withdrawer` | No | Address to receive withdrawal (defaults to caller) |
| `--from` | No | Wallet address (resolved from onchainos if omitted) |
| `--dry-run` | No | Show calldata without broadcasting |

**Important:**
- ~7 day withdrawal delay before completing
- **Ask user to confirm** before submitting

**Calldata:** `0x0dd8dd02` + ABI-encoded QueuedWithdrawalParams array

**Example:**
```bash
# Queue withdrawal of all stETH shares (dry-run)
eigencloud queue-withdrawal --symbol stETH --dry-run
eigencloud queue-withdrawal --symbol stETH
```

---

## Notes for AI Agent

- EigenLayer restaking requires holding LSTs (stETH, rETH, cbETH, etc.) — not plain ETH
- Native ETH restaking via EigenPod is NOT supported by this plugin
- Delegation to open operators does NOT require a signature
- Always check positions before suggesting delegate/undelegate
- Withdrawal delay is ~7 days (EigenLayer v2 parameter)
- For test environments: use `--dry-run` for all write ops to avoid accidental transactions
