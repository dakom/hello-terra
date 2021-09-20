use thiserror::Error;
use cosmwasm_std::{Addr, StdResult, Response, StdError, QueryResponse, Decimal, to_binary};
use serde::Serialize;

/// Alias for `Result<T, CustomResult>` 
pub type CustomResult<T> = Result<T, ContractError>;

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

    /// Any string error (useful for Reply) 
    #[error("{0}")]
    String (String),

    /// Not found 
    #[error("Not found")]
    NotFound,
}

//Common trait for keeping the different API response types aligned
//This also allows us to send data without having to populate all the
//separate attribute/keys
pub trait IntoExecuteResultExt {
    fn execute_result(&self) -> CustomResult<Response>;
}
pub trait IntoQueryResultExt {
    fn query_result(&self) -> CustomResult<QueryResponse>;
}

pub trait IntoStringResultExt {
    fn string_result(&self) -> CustomResult<String> {
        Ok(self.str_result()?.to_string())
    }
    fn str_result(&self) -> CustomResult<&str>;
}

impl <T: Serialize> IntoExecuteResultExt for T {
    // Can't use native binary format yet : https://github.com/terra-money/terra.js/issues/133
    // So use a custom wrapper and impl IntoResultExt for it
    // Frontend needs to deserialize
    fn execute_result(&self) -> CustomResult<Response> {
        let payload = bincode::serialize(self)
            .map_err(|_| ContractError::Std(StdError::serialize_err("payload", "bincode fail")))?;
        
        let data = base64::encode(payload);

        Ok(Response::new().add_attribute("data", data))
    }
}

impl <T: Serialize> IntoQueryResultExt for T {
    // QueryResponse supports native binary
    fn query_result(&self) -> CustomResult<QueryResponse> {
        to_binary(self).map_err(|err| ContractError::Std(err))
    }
}

impl <T: AsRef<[u8]>> IntoStringResultExt for T {
    fn str_result(&self) -> CustomResult<&str> {
        std::str::from_utf8(self.as_ref())
            .map_err(|_| ContractError::Std(StdError::invalid_utf8("byte slice to string")))
    }
}