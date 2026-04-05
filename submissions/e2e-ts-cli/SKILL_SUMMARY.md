
# e2e-ts-cli -- Skill Summary

## Overview
This TypeScript CLI tool provides basic echo functionality and cryptocurrency price querying capabilities. It integrates with the onchainos system to fetch real-time token prices for Ethereum and WBTC, making it useful for testing CLI interactions and obtaining market data through simple command-line operations.

## Usage
Install via npm and ensure onchainos CLI is available. Use the CLI commands to echo text or query cryptocurrency prices with simple command syntax.

## Commands
| Command | Description | Example |
|---------|-------------|---------|
| `e2e-ts-cli <args>` | Echo arguments back to console | `e2e-ts-cli hello world` |
| `e2e-ts-cli price ethereum <address>` | Query ETH price via CLI | `e2e-ts-cli price ethereum 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| `onchainos market price --address <addr> --chain ethereum` | Query token price directly | `onchainos market price --address "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" --chain ethereum` |

## Triggers
An AI agent should activate this skill when users need to test CLI functionality with echo commands or request current cryptocurrency prices for ETH or WBTC tokens.
