use soroban_sdk::{contracttype, Address, Env, Symbol};

#[contracttype]
pub struct EscrowCreatedEvent {
    pub escrow_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: u128,
    pub timeout_at: u64,
}

#[contracttype]
pub struct EscrowReleasedEvent {
    pub escrow_id: u64,
    pub seller: Address,
    pub amount: u128,
}

#[contracttype]
pub struct EscrowRefundedEvent {
    pub escrow_id: u64,
    pub buyer: Address,
    pub amount: u128,
}

pub fn emit_escrow_created(env: &Env, event: EscrowCreatedEvent) {
    env.events()
        .publish((Symbol::new(env, "escrow_created"),), event);
}

pub fn emit_escrow_released(env: &Env, event: EscrowReleasedEvent) {
    env.events()
        .publish((Symbol::new(env, "escrow_released"),), event);
}

pub fn emit_escrow_refunded(env: &Env, event: EscrowRefundedEvent) {
    env.events()
        .publish((Symbol::new(env, "escrow_refunded"),), event);
}
