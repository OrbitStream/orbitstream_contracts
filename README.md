# OrbitStream Contracts

Soroban smart contract(s) for OrbitStream — a token streaming / payroll platform built for Stellar/Soroban.

## Summary

This repo implements on-chain streaming payroll: employers open time-based streams to employees, deposit tokens, and employees claim accrued tokens. Streams track deposited/withdrawn amounts, per-second rates, pause/resume windows, and lifecycle events.

## Repository layout

- `src/lib.rs` — contract entrypoint and public methods
- `src/stream.rs` — `Stream` struct and `StreamStatus` enum
- `src/storage.rs` — persistence helpers and indices by employer/employee
- `src/math.rs` — claimable / accrued math and validators
- `src/events.rs` — event types and emitters
- `src/errors.rs` — contract error codes
- `tests/` — (planned) integration/unit test files

## Prerequisites

- Rust toolchain (stable)
- wasm target for Soroban: `rustup target add wasm32-unknown-unknown`
- Soroban CLI (recommended): https://soroban.stellar.org/docs/getting-started/cli

## Build

Build the contract wasm with Cargo:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Or use the Soroban CLI which wraps build+bundle:

```bash
soroban contract build
```

Output location:

- Cargo: `target/wasm32-unknown-unknown/release/` (look for the wasm file)
- Soroban: `target/wasm/` or the CLI's configured output directory

## Tests

Run unit tests:

```bash
cargo test
```

Run a quick type-check:

```bash
cargo check
```

Integration testing recommendation:

- Use the Soroban local sandbox or test harness to deploy the compiled wasm and exercise end-to-end flows (create -> accrue -> claim -> cancel).

## Deploy (example)

Using Soroban CLI:

```bash
# build first
soroban contract build

# deploy to local sandbox
soroban contract deploy --wasm target/wasm/orbitstream-contract.wasm --source <KEY>

# deploy to testnet (replace with actual network flags and key)
soroban contract deploy --wasm target/wasm/orbitstream-contract.wasm --network testnet --source <KEY>
```

Alternative (raw Stellar CLI placeholder):

```bash
# stellar contract deploy --wasm <wasm-path> --source <your-keypair> --network testnet
```

## Contract API (public functions)

This contract exposes a small set of core operations. See `src/lib.rs` for signatures and types.

- `create_stream(employer, employee, token, rate_per_second, start_time, end_time, deposit) -> stream_id`
  - Caller: `employer` (must authorize)
  - Creates a new stream and emits `StreamCreated` event.

- `get_claimable(env, stream_id) -> u128`
  - Caller: anyone
  - Returns the currently claimable amount for `stream_id`.

- `get_stream(env, stream_id) -> Stream`
  - Caller: anyone
  - Returns stream details.

Planned / TODO functions (to be implemented): `top_up`, `pause_stream`, `resume_stream`, `cancel_stream`, `update_rate`, `claim`, `claim_partial`.

## Example calls (Soroban CLI style)

Create stream (example placeholder — adjust types to your keys and addresses):

```bash
soroban contract invoke --wasm target/wasm/orbitstream-contract.wasm \
	--id create_stream --arg <employer_address> <employee_address> <token_address> 1_000_000_000000 1680000000 1682592000 100000000000
```

Get claimable:

```bash
soroban contract invoke --wasm target/wasm/orbitstream-contract.wasm --id get_claimable --arg <stream_id>
```

Note: Replace CLI invocation with the exact Soroban CLI flags or RPC library calls you use. The ABI is in `src/lib.rs`.

## Events

The contract emits typed events for lifecycle changes (see `src/events.rs`):

- `stream_created`
- `stream_top_up`
- `stream_claimed`
- `stream_paused`
- `stream_resumed`
- `stream_cancelled`
- `stream_rate_updated`

Events are useful for indexing and off-chain notifications.

## Security & Design Notes

- Claimable math uses defensive checks to avoid overflow/underflow — see `src/math.rs`.
- Streams respect `start_time`, `end_time`, and `paused_duration` in accrual calculations.
- Access control: only the employer may change stream configuration; only the employee may claim.

## Contributing

- Add tests under `tests/` for each public function and lifecycle path.
- Follow Rust formatting: `cargo fmt` and `cargo clippy` before PRs.

## License

MIT. See repository license file for details.

---

For background and design notes see `CLAUDE.md`.
