use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, HandleResponse, HumanAddr,
    InitResponse, MessageInfo, StdResult,
};

use crate::msg::{ContentResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, Config};
use crate::{error::ContractError, state::resolver_read};

/// Initializes new contract.
pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InitMsg,
) -> Result<InitResponse, ContractError> {
    let state = Config {
        owner: deps.api.canonical_address(&info.sender)?,
    };
    config(deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

/// Handle incoming messages.
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::Purchase { route, content } => try_purchase(deps, env, info, route, content),
        HandleMsg::Withdraw {} => try_withdraw(deps, env, info),
    }
}

fn try_purchase(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _route: String,
    _content: String,
) -> Result<HandleResponse, ContractError> {
    todo!()
}

fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<HandleResponse, ContractError> {
    let state = config_read(deps.storage).load()?;
    if deps.api.canonical_address(&info.sender)? != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let tokens = deps.querier.query_all_balances(&env.contract.address)?;

    let messages = send_tokens(&env.contract.address, &info.sender, tokens)?;

    Ok(HandleResponse {
        messages,
        ..Default::default()
    })
}

fn send_tokens(from: &HumanAddr, to: &HumanAddr, amount: Vec<Coin>) -> StdResult<Vec<CosmosMsg>> {
    if amount.is_empty() {
        Ok(vec![])
    } else {
        let msg = BankMsg::Send {
            from_address: from.into(),
            to_address: to.into(),
            amount,
        };
        Ok(vec![msg.into()])
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRoute { route } => to_binary(&query_route(deps, route)?),
    }
}

fn query_route(deps: Deps, route: String) -> StdResult<Option<ContentResponse>> {
    Ok(resolver_read(deps.storage)
        .may_load(route.as_bytes())?
        .map(|s| ContentResponse { content: s.content }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InitMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = init(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetRoute {
                route: "doesn't exist".to_string(),
            },
        )
        .unwrap();
        let value: Option<ContentResponse> = from_binary(&res).unwrap();
        assert!(value.is_none());
    }
}
