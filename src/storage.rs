use crate::escrow::Escrow;
use soroban_sdk::{contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct EscrowKey {
    pub escrow_id: u64,
}

pub fn store_escrow(env: &Env, escrow: &Escrow) {
    let key = EscrowKey {
        escrow_id: escrow.id,
    };
    env.storage().persistent().set(&key, escrow);
}

pub fn get_escrow(env: &Env, escrow_id: u64) -> Option<Escrow> {
    let key = EscrowKey { escrow_id };
    env.storage().persistent().get(&key)
}

pub fn get_next_escrow_id(env: &Env) -> u64 {
    let key = Symbol::new(env, "next_id");
    match env.storage().persistent().get::<Symbol, u64>(&key) {
        Some(id) => id,
        None => {
            env.storage().persistent().set(&key, &1u64);
            1
        }
    }
}

pub fn increment_escrow_id(env: &Env) {
    let key = Symbol::new(env, "next_id");
    let current = get_next_escrow_id(env);
    env.storage().persistent().set(&key, &(current + 1));
}
