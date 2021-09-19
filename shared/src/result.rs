use cosmwasm_std::{Addr, StdError, Decimal};
use thiserror::Error;

/// Alias for `Result<T, ContractResult>` 
pub type ContractResult<T> = Result<T, ContractError>;

/// Contract Error types
#[derive(Error, Debug)]
pub enum ContractError {
    /// Any StdError
    #[error("{0}")]
    Std(#[from] StdError),

    /// Not enough funds
    #[error("Insufficient funds (balance {balance}, required={required})")]
    InsufficientFunds { balance: Decimal, required: Decimal },


    /// Not enough funds
    #[error("Unauthorized user {addr}")]
    NoAuth { addr: Addr},
}