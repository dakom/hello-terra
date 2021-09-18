use cosmwasm_std::{StdResult, Response, StdError, QueryResponse, to_binary};
use serde::Serialize;

//Common trait for keeping the different API response types aligned
//This also allows us to send data without having to populate all the
//separate attribute/keys
pub trait IntoResultExt {
    fn execute_result(&self) -> StdResult<Response>;
    fn query_result(&self) -> StdResult<QueryResponse>;
}

impl <T: Serialize> IntoResultExt for T {
    // Can't use native binary format yet : https://github.com/terra-money/terra.js/issues/133
    // So use a custom wrapper and impl IntoResultExt for it
    // Frontend needs to deserialize
    fn execute_result(&self) -> StdResult<Response> {
        let payload = bincode::serialize(self)
            .map_err(|_| StdError::serialize_err("payload", "bincode fail"))?;
        
        let data = base64::encode(payload);

        Ok(Response::new().add_attribute("data", data))
    }

    // QueryResponse supports native binary
    fn query_result(&self) -> StdResult<QueryResponse> {
        to_binary(self)
    }
}