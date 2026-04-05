
# e2e-ts-cli -- Skill Summary

## Overview
This skill provides a TypeScript-based CLI tool that combines basic argument echoing functionality with cryptocurrency price querying through onchainos integration. It serves as both a utility for testing CLI workflows and a practical tool for retrieving real-time token prices, particularly focusing on Ethereum and Bitcoin-based tokens.

## Usage
Install the `e2e-ts-cli` binary and ensure `onchainos` CLI is authenticated via `onchainos wallet status`. Use the tool by running commands like `e2e-ts-cli hello world` for testing or `e2e-ts-cli price ethereum <address>` for price queries.

## Commands
| Command | Description | Example |
|---------|-------------|---------|
| `e2e-ts-cli <args>` | Echo arguments for testing | `e2e-ts-cli hello world` |
| `e2e-ts-cli price ethereum <address>` | Query ETH price via onchainos | `e2e-ts-cli price ethereum 0xeeee...` |
| `onchainos market price --address <addr> --chain ethereum` | Direct WBTC price query | `onchainos market price --address "0x2260..." --chain ethereum` |

## Triggers
An AI agent should activate this skill when users need to test CLI functionality, query cryptocurrency prices (especially ETH or WBTC), or perform e2e testing of TypeScript CLI applications. It's particularly useful for price discovery and CLI workflow validation scenarios.
