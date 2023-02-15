#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
    Uint128,
};
use cw4::MemberListResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, MICROWORLD_TO_MEMBERSHIP};
use cw_microworld::state::MicroworldState;

const _CONTRACT_NAME: &str = "crates.io:cw-microworld-state";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
        ExecuteMsg::RegisterMembershipContract {
            microworld_addr: dao_addr,
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
    MICROWORLD_TO_MEMBERSHIP.save(deps.storage, &dao_addr, &membership_contract_addr)?;
    return Ok(Response::default()
        .add_attribute("dao", dao_addr)
        .add_attribute("membership_contract", membership_contract_addr));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetWinningMicroworldState {} => to_binary(&query_winning_microworld_state(deps)?),
    }
}

// Returns a struct representing the winning microworld configuration
fn query_winning_microworld_state(deps: Deps) -> StdResult<MicroworldState> {
    // Get the microworld which corresponds to the coalition with the greatest number of members
    let winning_microworld = find_winning_microworld(deps)?;

    // Query the microworld's state
    let res: MicroworldState = deps.querier.query_wasm_smart(
        winning_microworld,
        &cw_microworld::msg::QueryMsg::GetMicroworldState {},
    )?;

    return Ok(res);
}

fn find_winning_microworld(deps: Deps) -> StdResult<Addr> {
    // Find the dao with the highest membership count
    let iter = MICROWORLD_TO_MEMBERSHIP.range(deps.storage, None, None, Order::Ascending);

    let winning_microworld: StdResult<Option<(Addr, u128)>> =
        iter.fold(Ok(None), |acc: StdResult<Option<(Addr, u128)>>, curr| {
            // Query the membership count for this DAO
            let (curr_microworld_addr, curr_membership_addr): (Addr, Addr) = curr?;
            let curr_membership_count = query_membership_count(deps, curr_membership_addr)?;

            // If the current DAO has a higher membership count than the current winner, update the winner
            if let Some((winning_microworld, winning_membership_count)) = acc?.clone() {
                if curr_membership_count > winning_membership_count {
                    return Ok(Some((curr_microworld_addr.clone(), curr_membership_count)));
                } else {
                    return Ok(Some((winning_microworld, winning_membership_count)));
                }
            } else {
                // if acc is None, return curr
                return Ok(Some((curr_microworld_addr.clone(), curr_membership_count)));
            }
        });

    // If a winning DAO was found, return its address
    match winning_microworld? {
        Some((dao_addr, _)) => Ok(dao_addr),
        None => Err(StdError::generic_err("No microworlds registered")),
    }
}

// Membership contract address is expected to be a cw4 group contract which implements the following query:
// MemberList{start_after, limit}
fn query_membership_count(deps: Deps, membership_contract_addr: Addr) -> StdResult<u128> {
    let res: MemberListResponse = deps.querier.query_wasm_smart(
        membership_contract_addr.to_string(),
        &&cw4_group::msg::QueryMsg::ListMembers {
            start_after: None,
            limit: None,
        },
    )?;

    Ok(res.members.len() as u128)
}

#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use crate::state::ADMIN;
    use crate::ContractError;
    use cosmwasm_std::{Addr, Api, Empty, Env, Querier, QueryRequest, StdError, Storage, Uint128};
    use cw4::Member;
    use cw_microworld::state::MicroworldState;
    use cw_multi_test::Executor;
    use cw_multi_test::{App, Contract, ContractWrapper};
    use cw_storage_plus::Item;

    fn control_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    fn microworld_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw_microworld::contract::execute,
            cw_microworld::contract::instantiate,
            cw_microworld::contract::query,
        );
        Box::new(contract)
    }

    fn cw4_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw4_group::contract::execute,
            cw4_group::contract::instantiate,
            cw4_group::contract::query,
        );
        Box::new(contract)
    }

    #[test]
    fn test_query_winning_microworld_state() {
        let mut app = App::default();
        let control_id = app.store_code(control_contract());
        let microworld_id = app.store_code(microworld_contract());
        let cw4_id = app.store_code(cw4_contract());

        // setup admin
        let admin = Addr::unchecked("admin");
        let instantiate_msg = InstantiateMsg {
            admin: admin.to_string(),
        };

        // Instantiate control contract
        let control_addr = app
            .instantiate_contract(
                control_id,
                admin.clone(),
                &instantiate_msg,
                &[],
                "control_contract",
                None,
            )
            .unwrap();

        // Instantiate winning microworld contract
        let winning_microworld_addr: Addr = app
            .instantiate_contract(
                microworld_id,
                admin.clone(),
                &cw_microworld::msg::InstantiateMsg {
                    admin: admin.to_string(),
                },
                &[],
                "microworld_1",
                None,
            )
            .unwrap();

        // Instantiate cw4-group contracts

        // Winning contract has the most members
        let cw4_addr_winner = app
            .instantiate_contract(
                cw4_id,
                admin.clone(),
                &cw4_group::msg::InstantiateMsg {
                    admin: None,
                    members: [
                        Member {
                            addr: "1111".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "22222".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "33333".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "44444".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "55555".to_string(),
                            weight: 1,
                        },
                    ]
                    .to_vec(),
                },
                &[],
                "cw4-group",
                None,
            )
            .unwrap();

        let cw4_addr_1 = app
            .instantiate_contract(
                cw4_id,
                admin.clone(),
                &cw4_group::msg::InstantiateMsg {
                    admin: None,
                    members: [Member {
                        addr: "1111".to_string(),
                        weight: 1,
                    }]
                    .to_vec(),
                },
                &[],
                "cw4-group",
                None,
            )
            .unwrap();

        let cw4_addr_2 = app
            .instantiate_contract(
                cw4_id,
                admin.clone(),
                &cw4_group::msg::InstantiateMsg {
                    admin: None,
                    members: [
                        Member {
                            addr: "11111".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "22222".to_string(),
                            weight: 1,
                        },
                    ]
                    .to_vec(),
                },
                &[],
                "cw4-group",
                None,
            )
            .unwrap();

        let cw4_addr_3 = app
            .instantiate_contract(
                cw4_id,
                admin.clone(),
                &cw4_group::msg::InstantiateMsg {
                    admin: None,
                    members: [
                        Member {
                            addr: "11111".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "222222".to_string(),
                            weight: 1,
                        },
                        Member {
                            addr: "33333".to_string(),
                            weight: 1,
                        },
                    ]
                    .to_vec(),
                },
                &[],
                "cw4-group",
                None,
            )
            .unwrap();

        // set winning microworld microworld state
        let winning_state = MicroworldState {
            brightness: Uint128::from(100u128),
            color: "electricblu supweeeme dweem".into(),
        };

        app.execute_contract(
            admin.clone(),
            winning_microworld_addr.clone(),
            &cw_microworld::msg::ExecuteMsg::SetMicroworldState(winning_state.clone()),
            &[],
        )
        .unwrap();

        // Register memberships with control contract
        app.execute_contract(
            admin.clone(),
            control_addr.clone(),
            &crate::msg::ExecuteMsg::RegisterMembershipContract {
                microworld_addr: winning_microworld_addr.to_string(),
                membership_contract_addr: cw4_addr_winner.to_string(),
            },
            &[],
        )
        .unwrap();

        app.execute_contract(
            admin.clone(),
            control_addr.clone(),
            &crate::msg::ExecuteMsg::RegisterMembershipContract {
                microworld_addr: "addr_1".to_string(),
                membership_contract_addr: cw4_addr_1.to_string(),
            },
            &[],
        )
        .unwrap();

        app.execute_contract(
            admin.clone(),
            control_addr.clone(),
            &crate::msg::ExecuteMsg::RegisterMembershipContract {
                microworld_addr: "addr_2".to_string(),
                membership_contract_addr: cw4_addr_2.to_string(),
            },
            &[],
        )
        .unwrap();

        app.execute_contract(
            admin.clone(),
            control_addr.clone(),
            &crate::msg::ExecuteMsg::RegisterMembershipContract {
                microworld_addr: "addr_3".to_string(),
                membership_contract_addr: cw4_addr_3.to_string(),
            },
            &[],
        )
        .unwrap();

        // Query winning microworld state
        let res: MicroworldState = app
            .wrap()
            .query_wasm_smart(
                &control_addr,
                &crate::msg::QueryMsg::GetWinningMicroworldState {},
            )
            .unwrap();

        // assert that we get the correct microworld state for the winning microworld
        let expected_state = MicroworldState {
            brightness: Uint128::from(100u128),
            color: "electricblu supweeeme dweem".into(),
        };

        assert_eq!(res, expected_state);
    }

    #[test]
    fn test_query_winning_microworld_state_no_microworlds_registered() {
        let mut app = App::default();
        let control_id = app.store_code(control_contract());

        // setup admin
        let admin = Addr::unchecked("admin");
        let instantiate_msg = InstantiateMsg {
            admin: admin.to_string(),
        };

        // Instantiate control contract
        let control_addr = app
            .instantiate_contract(
                control_id,
                admin.clone(),
                &instantiate_msg,
                &[],
                "control_contract",
                None,
            )
            .unwrap();

        // No membership contracts registered, should error
        let err: StdError = app
            .wrap()
            .query_wasm_smart::<MicroworldState>(
                control_addr,
                &crate::msg::QueryMsg::GetWinningMicroworldState {},
            )
            .unwrap_err();

        assert_eq!(
            err,
            StdError::generic_err(
                "Querier contract error: Generic error: No microworlds registered"
            )
        );
    }
}
