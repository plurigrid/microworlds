use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[cw_serde]
pub struct MicroworldState {
    pub brightness: Uint128,
    pub color: String,
}

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const MICROWORLD_STATE: Item<MicroworldState> = Item::new("microworld_state");
