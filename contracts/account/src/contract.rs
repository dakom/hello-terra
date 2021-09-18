use cosmwasm_std::{Addr, BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult, Uint128, to_binary};
use cosmwasm_storage::{singleton, bucket, bucket_read, Bucket};
use shared::{execute::{AccountSummary, ExecuteMsg}, state::Account, instantiate::InstantiateMsg, query::QueryMsg};
use terra_cosmwasm::TerraQuerier;
use crate::{state::{ACCOUNTS}, utils::IntoResultExt};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    // use the sender as the key into the singleton
    // effectively it prevents us from mixing up values with other users
    // even if they execute this contract
    let mut accounts = bucket(deps.storage, ACCOUNTS);

    for coin in info.funds.iter() {
        let Coin {denom, amount} = coin;

        let amount = Decimal::from_ratio(amount.clone(), 1u128);

        accounts.update::<_, StdError>(denom.as_bytes(), |entry:Option<Account>| {
            match entry {
                Some(mut account) => {
                    Ok(Account {
                        total_deposits: account.total_deposits + amount,
                        balance: account.balance + amount
                    })
                },
                None => {
                    Ok(Account {
                        total_deposits: amount,
                        balance: amount
                    })
                }
            }
        })?;
    }




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
            AccountSummary {
                name: "bob".to_string(),
                addr: Addr::unchecked("home"),
                total_history: Decimal::one()
            }.execute_result()
        },
        _ => Ok(Response::default())
    }
}

pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetAccountSummary => {

            let accounts = bucket_read::<Account>(deps.storage, ACCOUNTS);

            AccountSummary {
                name: "sally".to_string(),
                addr: Addr::unchecked("mars"),
                total_history: Decimal::one()
            }.query_result()
        },
        _ => Ok(QueryResponse::default())
    }
}