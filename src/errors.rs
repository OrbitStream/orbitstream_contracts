use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    StreamNotFound = 1,
    Unauthorized = 2,
    InsufficientBalance = 3,
    InvalidStream = 4,
    OverflowError = 5,
    UnderflowError = 6,
    InvalidTimeRange = 7,
    StreamAlreadyPaused = 8,
    StreamNotPaused = 9,
    StreamCancelled = 10,
    InvalidAmount = 11,
    InvalidRate = 12,
}
