# 🏥 RPC Health Checker

A robust monitoring utility designed to evaluate the performance and reliability of blockchain RPC nodes. It tracks latency, block height synchronization, and API availability across multiple networks simultaneously. This tool is indispensable for infrastructure providers and dApp developers who demand high availability and consistency from their node providers.

## Features

- Tests multiple public Ethereum RPC endpoints in parallel
- Measures response latency in milliseconds
- Ranks endpoints from fastest to slowest
- Detects and filters out unreachable endpoints

## Usage

```bash
cargo run
```

### Sample Output

```
Starting RPC Health Check...

Results (Sorted by Response Time):
🏆 https://rpc.ankr.com/eth - 120ms
✅ https://cloudflare-eth.com - 185ms
✅ https://eth.public-rpc.com - 210ms
✅ https://eth-mainnet.public.blastapi.io - 340ms
```

## Tech Stack

- `reqwest` — HTTP client
- `futures` — Concurrent async execution
- `serde` — JSON-RPC request serialization
- `tokio` — Async runtime
