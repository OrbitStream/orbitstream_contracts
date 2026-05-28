#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use orbitstream_contracts::OrbitStreamContract;
use orbitstream_contracts::OrbitStreamContractClient;

fn create_contract<'a>(env: &Env) -> OrbitStreamContractClient<'a> {
    let contract_id = env.register(OrbitStreamContract, ());
    OrbitStreamContractClient::new(env, &contract_id)
}

#[test]
fn test_create_stream_basic() {
    let env = Env::default();
    env.mock_all_auths();

    let client = create_contract(&env);

    let employer = Address::generate(&env);
    let employee = Address::generate(&env);
    let token = Address::generate(&env);

    let start_time: u64 = 1000;
    let end_time: u64 = 2000;
    let rate_per_second: u128 = 100;
    let deposit: u128 = 100_000;

    let stream_id = client.create_stream(
        &employer,
        &employee,
        &token,
        &rate_per_second,
        &start_time,
        &end_time,
        &deposit,
    );

    assert_eq!(stream_id, 1);

    let stream = client.get_stream(&stream_id).unwrap();
    assert_eq!(stream.employer, employer);
    assert_eq!(stream.employee, employee);
    assert_eq!(stream.rate_per_second, rate_per_second);
    assert_eq!(stream.deposited, deposit);
}

#[test]
fn test_create_stream_invalid_time_range() {
    let env = Env::default();
    env.mock_all_auths();

    let client = create_contract(&env);

    let employer = Address::generate(&env);
    let employee = Address::generate(&env);
    let token = Address::generate(&env);

    // start_time >= end_time should fail
    let result = client.try_create_stream(
        &employer,
        &employee,
        &token,
        &100u128,
        &2000u64, // start after end
        &1000u64,
        &100_000u128,
    );

    assert!(result.is_err());
}
