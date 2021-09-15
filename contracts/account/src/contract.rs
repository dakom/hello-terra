use cosmwasm_std::{
    BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128, QueryResponse
};

use shared::{
    execute::ExecuteMsg, 
    instantiate::InstantiateMsg, 
    query::QueryMsg
};
use terra_cosmwasm::TerraQuerier;


pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    Ok(QueryResponse::default())
}