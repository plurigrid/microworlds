use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Timestamp, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{AUCTION_IN_PROGRESS, BID_TO_BIDDERS, NUMBER_OF_PARTICIPANTS, Winner};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ExecuteStartAuction { number_of_participants } => execute_start_auction(deps, number_of_participants),
        ExecuteMsg::ExecuteBid { bid_amount } => execute_bid(deps, info, bid_amount),
        ExecuteMsg::ExecuteCloseAuction {} => execute_close_auction(deps),
    }
}

fn execute_bid(deps: DepsMut, info: MessageInfo, bid_amount: Uint128) -> Result<Response, ContractError> {
    if !AUCTION_IN_PROGRESS.may_load(deps.storage)?.unwrap_or(false) {
        return Err(ContractError::AuctionNotStarted {});
    }
    if bid_amount < Uint128::from(1u128) {
        return Err(ContractError::InvalidBidAmount {});
    }
    if BID_TO_BIDDERS.may_load(deps.storage, bid_amount.u128())?.is_some() {
        return Err(ContractError::ParticipantAlreadyBid {});
    }
    BID_TO_BIDDERS.save(deps.storage, bid_amount.u128(), &info.sender)?;
    Ok(Response::default())
}

fn execute_start_auction(deps: DepsMut, number_of_participants: Uint128) -> Result<Response, ContractError> {
    if AUCTION_IN_PROGRESS.may_load(deps.storage)?.unwrap_or(false) {
        return Err(ContractError::AuctionAlreadyStarted {});
    }
    if number_of_participants < Uint128::from(2u128) {
        return Err(ContractError::TooFewParticipants {});
    }
    NUMBER_OF_PARTICIPANTS.save(deps.storage, number_of_participants)?;
    AUCTION_IN_PROGRESS.save(deps.storage, &true)?;
    Ok(Response::default())
}
