#[cfg(feature = "entry")]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Decimal, Reply, Deps, DepsMut, Env, MessageInfo, Order, QueryResponse, Response, SubMsg, wasm_instantiate};
use shared::{
    coin::CoinDenom, 
    contracts::{
        account, 
        hub::{
            execute::{ExecuteMsg, AccountInfo}, 
            instantiate::InstantiateMsg, 
            query::{QueryMsg, AvailableCoins, TotalDeposited}
        }
    }, 
    result::{CustomResult, ContractError, IntoQueryResultExt, IntoStringResultExt, IntoExecuteResultExt}
};

use crate::state::{ACCOUNT_CODE_ID, USERS, TOTALS, CONTRACTS};

const REPLY_ON_NEW_ACCOUNT_ID:u64 = 1;

#[cfg_attr(feature = "entry", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> CustomResult<Response> {
    ACCOUNT_CODE_ID.save(deps.storage, &msg.account_code_id)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> CustomResult<Response> {
    match msg {
        ExecuteMsg::GetAccountInfo => {
            match USERS.may_load(deps.storage, info.sender.as_bytes())? {
                Some(contract_addr) => {
                    AccountInfo { contract_addr }.execute_result()
                },
                None => {
                    let code_id = ACCOUNT_CODE_ID.load(deps.storage)?;


                    let msg = wasm_instantiate(
                        code_id, 
                        &account::instantiate::InstantiateMsg::new(info.sender),
                        Vec::new(), 
                        "".to_string()
                    )?;

                    Ok(Response::new().add_submessage(
                        SubMsg::reply_on_success(msg, REPLY_ON_NEW_ACCOUNT_ID)
                    ))
                }
            }
        },
        ExecuteMsg::AddDeposits(coins)=> {
            //ensure we've instantiated a contract for this user
            //and that the sender here is actually an account contract 
            let _user_addr = CONTRACTS.load(deps.storage, info.sender.as_bytes())?;

            for coin in coins.iter() {
                let key = coin.denom.as_bytes();
                let amount = Decimal::from_ratio(coin.amount, 1u128);
                let totals = TOTALS.load(deps.storage, key).unwrap_or_default();
                TOTALS.save(deps.storage, key, &(totals + amount))?;
            }
            
            Ok(Response::default())
        },
    }
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> CustomResult<Response> {
    if msg.id == REPLY_ON_NEW_ACCOUNT_ID {
        let result = msg.result.into_result().map_err(ContractError::String)?;

        //debugging
        //return Err(ContractError::String(serde_json::to_string(&result).unwrap()));

        // yeah, eww, getting it from event attributes :(
        // could add owner_addr to data but need to get contract_address anyway
        // contract_address should always just be there
        // owner_addr is custom
        let (contract_addr, owner_addr):(Addr, Addr) = result.events
            .iter()
            .find(|e| e.ty == "wasm")
            .and_then(|ev| {
                let contract_addr =  ev.attributes
                    .iter()
                    .find(|a| a.key == "contract_address")
                    .and_then(|addr| deps.api.addr_validate(&addr.value).ok());
                
                let owner_addr = ev.attributes
                    .iter()
                    .find(|a| a.key == "owner_addr")
                    .and_then(|addr| deps.api.addr_validate(&addr.value).ok());
                
                match (contract_addr, owner_addr) {
                    (Some(contract_addr), Some(owner_addr)) => Some((contract_addr, owner_addr)),
                    _ => None
                }
            })
            .ok_or(ContractError::NotFound)?;

        // Add it to our lookup and reverse-lookup 
        USERS.save(deps.storage, owner_addr.as_bytes(), &contract_addr)?;
        CONTRACTS.save(deps.storage, contract_addr.as_bytes(), &owner_addr)?;

        AccountInfo { contract_addr }.execute_result()
    } else {
        Ok(Response::default())
    }
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> CustomResult<QueryResponse> {
    match msg {
        QueryMsg::AvailableCoins => {
            // add all the available coins from the accounts on file
            let list:Vec<CoinDenom> = TOTALS.keys(deps.storage, None, None, Order::Ascending)
                .filter_map(|denom| {
                    denom.string_result().ok()
                })
                .collect();

            //return it
            AvailableCoins { list }.query_result()
                
        },
        QueryMsg::TotalDeposited(denom) => {
            let total = TOTALS.load(deps.storage, denom.as_bytes()).unwrap_or_default();

            TotalDeposited {total}.query_result()
        }
    }
}