# 🏥 RPC Health Checker

A Rust tool that benchmarks multiple Ethereum RPC endpoints concurrently and ranks them by response time.

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
