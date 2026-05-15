# ⚡ OrbitStream Contracts

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.74+-orange)](https://www.rust-lang.org/)
[![Soroban SDK](https://img.shields.io/badge/Soroban-SDK-7C68EE)](https://soroban.stellar.org/)
[![Stellar](https://img.shields.io/badge/Stellar-Testnet-blue)](https://stellar.org/)

> **Soroban smart contracts powering real-time token streaming on Stellar.**

OrbitStream Contracts is the on-chain layer of the OrbitStream protocol. A single Rust/Soroban contract handles the full stream lifecycle — from creation and top-ups to per-second claiming, pausing, resuming, and cancellation with prorated settlement.

---

## ✨ What the contract does

- 🌊 **Continuous streaming** — tokens accrue every active second using `env.ledger().timestamp()`
- ⏸️ **Pause & resume** — senders freeze the clock; paused time is excluded from earnings
- 💸 **Cancel & settle** — recipient gets earned tokens; sender gets unearned funds back
- 🔒 **Solvency invariant** — claimable always capped at `total_deposited - total_claimed`
- ⛽ **Top-up** — add more tokens to extend stream runway at any time
- 🔁 **Upgradeable** — admin pushes new WASM without redeploying via `upgrade()`

---

## 🗂️ Structure

```
Contract/
├── Cargo.toml
└── contracts/
    ├── hello-world/       # Soroban starter example
    └── stream/            # OrbitStream production contract
        ├── Cargo.toml
        └── src/lib.rs     # Full contract + 8 test cases
```

---

## 🚀 Getting Started

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install stellar-cli --features opt
```

### Build

```bash
cd Contract
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/orbitstream_stream.wasm \
  --source <your-keypair> \
  --network testnet
```

---

## 📖 Contract API

| Function | Caller | Description |
|----------|--------|-------------|
| `initialize(admin)` | Admin | One-time setup |
| `create_stream(sender, recipient, token, rate, duration, deposit)` | Sender | Open a new stream |
| `top_up(sender, stream_id, amount)` | Sender | Add more tokens |
| `claim(recipient, stream_id)` | Recipient | Withdraw accrued tokens |
| `pause_stream(sender, stream_id)` | Sender | Freeze the clock |
| `resume_stream(sender, stream_id)` | Sender | Unfreeze the clock |
| `cancel_stream(sender, stream_id)` | Sender | Close with settlement |
| `admin_cancel(stream_id)` | Admin | Emergency cancel |
| `get_stream(stream_id)` | Anyone | Read stream state |
| `claimable_amount(stream_id)` | Anyone | Preview claimable tokens |

---

## 🧪 Tests

8 test cases — happy path, solvency cap, pause/resume, cancel settlement, top-up, auto-complete, self-stream rejection, stream count.

---

## 🔗 Related Repos

- [OrbitStream Backend](https://github.com/OrbitStream/OrbitStream_backend) — NestJS API
- [OrbitStream Frontend](https://github.com/OrbitStream/orbitstream-frontend) — Web dashboard
- [OrbitStream Docs](https://github.com/OrbitStream/orbitstream-docs) — Documentation

---

## 📜 License

MIT License. Copyright (c) 2026 OrbitStream Protocol.
