# Smart Money Copy Trade

A copy-trading strategy plugin for OnchaInOS that monitors smart money, whale, and KOL buy signals, then executes follow-up trades with built-in security screening and risk management.

## What it does

- Scans real-time smart money / whale / KOL buy signals across Solana, Ethereum, Base, and BSC
- Scores signal strength by wallet count, sold ratio, and trade amount
- Runs mandatory security scans before every trade (honeypot, rug-pull, tax detection)
- Validates token fundamentals (liquidity, market cap, holder distribution)
- Calculates position size based on configurable risk parameters
- Executes trades via onchainos swap with full risk controls
- Monitors positions and generates exit signals

## Strategy Pipeline

```
Signal → Screen → Size → Execute → Monitor
```

## Permissions

- Read balance: Yes (for position sizing)
- Send transactions: Yes (swap execution via onchainos CLI)
- Sign messages: Yes (via Agentic Wallet)
- Contract calls: Yes (approve + swap)

## Risk Controls

- Honeypot detection (auto-block)
- Price impact gates (5% warn, 10% block)
- Tax token disclosure
- Liquidity minimums ($50k default)
- Position size limits (2% of portfolio default)
- Stop-loss / take-profit alerts

## License

Apache-2.0
