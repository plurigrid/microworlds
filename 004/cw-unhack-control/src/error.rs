use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized; only admin may add a membership contract")]
    UnauthorizedOnlyAdminMayRegister {},

    #[error("No winning microworld found")]
    NoWinningMicroworldFound {},
}
