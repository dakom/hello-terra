use std::{str::FromStr};

use contract_account::{
    contract::instantiate, 
    contract::execute,
    contract::query,
    state::OWNER
};
use shared::{
    contracts::account::{
        execute::{ExecuteMsg}, 
        instantiate::InstantiateMsg, 
        query::{QueryMsg, AvailableCoins, AccountSummary}
    },
    result::ContractError 
};
use cosmwasm_std::{
    Coin, Addr, Decimal, StdError, from_binary, 
    testing::{mock_dependencies, mock_env, mock_info}
};

#[test]
fn can_instantiate() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("hub", &[]);

    let msg = InstantiateMsg::new(Addr::unchecked("creator"));

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let owner = OWNER.load(&deps.storage).unwrap();

    assert_eq!(owner, Addr::unchecked("creator"));
}

#[test]
fn can_query_coins_empty() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("creator", &[]);

    let msg = InstantiateMsg::new(Addr::unchecked("creator"));

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = QueryMsg::AvailableCoins;

    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    let AvailableCoins { list } = from_binary(&res).unwrap();

    assert_eq!(list.len(), 0);
}

#[test]
fn can_query_summary_empty() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("creator", &[]);

    let msg = InstantiateMsg::new(Addr::unchecked("creator"));

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = QueryMsg::AccountSummary("ust".to_string());

    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    let summary:AccountSummary = from_binary(&res).unwrap();

    assert_eq!(summary.total_deposits, Decimal::zero());
}


#[test]
fn can_execute_all() {
    let mut deps = mock_dependencies(&[]);
    //first instantiate
    {
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg::new(Addr::unchecked("creator"));

        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    //execute deposit
    {
        let info = mock_info("creator", &[Coin::new(90, "ust")]);

        let msg = ExecuteMsg::Deposit;

        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    //query
    {
        let msg = QueryMsg::AccountSummary("ust".to_string());
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let summary:AccountSummary = from_binary(&res).unwrap();

        assert_eq!(summary.total_deposits, Decimal::from_str("90").unwrap());
        assert_eq!(summary.account_balance, Decimal::from_str("90").unwrap());
    }

    //execute withdrawal
    {
        let info = mock_info("creator", &[]);

        let msg = ExecuteMsg::Withdraw(Coin::new(50u128, "ust".to_string()));

        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    //query
    {
        let msg = QueryMsg::AccountSummary("ust".to_string());
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let summary:AccountSummary = from_binary(&res).unwrap();

        assert_eq!(summary.total_deposits, Decimal::from_str("90").unwrap());
        assert_eq!(summary.account_balance, Decimal::from_str("40").unwrap());
    }

    //execute withdrawal - not enough funds
    {
        let info = mock_info("creator", &[]);

        let msg = ExecuteMsg::Withdraw(Coin::new(50u128, "ust".to_string()));

        match execute(deps.as_mut(), mock_env(), info, msg) {
            Err(err) => {
                if !matches!(err, ContractError::InsufficientFunds { .. }) {
                    panic!("wrong error type!");
                }
            },
            Ok(_) => {
                panic!("should not be ok!");
            }
        }

    }
}