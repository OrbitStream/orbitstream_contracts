use crate::stream::Stream;
use soroban_sdk::{contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct StreamKey {
    pub stream_id: u64,
}

pub fn store_stream(env: &Env, stream: &Stream) {
    let key = StreamKey {
        stream_id: stream.id,
    };
    env.storage().persistent().set(&key, stream);
}

pub fn get_stream(env: &Env, stream_id: u64) -> Option<Stream> {
    let key = StreamKey { stream_id };
    env.storage().persistent().get(&key)
}

pub fn delete_stream(env: &Env, stream_id: u64) {
    let key = StreamKey { stream_id };
    env.storage().persistent().remove(&key);
}

pub fn get_next_stream_id(env: &Env) -> u64 {
    let key = Symbol::new(env, "next_id");
    match env.storage().persistent().get::<Symbol, u64>(&key) {
        Some(id) => id,
        None => {
            env.storage().persistent().set(&key, &1u64);
            1
        }
    }
}

pub fn increment_stream_id(env: &Env) {
    let key = Symbol::new(env, "next_id");
    let current = get_next_stream_id(env);
    env.storage().persistent().set(&key, &(current + 1));
}

pub fn add_stream_to_employer(env: &Env, employer: &Address, stream_id: u64) {
    let key = (Symbol::new(env, "emp_streams"), employer.clone());
    let mut streams: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    streams.push_back(stream_id);
    env.storage().persistent().set(&key, &streams);
}

pub fn add_stream_to_employee(env: &Env, employee: &Address, stream_id: u64) {
    let key = (Symbol::new(env, "emp_receives_streams"), employee.clone());
    let mut streams: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    streams.push_back(stream_id);
    env.storage().persistent().set(&key, &streams);
}

pub fn get_streams_by_employer(env: &Env, employer: &Address) -> Vec<u64> {
    let key = (Symbol::new(env, "emp_streams"), employer.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}

pub fn get_streams_by_employee(env: &Env, employee: &Address) -> Vec<u64> {
    let key = (Symbol::new(env, "emp_receives_streams"), employee.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}
