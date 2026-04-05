
# e2e-node-cli -- Skill Summary

## Overview
This skill provides a Node.js command-line interface for testing argument processing and querying cryptocurrency token prices. It combines basic echo functionality with onchainos integration to retrieve real-time price data for various tokens including Ethereum and Bitcoin, making it useful for end-to-end testing scenarios and price monitoring workflows.

## Usage
Install the CLI via npm and ensure onchainos is available with `onchainos wallet status`. Use the tool to echo test arguments or query cryptocurrency prices through the integrated onchainos system.

## Commands
| Command | Description | Example |
|---------|-------------|---------|
| `e2e-node-cli <args>` | Echo the provided arguments | `e2e-node-cli hello world` |
| `e2e-node-cli price <token> <address>` | Query token price via onchainos | `e2e-node-cli price ethereum 0xeeee...` |
| `onchainos market price --address <addr> --chain <chain>` | Direct onchainos price query | `onchainos market price --address "0x2260..." --chain ethereum` |

## Triggers
An AI agent should activate this skill when users need to test CLI argument processing, query cryptocurrency token prices, or perform end-to-end testing of Node.js applications with price data integration.
