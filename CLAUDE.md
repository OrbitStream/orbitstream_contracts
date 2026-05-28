# OrbitStream Contracts

## Project
Soroban smart contracts for OrbitStream — a developer-friendly payment gateway for Stellar. This repo contains the on-chain escrow contract used for dispute-prone payments (marketplace, freelance, subscriptions).

## Stack
- Rust (edition 2021)
- Soroban SDK v21
- Stellar Testnet / Mainnet

## What This Contract Does
Manages escrowed payments for marketplace and freelance transactions. Buyers deposit funds
that are locked until released by seller or refunded after timeout.

## Contract Functions
- `create_escrow(buyer, seller, token, amount, timeout_seconds) -> escrow_id` — lock funds
- `release(escrow_id)` — seller releases funds to themselves
- `refund(escrow_id)` — buyer claims refund after timeout
- `get_escrow(escrow_id) -> Escrow` — read escrow details

## Data Structure
Escrow: id, buyer, seller, token, amount, status (Active/Released/Refunded), created_at, timeout_at

## File Structure
src/
  lib.rs        - contract entry point (OrbitStream)
  escrow.rs     - Escrow struct + EscrowStatus enum
  storage.rs    - read/write contract storage
  events.rs     - EscrowCreated, EscrowReleased, EscrowRefunded
  errors.rs     - custom error types
tests/
  test_create_escrow.rs
  test_release.rs
  test_refund.rs

## Key Rules
- Only buyer can create escrow and request refund
- Only seller can release funds
- Refund only valid after timeout
- All state changes emit events
- Protect against invalid amounts and timeouts

## OrbitStream Ecosystem
This contract is the on-chain component of the broader OrbitStream platform:
- **JS/React SDK** — `<StellarCheckout>` drop-in payment widget
- **Hosted checkout page** — shareable payment links
- **Backend API** — payment session management, webhook dispatch, ledger monitoring
- **Merchant dashboard** — transaction history, analytics, settings
- **This contract** — escrow logic for marketplace/freelance payments

OrbitStream uses Stellar-native features: SEP protocols (fiat on/off ramps), built-in DEX (multi-asset), muxed accounts (payment matching), and Claimable Balances (conditional transfers).
