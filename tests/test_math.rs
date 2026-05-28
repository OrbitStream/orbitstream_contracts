#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use orbitstream_contracts::stream::{Stream, StreamStatus};
use orbitstream_contracts::math::calculate_claimable;

fn make_stream(
    env: &Env,
    start: u64,
    end: u64,
    rate: u128,
    deposited: u128,
    withdrawn: u128,
    status: StreamStatus,
) -> Stream {
    Stream {
        id: 1,
        employer: Address::generate(env),
        employee: Address::generate(env),
        token: Address::generate(env),
        rate_per_second: rate,
        deposited,
        withdrawn,
        start_time: start,
        end_time: end,
        pause_time: 0,
        paused_duration: 0,
        status,
    }
}

#[test]
fn test_claimable_mid_stream() {
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.timestamp = 1500; // halfway through 1000-2000
    });

    let stream = make_stream(&env, 1000, 2000, 100, 100_000, 0, StreamStatus::Active);
    let claimable = calculate_claimable(&env, &stream).unwrap();

    // 500 seconds * 100 rate = 50_000
    assert_eq!(claimable, 50_000);
}

#[test]
fn test_claimable_end_of_stream() {
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.timestamp = 2000;
    });

    let stream = make_stream(&env, 1000, 2000, 100, 100_000, 0, StreamStatus::Active);
    let claimable = calculate_claimable(&env, &stream).unwrap();

    // 1000 seconds * 100 rate = 100_000
    assert_eq!(claimable, 100_000);
}

#[test]
fn test_claimable_cancelled_stream() {
    let env = Env::default();
    let stream = make_stream(&env, 1000, 2000, 100, 100_000, 0, StreamStatus::Cancelled);

    let claimable = calculate_claimable(&env, &stream).unwrap();
    assert_eq!(claimable, 0);
}

#[test]
fn test_claimable_with_withdrawn() {
    let env = Env::default();
    env.ledger().with_mut(|l| {
        l.timestamp = 1500;
    });

    let stream = make_stream(&env, 1000, 2000, 100, 100_000, 20_000, StreamStatus::Active);
    let claimable = calculate_claimable(&env, &stream).unwrap();

    // 500 * 100 = 50_000 accrued - 20_000 withdrawn = 30_000
    assert_eq!(claimable, 30_000);
}
