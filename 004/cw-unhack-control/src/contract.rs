use std::sync::Arc;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, DAO_TO_MEMBERSHIP};

const _CONTRACT_NAME: &str = "crates.io:cw-microworld-state";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterMembershipContract {
            dao_addr,
            membership_contract_addr,
        } => execute_register_membership_contract(
            deps,
            env,
            info,
            dao_addr,
            membership_contract_addr,
        ),
    }
}

pub fn execute_register_membership_contract(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    dao_addr: String,
    membership_contract_addr: String,
) -> Result<Response, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if admin != _info.sender {
        return Err(ContractError::UnauthorizedOnlyAdminMayRegister {});
    }

    let membership_contract_addr = deps.api.addr_validate(&membership_contract_addr)?;
    let dao_addr = deps.api.addr_validate(&dao_addr)?;
    DAO_TO_MEMBERSHIP.save(deps.storage, &dao_addr, &membership_contract_addr)?;
    return Ok(Response::default()
        .add_attribute("dao", dao_addr)
        .add_attribute("membership_contract", membership_contract_addr));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetWinningMicroworldState {} => {
            to_binary(&query_winning_microworld_state(deps, env)?)
        }
    }
}

fn query_winning_microworld_state(deps: Deps, env: Env) -> StdResult<Addr> {
    // Find the DAO with the highest membership count
    let winning_dao: StdResult<Option<(Addr, u128)>> = DAO_TO_MEMBERSHIP
        .range(deps.storage, None, None, Order::Ascending)
        .fold(Ok(None), |acc: StdResult<Option<(Addr, u128)>>, kv| {
            // Query the membership count for this DAO
            let (dao_addr, membership_contract_addr): (Addr, Addr) = kv?;
            let curr_membership_count =
                query_membership_count(deps, env.clone(), membership_contract_addr)?;

            // If the current DAO has a higher membership count than the current winner, update the winner
            if let Some((winning_dao, winning_membership_count)) = acc?.clone() {
                if curr_membership_count > winning_membership_count {
                    return Ok(Some((dao_addr.clone(), curr_membership_count)));
                } else {
                    return Ok(Some((winning_dao, winning_membership_count)));
                }
            } else {
                return Err(StdError::not_found("DAO"));
            }
        });

    // If a winning DAO was found, return its address
    match winning_dao? {
        Some((dao_addr, _)) => Ok(dao_addr),
        None => Err(StdError::generic_err("No DAOs found")),
    }
}

fn query_membership_count(deps: Deps, env: Env, membership_contract_addr: Addr) -> StdResult<u128> {
    return Ok(0);
}

#[cfg(test)]
mod tests {}
