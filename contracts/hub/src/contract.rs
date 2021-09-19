use std::collections::HashSet;
use cosmwasm_std::{Decimal, Deps, DepsMut, Env, MessageInfo, Order, QueryResponse, Response, StdError, StdResult, WasmMsg, to_binary};
use shared::{
    contracts::{
        hub::{
            execute::ExecuteMsg,
            instantiate::InstantiateMsg,
            query::{QueryMsg, TotalDeposited}
        }
    },
    coin::CoinDenom,
    result::{ContractResult, ContractError},
    utils::{IntoQueryResultExt, IntoStringResultExt}
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ContractResult<Response> {
    Ok(Response::default())
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> ContractResult<Response> {
    Ok(Response::default())
}

pub fn query(
    _deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<QueryResponse> {
    Ok(QueryResponse::default())
}