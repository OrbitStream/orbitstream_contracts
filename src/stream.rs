use soroban_sdk::{contracttype, Address, Env, Symbol};

#[derive(Clone, Copy, PartialEq, Eq)]
#[contracttype]
pub enum StreamStatus {
    Active = 0,
    Paused = 1,
    Cancelled = 2,
    Completed = 3,
}

#[derive(Clone)]
#[contracttype]
pub struct Stream {
    pub id: u64,
    pub employer: Address,
    pub employee: Address,
    pub token: Address,
    pub rate_per_second: u128,
    pub deposited: u128,
    pub withdrawn: u128,
    pub start_time: u64,
    pub end_time: u64,
    pub pause_time: u64,
    pub paused_duration: u64,
    pub status: StreamStatus,
}

impl Stream {
    pub fn new(
        id: u64,
        employer: Address,
        employee: Address,
        token: Address,
        rate_per_second: u128,
        deposited: u128,
        start_time: u64,
        end_time: u64,
    ) -> Self {
        Stream {
            id,
            employer,
            employee,
            token,
            rate_per_second,
            deposited,
            withdrawn: 0,
            start_time,
            end_time,
            pause_time: 0,
            paused_duration: 0,
            status: StreamStatus::Active,
        }
    }
}
