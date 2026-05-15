use crate::errors::ContractError;
use crate::stream::{Stream, StreamStatus};
use soroban_sdk::Env;

pub fn calculate_elapsed_time(env: &Env, stream: &Stream) -> Result<u64, ContractError> {
    let now = env.ledger().timestamp();

    if now < stream.start_time {
        return Ok(0);
    }

    let elapsed_before_pause = if now > stream.end_time {
        stream.end_time - stream.start_time
    } else {
        now - stream.start_time
    };

    let elapsed = elapsed_before_pause
        .checked_sub(stream.paused_duration)
        .ok_or(ContractError::UnderflowError)?;

    Ok(elapsed)
}

pub fn calculate_accrued(stream: &Stream, elapsed_time: u64) -> Result<u128, ContractError> {
    let elapsed_u128 = elapsed_time as u128;

    elapsed_u128
        .checked_mul(stream.rate_per_second)
        .ok_or(ContractError::OverflowError)
}

pub fn calculate_claimable(env: &Env, stream: &Stream) -> Result<u128, ContractError> {
    // Streams that are cancelled have no claimable amount
    if stream.status == StreamStatus::Cancelled {
        return Ok(0);
    }

    let elapsed = calculate_elapsed_time(env, stream)?;
    let accrued = calculate_accrued(stream, elapsed)?;

    // Claimable can't exceed what was deposited
    let max_accrued = accrued.min(stream.deposited);

    // Subtract what's already been withdrawn
    max_accrued
        .checked_sub(stream.withdrawn)
        .ok_or(ContractError::UnderflowError)
}

pub fn validate_stream_time_range(start_time: u64, end_time: u64) -> Result<(), ContractError> {
    if start_time >= end_time {
        return Err(ContractError::InvalidTimeRange);
    }
    Ok(())
}

pub fn validate_amount(amount: u128) -> Result<(), ContractError> {
    if amount == 0 {
        return Err(ContractError::InvalidAmount);
    }
    Ok(())
}

pub fn validate_rate(rate: u128) -> Result<(), ContractError> {
    if rate == 0 {
        return Err(ContractError::InvalidRate);
    }
    Ok(())
}
