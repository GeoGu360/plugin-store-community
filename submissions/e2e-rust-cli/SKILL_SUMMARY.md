
# e2e-rust-cli -- Skill Summary

## Overview
This skill provides a Rust-based CLI tool designed for end-to-end testing scenarios. It combines basic echo functionality with cryptocurrency price querying capabilities through onchainos integration, allowing users to test both simple command execution and external API interactions in a unified testing environment.

## Usage
Ensure the e2e-rust-cli binary and onchainos CLI are installed and authenticated. Use the tool for testing echo functionality or querying cryptocurrency prices with simple command-line invocations.

## Commands
| Command | Description |
|---------|-------------|
| `e2e-rust-cli hello world` | Echo arguments back to console |
| `e2e-rust-cli price ethereum 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` | Query ETH price via CLI binary |
| `onchainos market price --address "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" --chain ethereum` | Query BTC/WBTC price directly through onchainos |

## Triggers
Activate this skill when users need to test basic CLI functionality, query cryptocurrency prices (ETH or BTC), or perform end-to-end testing of Rust CLI applications with external API integrations.
