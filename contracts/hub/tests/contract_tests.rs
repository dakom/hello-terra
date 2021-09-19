use std::{str::FromStr};

use contract_hub::{
    contract::instantiate, 
    contract::execute,
    contract::query,
};
use shared::{
    contracts::hub::{
        execute::{ExecuteMsg}, 
        instantiate::InstantiateMsg, 
        query::{QueryMsg, TotalDeposited}
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

    let msg = InstantiateMsg{};

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

}