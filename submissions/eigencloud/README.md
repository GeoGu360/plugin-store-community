# EigenCloud — EigenLayer Restaking Plugin

EigenLayer AVS restaking integration for onchainos. Stake LSTs, delegate to operators, and manage withdrawals on Ethereum mainnet.

## Features

- **strategies** — List EigenLayer LST strategies with TVL
- **operators** — Browse active operators available for delegation
- **positions** — View your restaked positions and delegation status
- **stake** — Deposit stETH/rETH/cbETH into EigenLayer strategies
- **delegate** — Delegate stake to an operator for AVS rewards
- **undelegate** — Undelegate (queues withdrawal for all shares)
- **queue-withdrawal** — Queue a partial withdrawal from a strategy

## Supported Chains

- Ethereum mainnet (chain ID 1)

## Usage

```bash
# Read operations
eigencloud strategies
eigencloud operators
eigencloud positions --address 0xYourAddress

# Write operations (confirm before submitting)
eigencloud stake --symbol stETH --amount 1.0 --dry-run
eigencloud delegate --operator 0xOperatorAddress --dry-run
eigencloud undelegate --dry-run
eigencloud queue-withdrawal --symbol stETH --dry-run
```

## Key Contracts

| Contract | Ethereum Mainnet |
|---|---|
| StrategyManager | `0x858646372CC42E1A627fcE94aa7A7033e7CF075A` |
| DelegationManager | `0x39053D51B77DC0d36036Fc1fCc8Cb819df8Ef37b` |

## License

MIT
