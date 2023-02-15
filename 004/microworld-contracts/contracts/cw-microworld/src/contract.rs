use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    from_slice, to_binary, Addr, Binary, Deps, DepsMut, Env, Instantiate2AddressError, MessageInfo,
    Order, QueryRequest, Response, StdError, StdResult, Storage, Uint128,
};
use cw_storage_plus::{Bound, Item, Map};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use serde::{Deserialize, Serialize};

use crate::state::{MicroworldState, ADMIN, MICROWORLD_STATE};
use crate::{msg::InstantiateMsg, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    ADMIN.save(deps.storage, &admin)?;
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
        ExecuteMsg::SetMicroworldState(state) => {
            execute_set_microworld_state(deps, env, info, state)
        }
    }
}

pub fn execute_set_microworld_state(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    state: MicroworldState,
) -> Result<Response, ContractError> {
    // Get the contract admin
    let admin: Addr = ADMIN
        .load(deps.storage)
        .map_err(|e| ContractError::AdminNotFound {})?;

    // Check that the sender is the contract admin
    if info.sender != admin {
        return Err(ContractError::Unauthorized {});
    }

    // Set the microworld state
    set_microworld_state(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMicroworldState {} => {
            let state = get_microworld_state(deps.storage)?;
            Ok(to_binary(&state)?)
        }
    }
}

fn set_microworld_state(storage: &mut dyn Storage, state: &MicroworldState) -> StdResult<()> {
    MICROWORLD_STATE.save(storage, state)?;
    Ok(())
}

fn get_microworld_state(storage: &dyn Storage) -> StdResult<MicroworldState> {
    let state = MICROWORLD_STATE
        .load(storage)
        .map_err(|_| StdError::generic_err("No microworld state found."))?;
    Ok(state)
}

#[cw_serde]
pub enum ExecuteMsg {
    SetMicroworldState(MicroworldState),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MicroworldState)]
    GetMicroworldState {},
}
