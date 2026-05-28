# Stellar Checkout Contracts

## Project
Soroban smart contract for Stellar Checkout — escrow functionality for dispute-prone payments.

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
  lib.rs        - contract entry point (StellarCheckoutEscrow)
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
