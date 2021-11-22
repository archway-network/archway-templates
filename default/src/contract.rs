#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        _ => try_execute(deps),
    }
}

pub fn try_execute(_deps: DepsMut) -> Result<Response, ContractError> {
    Err(ContractError::Std(StdError::generic_err("Not implemented")))
    // TODO: Ok(Response::new().add_attribute("method", "try_execute"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        _ => to_binary(&hello_world(deps)?),
    }
}

fn hello_world(_deps: Deps) -> StdResult<String> {
    Ok(String::from("Hello, Archway!"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn can_instantiate() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate_contract(deps.as_mut());
        assert_eq!(0, res.messages.len());

        let owner = &res
            .attributes
            .iter()
            .find(|a| a.key == "owner")
            .unwrap()
            .value;
        assert_eq!("creator", owner);
    }

    #[test]
    fn can_execute() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        instantiate_contract(deps.as_mut());

        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Dummy {};

        // TODO: fix this test when execute() is implemented
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Err(ContractError::Std(StdError::GenericErr { msg })) => {
                assert_eq!("Not implemented", msg)
            }
            _ => panic!("Must return not implemented error"),
        }
    }

    #[test]
    fn can_query() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        instantiate_contract(deps.as_mut());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Hello {}).unwrap();
        let value: String = from_binary(&res).unwrap();
        assert_eq!("Hello, Archway!", value);
    }

    fn instantiate_contract(deps: DepsMut) -> Response {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "token"));
        instantiate(deps, mock_env(), info, msg).unwrap()
    }
}
