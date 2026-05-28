# OrbitStream Contracts

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-SDK%2021-7C68EE)](https://soroban.stellar.org/)

> **On-chain escrow contracts for OrbitStream — a Stripe-like payment gateway for Stellar.**

OrbitStream is a developer-friendly payment gateway that brings Stripe-like DX to Stellar's native payment rails. This repo contains the Soroban smart contract for escrow functionality — locking funds for marketplace and freelance transactions until released or refunded.

---

## Contract Functions

| Function | Auth | Description |
|----------|------|-------------|
| `create_escrow(buyer, seller, token, amount, timeout_seconds)` | Buyer | Lock funds in escrow |
| `release(escrow_id)` | Seller | Release funds to seller |
| `refund(escrow_id)` | Buyer | Refund after timeout |
| `get_escrow(escrow_id)` | — | Read escrow details |

---

## Data Model

```rust
pub struct Escrow {
    pub id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: u128,
    pub status: EscrowStatus,  // Active, Released, Refunded
    pub created_at: u64,
    pub timeout_at: u64,
}
```

---

## Project Structure

```
src/
├── lib.rs        # Contract entry point (OrbitStream)
├── escrow.rs     # Escrow struct + EscrowStatus enum
├── storage.rs    # Persistence helpers
├── events.rs     # EscrowCreated, EscrowReleased, EscrowRefunded
└── errors.rs     # Error codes
tests/
├── test_create_escrow.rs
├── test_release.rs
└── test_refund.rs
```

---

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

Or with Soroban CLI:

```bash
soroban contract build
```

## Test

```bash
cargo test
```

## Deploy

```bash
soroban contract deploy --wasm target/wasm/orbitstream-contracts.wasm --network testnet --source <KEY>
```

---

## Events

| Event | Fields | When |
|-------|--------|------|
| `escrow_created` | escrow_id, buyer, seller, token, amount, timeout_at | New escrow created |
| `escrow_released` | escrow_id, seller, amount | Seller releases funds |
| `escrow_refunded` | escrow_id, buyer, amount | Buyer refunds after timeout |

---

## OrbitStream Ecosystem

This contract is one piece of the broader OrbitStream platform:

| Component | Description |
|-----------|-------------|
| **JS/React SDK** | `<StellarCheckout amount={25} currency="USDC" />` drop-in widget |
| **Hosted checkout** | Shareable payment links (like Stripe Payment Links) |
| **Backend API** | Payment session management, webhook dispatch, ledger monitoring |
| **Merchant dashboard** | Transaction history, analytics, settings |
| **This contract** | On-chain escrow for dispute-prone payments |

---

## Related Repositories

- [OrbitStream_backend](https://github.com/OrbitStream/OrbitStream_backend) — Backend API
- [orbitstream_frontend](https://github.com/OrbitStream/orbitstream_frontend) — Checkout UI
- [orbitstream_docs](https://github.com/OrbitStream/orbitstream_docs) — Documentation

---

## License

MIT License. Copyright (c) 2026 OrbitStream.
