# OrbitStream Contracts

## Project
Soroban smart contract for OrbitStream — a token streaming/payroll platform on Stellar blockchain.

## Stack
- Rust
- Soroban SDK
- Stellar Testnet / Mainnet

## What This Contract Does
Handles salary payroll streams on Stellar. Employers create streams to employees,
employees claim accrued tokens anytime (pull-based).

## Core Data Structure
- Stream: id, employer, employee, token, rate_per_second, deposited, withdrawn,
  start_time, end_time, pause_time, paused_duration, status

## Contract Functions
### Employer
- create_stream(employer, employee, token, rate_per_second, start_time, end_time, deposit)
- top_up(stream_id, amount)
- pause_stream(stream_id)
- resume_stream(stream_id)
- cancel_stream(stream_id)
- update_rate(stream_id, new_rate)

### Employee
- claim(stream_id)
- claim_partial(stream_id, amount)

### Read
- get_stream(stream_id)
- get_claimable(stream_id)
- get_streams_by_employer(employer)
- get_streams_by_employee(employee)

## Claimable Math
elapsed = min(now, end_time) - start_time - paused_duration
accrued = elapsed * rate_per_second
claimable = min(accrued, deposited) - withdrawn

## File Structure
src/
  lib.rs        - contract entry point
  stream.rs     - Stream struct + StreamStatus enum
  storage.rs    - read/write contract storage
  math.rs       - claimable calculation
  events.rs     - all event definitions
  errors.rs     - custom error types
tests/
  test_create.rs
  test_claim.rs
  test_pause_resume.rs
  test_cancel.rs

## Key Rules
- No business logic outside of contract (no user profiles, no analytics)
- All state changes must emit events
- Protect against overflow/underflow everywhere
- Only employer can pause/cancel/top_up/update_rate
- Only employee can claim
- Token is USDC on Stellar by default, but contract is multi-token
