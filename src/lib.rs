#![no_std]

pub mod errors;
pub mod stream;
pub mod storage;
pub mod math;
pub mod events;

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};
use errors::ContractError;
use stream::{Stream, StreamStatus};
use storage::{
    store_stream, get_stream, get_next_stream_id, increment_stream_id,
    add_stream_to_employer, add_stream_to_employee,
    get_streams_by_employer, get_streams_by_employee,
};
use math::{
    calculate_claimable, validate_stream_time_range,
    validate_amount, validate_rate,
};
use events::*;

#[contract]
pub struct OrbitStreamContract;

#[contractimpl]
impl OrbitStreamContract {
    /// Create a new stream
    pub fn create_stream(
        env: Env,
        employer: Address,
        employee: Address,
        token: Address,
        rate_per_second: u128,
        start_time: u64,
        end_time: u64,
        deposit: u128,
    ) -> Result<u64, ContractError> {
        employer.require_auth();
        
        validate_stream_time_range(start_time, end_time)?;
        validate_amount(deposit)?;
        validate_rate(rate_per_second)?;
        
        let stream_id = get_next_stream_id(&env);
        increment_stream_id(&env);
        
        let stream = Stream::new(
            stream_id,
            employer.clone(),
            employee.clone(),
            token.clone(),
            rate_per_second,
            deposit,
            start_time,
            end_time,
        );
        
        store_stream(&env, &stream);
        add_stream_to_employer(&env, &employer, stream_id);
        add_stream_to_employee(&env, &employee, stream_id);
        
        emit_stream_created(&env, StreamCreatedEvent {
            stream_id,
            employer,
            employee,
            token,
            rate_per_second,
            deposited: deposit,
            start_time,
            end_time,
        });
        
        Ok(stream_id)
    }
    
    /// Get claimable amount for a stream
    pub fn get_claimable(env: Env, stream_id: u64) -> Result<u128, ContractError> {
        let stream = get_stream(&env, stream_id)
            .ok_or(ContractError::StreamNotFound)?;
        
        calculate_claimable(&env, &stream)
    }
    
    /// Get stream details
    pub fn get_stream(env: Env, stream_id: u64) -> Result<Stream, ContractError> {
        get_stream(&env, stream_id)
            .ok_or(ContractError::StreamNotFound)
    }
}
