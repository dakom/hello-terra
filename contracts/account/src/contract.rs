use cosmwasm_std::{Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult, Uint128};

use shared::{execute::{AccountSummary, ExecuteMsg}, instantiate::InstantiateMsg};
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
    match msg {
        ExecuteMsg::GetAccountSummary => {
            let data = AccountSummary {
                name: "bob".to_string(),
                addr: Addr::unchecked("home"),
                total_history: Decimal::one()
            };

            let payload = bincode::serialize(&data)
                .map_err(|_| StdError::serialize_err("payload", "bincode fail"))?;
           
            //Waiting on https://github.com/terra-money/terra.js/issues/133
            //Ok(Response::new().set_data(Binary(payload))

            let data = base64::encode(payload);

            Ok(Response::new().add_attribute("data", data))

        },
        _ => Ok(Response::default())
    }
}

pub fn query(
    deps: Deps,
    env: Env,
    msg: (),
) -> StdResult<QueryResponse> {
    Ok(QueryResponse::default())
}