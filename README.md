# Stellar Checkout Contracts

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![Soroban](https://img.shields.io/badge/Soroban-SDK%2021-7C68EE)](https://soroban.stellar.org/)

> **Soroban smart contract for Stellar Checkout — escrow functionality for dispute-prone payments.**

This contract manages escrowed payments for marketplace and freelance transactions. Buyers deposit funds that are locked until released by the seller or refunded after a timeout period.

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
├── lib.rs        # Contract entry point (StellarCheckoutEscrow)
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

## Related Repositories

- [OrbitStream_backend](https://github.com/OrbitStream/OrbitStream_backend) — Backend API
- [orbitstream_frontend](https://github.com/OrbitStream/orbitstream_frontend) — Checkout UI
- [orbitstream_docs](https://github.com/OrbitStream/orbitstream_docs) — Documentation

---

## License

MIT License. Copyright (c) 2026 OrbitStream.
