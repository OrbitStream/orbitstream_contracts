use soroban_sdk::{contracttype, Address, Env, Symbol, Val};

#[contracttype]
pub struct StreamCreatedEvent {
    pub stream_id: u64,
    pub employer: Address,
    pub employee: Address,
    pub token: Address,
    pub rate_per_second: u128,
    pub deposited: u128,
    pub start_time: u64,
    pub end_time: u64,
}

#[contracttype]
pub struct StreamTopUpEvent {
    pub stream_id: u64,
    pub amount: u128,
}

#[contracttype]
pub struct StreamClaimedEvent {
    pub stream_id: u64,
    pub employee: Address,
    pub amount: u128,
}

#[contracttype]
pub struct StreamPausedEvent {
    pub stream_id: u64,
}

#[contracttype]
pub struct StreamResumedEvent {
    pub stream_id: u64,
}

#[contracttype]
pub struct StreamCancelledEvent {
    pub stream_id: u64,
}

#[contracttype]
pub struct StreamRateUpdatedEvent {
    pub stream_id: u64,
    pub new_rate: u128,
}

pub fn emit_stream_created(env: &Env, event: StreamCreatedEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_created"),), event);
}

pub fn emit_stream_top_up(env: &Env, event: StreamTopUpEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_top_up"),), event);
}

pub fn emit_stream_claimed(env: &Env, event: StreamClaimedEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_claimed"),), event);
}

pub fn emit_stream_paused(env: &Env, event: StreamPausedEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_paused"),), event);
}

pub fn emit_stream_resumed(env: &Env, event: StreamResumedEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_resumed"),), event);
}

pub fn emit_stream_cancelled(env: &Env, event: StreamCancelledEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_cancelled"),), event);
}

pub fn emit_stream_rate_updated(env: &Env, event: StreamRateUpdatedEvent) {
    env.events()
        .publish((Symbol::new(env, "stream_rate_updated"),), event);
}
