#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cwtemplate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage,CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State{ count: msg.count, owner: info.sender.clone()  };
    STATE.save(deps.storage,&state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner",info.sender)
        .add_attribute("count", msg.count.to_string()))

    // let state = State {
    //     count: msg.count,
    //     owner: info.sender.clone(),
    // };
    // set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // STATE.save(deps.storage, &state)?;
    //
    // Ok(Response::new()
    //     .add_attribute("method", "instantiate")
    //     .add_attribute("owner", info.sender)
    //     .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => {try_increment(deps)},
        ExecuteMsg::Reset { count } => {try_reset(deps, info, count)},
        }
    // unimplemented!()
    // match msg {
    //     ExecuteMsg::Increment {} => try_increment(deps),
    //     ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    // }
}

fn try_increment (deps: DepsMut) -> Result<Response,ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_,ContractError> {
        state.count +=1;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method","try_increment"))
}

fn try_reset (deps: DepsMut,info: MessageInfo, count:i32) -> Result <Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_,ContractError> {
        if  state.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    }
    )?;
    Ok(Response::new().add_attribute("method", "try_reset"))
}

// pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         state.count += 1;
//         Ok(state)
//     })?;
//
//     Ok(Response::new().add_attribute("method", "try_increment"))
// }

// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("method", "reset"))
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount { .. } => query_count(deps)
    }

    // match msg {
    //     QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    // }
}

fn query_count (deps:Deps) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;
    to_binary(&GetCountResponse{ count: state.count })
}

// fn query_count(deps: Deps) -> StdResult<GetCountResponse> {
//     let state = STATE.load(deps.storage)?;
//     Ok(GetCountResponse { count: state.count })
// }

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary};

    #[test]
    fn my_proper_initialization(){
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info =mock_info("creator_address", &coins(1000, "earth"));
        let msg = InstantiateMsg{ count: 17};
        let res = instantiate(deps.as_mut(),env.clone(),info.clone(),msg).unwrap();
        assert_eq!(res.messages.len(),0);

        let msg = QueryMsg::GetCount {};
        let res = query(deps.as_ref(),env.clone(), msg).unwrap();
        let get_count_response : GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(get_count_response.count,17);

}
   #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn my_increment () {
        let mut deps = mock_dependencies();
        let info = mock_info("creator_address", &coins(2,"token_created"));
        let msg = InstantiateMsg{ count: 17 };
        let _res = instantiate(deps.as_mut(),mock_env(), info,msg).unwrap();

        let info = mock_info("anyone", &[]);
        let msg = ExecuteMsg::Increment {};
        let res = execute(deps.as_mut(),mock_env(),info.clone(), msg).unwrap();

        let msg = QueryMsg::GetCount {};
        let res = query(deps.as_ref(), mock_env(),msg).unwrap();
        let response : GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(response.count,18);
    }


    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }


    #[test]
    fn my_reset() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator_address", &[]);
        let res = instantiate(deps.as_mut(),mock_env(),info, InstantiateMsg{ count: 17 }).unwrap();

        let bad_info = mock_info("bad_address", &[]);
        let res = execute(deps.as_mut(),mock_env(),bad_info,ExecuteMsg::Reset { count: 25 });
        match res {
            Ok(_) => {}
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error")
        }

        let good_info = mock_info("creator_address", &[]);
        let res = execute(deps.as_mut(), mock_env(), good_info, ExecuteMsg::Reset { count: 100 }).unwrap();
        let res = query(deps.as_ref(),mock_env(),QueryMsg::GetCount {}).unwrap();
        let count_response : GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(count_response.count,100);

    }
       #[test]
        fn reset() {
            let mut deps = mock_dependencies();

            let msg = InstantiateMsg { count: 17 };
            let info = mock_info("creator", &coins(2, "token"));
            let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

            // beneficiary can release it
            let unauth_info = mock_info("anyone", &coins(2, "token"));
            let msg = ExecuteMsg::Reset { count: 5 };

            //let res = execute(deps.as_mut(), mock_env(), unauth_info, msg).unwrap_err();

            let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
            match res {
                Err(ContractError::Unauthorized {}) => {}
                _ => panic!("Must return unauthorized error"),
            }

            // only the original creator can reset the counter
            let auth_info = mock_info("creator", &coins(2, "token"));
            let msg = ExecuteMsg::Reset { count: 5 };
            let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

            // should now be 5
            let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
            let value: GetCountResponse = from_binary(&res).unwrap();
            assert_eq!(5, value.count);
        }
}
