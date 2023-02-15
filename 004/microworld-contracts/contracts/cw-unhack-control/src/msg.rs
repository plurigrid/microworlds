use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_microworld::state::MicroworldState;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterMembershipContract {
        microworld_addr: String,
        membership_contract_addr: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MicroworldState)]
    GetWinningMicroworldState {},
}
