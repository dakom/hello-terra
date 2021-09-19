use std::collections::HashSet;
use cosmwasm_std::{Decimal, Deps, DepsMut, Env, MessageInfo, Order, QueryResponse, Response, StdResult, WasmMsg, to_binary};
use shared::{
    contracts::{
        hub,
        account::{
            execute::ExecuteMsg, 
            instantiate::InstantiateMsg, 
            query::{AccountSummary, AvailableCoins, QueryMsg}
        },
    },
    coin::CoinDenom,
    result::{ContractResult, ContractError},
    utils::{IntoQueryResultExt, IntoStringResultExt}
};
use crate::{
    state::{ACCOUNTS, OWNER, HUB}, 
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    HUB.save(deps.storage, &info.sender)?;
    OWNER.save(deps.storage, &msg.owner_addr)?;

    Ok(Response::default())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
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

            let hub_msg = hub::execute::ExecuteMsg::AddDeposits(info.funds);

            Ok(Response::new().add_message(
                WasmMsg::Execute {
                    contract_addr: hub.into_string(),
                    msg: to_binary(&hub_msg)?,
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

                Ok(Response::default())
            }
        },
    }
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::AvailableCoins => {
            //temp hash set to make collecting mixture easier
            let mut all_coins:HashSet<CoinDenom> = HashSet::new();

            //first get all the available coins in the owner's wallet
            let owner = OWNER.load(deps.storage)?;
            let owner_balances = deps.querier.query_all_balances(owner)?;
            for coin in owner_balances {
                all_coins.insert(coin.denom);
            }

            // next add all the available coins from the accounts on file
            let account_coins:Vec<CoinDenom> = ACCOUNTS.keys(deps.storage, None, None, Order::Ascending)
                .filter_map(|denom| {
                    denom.string_result().ok()
                })
                .collect();

            for coin_denom in account_coins {
                all_coins.insert(coin_denom);
            }

            //collect the set into a vec
            let list:Vec<CoinDenom> = all_coins.into_iter().collect();

            //return it
            AvailableCoins { list }.query_result()
                
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
        /*
        QueryMsg::GetAccountSummary => {

            let accounts:Vec<(String, Account)> = ACCOUNTS.range(deps.storage, None, None, Order::Ascending)
                .filter_map(|res| {
                    res.ok().map(|(denom, account)| {
                        let denom = denom.string_result()?;
                        Ok((denom, account))
                    })
                })
                .collect::<Result<Vec<(String, Account)>, StdError>>()?;

            let balances = deps.querier.query_all_balances(deps.)?;

            AccountSummary { accounts }.query_result()
        },
        */
    }
}