#[cfg(feature = "entry")]
use cosmwasm_std::entry_point;

use std::collections::HashSet;
use cosmwasm_std::{BankMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Order, QueryResponse, Reply, Response, StdResult, WasmMsg, to_binary};
use shared::{
    contracts::{
        hub,
        account::{
            execute::ExecuteMsg, 
            instantiate::InstantiateMsg, 
            query::{AccountSummary, AvailableCoinsInWallet, QueryMsg}
        },
    },
    coin::CoinDenom,
    result::{CustomResult, ContractError, IntoQueryResultExt, IntoStringResultExt}
};
use crate::{
    state::{ACCOUNTS, OWNER, HUB}, 
};

#[cfg_attr(feature = "entry", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> CustomResult<Response> {
    HUB.save(deps.storage, &info.sender)?;
    OWNER.save(deps.storage, &msg.owner_addr)?;

    Ok(Response::new().add_attribute("owner_addr", msg.owner_addr.as_str()))
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> CustomResult<Response> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::NoAuth{ addr: info.sender});
    }


    match &msg {
        ExecuteMsg::Deposit => {
            for coin in info.funds.iter() {
                let key = coin.denom.as_bytes();
                let amount = Decimal::from_ratio(coin.amount, 1u128);
                let mut account = ACCOUNTS.load(deps.storage, key).unwrap_or_default();
                account.deposit(amount);
                ACCOUNTS.save(deps.storage, key, &account)?;
            }

            let hub = HUB.load(deps.storage)?;

            Ok(Response::new().add_message(
                WasmMsg::Execute {
                    contract_addr: hub.into_string(),
                    msg: to_binary(&hub::execute::ExecuteMsg::AddDeposits(info.funds))?,
                    funds: Vec::new()
                }
            ))

        }
       
        ExecuteMsg::Withdraw(coin) => {
            let key = coin.denom.as_bytes();
            let amount = Decimal::from_ratio(coin.amount, 1u128);

            let mut account = ACCOUNTS.load(deps.storage, key).unwrap_or_default();

            if !account.withdraw(amount) {
                Err(ContractError::InsufficientFunds { balance: account.balance, required: amount })
            } else {
                ACCOUNTS.save(deps.storage, key, &account)?;

                Ok(Response::new()
                    .add_message(BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: vec![coin.clone()]
                    })
                )
            }
        },
    }
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> CustomResult<QueryResponse> {
    match msg {
        QueryMsg::AvailableCoinsInWallet => {
            //temp hash set to make sure there's no duplicates 
            let mut all_coins:HashSet<CoinDenom> = HashSet::new();

            //first get all the available coins in the owner's wallet
            let owner = OWNER.load(deps.storage)?;
            let owner_balances = deps.querier.query_all_balances(owner)?;
            for coin in owner_balances {
                all_coins.insert(coin.denom);
            }

            //collect the set into a vec
            let list:Vec<CoinDenom> = all_coins.into_iter().collect();

            //return it
            AvailableCoinsInWallet { list }.query_result()
                
        },

        QueryMsg::AccountSummary(denom) => {
            let account = ACCOUNTS.load(deps.storage, denom.as_bytes()).unwrap_or_default();
            let owner = OWNER.load(deps.storage)?;

            let wallet_balance = deps.querier.query_balance(owner, denom)?;

            AccountSummary {
                total_deposits: account.total_deposits,
                account_balance: account.balance,
                wallet_balance: Decimal::from_ratio(wallet_balance.amount, 1u128)
            }.query_result()
            
        }
    }
}

#[cfg_attr(feature = "entry", entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> CustomResult<Response> {
    Ok(Response::default())
}