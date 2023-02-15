use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const DAO_TO_MEMBERSHIP: Map<&Addr, Addr> = Map::new("dao_to_membership");
pub const ADMIN: Item<Addr> = Item::new("admin");
