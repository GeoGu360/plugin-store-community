# GMX V2 Plugin

Trade perpetuals and manage GM pool liquidity on GMX V2 (Arbitrum/Avalanche).

## Supported Operations

- `get-markets` — List all GM markets
- `get-prices` — Fetch oracle prices
- `get-positions` — View open positions
- `open-long` — Open leveraged long
- `open-short` — Open leveraged short
- `close-position` — Close a position
- `swap` — Market swap via GMX
- `deposit-gm` — Add GM pool liquidity
- `withdraw-gm` — Remove GM pool liquidity
- `approve-token` — ERC-20 approve for Router

## Chains

- Arbitrum One (42161) — primary
- Avalanche C-Chain (43114)

## Build

```bash
cargo build --release
```

## Usage

```bash
gmx-v2 --help
gmx-v2 get-markets --chain 42161
gmx-v2 get-prices --chain 42161
gmx-v2 get-positions --chain 42161
```

See `skills/gmx-v2/SKILL.md` for full documentation.
