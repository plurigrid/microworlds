use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

// MICROWORLD_TO_MEMBERSHIP is a mapping of microworld contracts (1:1 with a coalition) and its membership contract.
pub const MICROWORLD_TO_MEMBERSHIP: Map<&Addr, Addr> = Map::new("microworld_to_membership");
pub const ADMIN: Item<Addr> = Item::new("admin");
