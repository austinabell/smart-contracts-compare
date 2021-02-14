use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, HandleResponse, HumanAddr,
    InitResponse, MessageInfo, StdResult,
};

use crate::state::{config, config_read, Config};
use crate::{
    error::ContractError,
    state::{resolver, resolver_read},
};
use crate::{
    msg::{ContentResponse, HandleMsg, InitMsg, QueryMsg},
    state::ContentRecord,
};

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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    route: String,
    content: String,
) -> Result<HandleResponse, ContractError> {
    let resolved = resolver_read(deps.storage).may_load(route.as_bytes())?;

    let (new_price, messages) = if let Some(existing) = resolved {
        // Route is taken, check if sent funds is greater before replacing
        let sent = info
            .sent_funds
            .into_iter()
            .find(|coin| coin.denom == existing.price.denom)
            .ok_or(ContractError::InvalidCoins {})?;

        if sent.amount <= existing.price.amount {
            return Err(ContractError::InsufficientFunds {
                sent: sent.amount,
                required: existing.price.amount,
            });
        }

        // Refund existing owner original price paid.
        let messages = send_tokens(&env.contract.address, &existing.owner, vec![existing.price])?;

        (sent, messages)
    } else {
        let mut sent = info.sent_funds;
        // No existing entry, continue with purchase
        (sent.pop().ok_or(ContractError::InvalidCoins {})?, vec![])
    };

    resolver(deps.storage).save(
        route.as_bytes(),
        &ContentRecord {
            content,
            price: new_price,
            owner: info.sender,
        },
    )?;

    Ok(HandleResponse {
        messages,
        ..Default::default()
    })
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
        .map(From::from))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{coins, from_binary};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Uint128,
    };

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

    #[test]
    fn purchase_and_replace() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InitMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = init(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Simple route purchase works
        let info = mock_info("addr1", &coins(2, "token"));
        let msg = HandleMsg::Purchase {
            route: "troute".to_string(),
            content: "tcontent".to_string(),
        };
        let _res = handle(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetRoute {
                route: "troute".to_string(),
            },
        )
        .unwrap();
        let value: Option<ContentResponse> = from_binary(&res).unwrap();
        assert_eq!(
            value.unwrap(),
            ContentResponse {
                content: "tcontent".into(),
                price: Coin {
                    denom: "token".into(),
                    amount: Uint128(2)
                }
            }
        );

        // Try replace with wrong token
        let info = mock_info("addr2", &coins(4, "other"));
        let msg = HandleMsg::Purchase {
            route: "troute".to_string(),
            content: "null".to_string(),
        };
        let res = handle(deps.as_mut(), mock_env(), info, msg);
        println!("{:?}", res);
        assert!(matches!(res, Err(ContractError::InvalidCoins {})));

        // Try to replace content with more "token"
        let info = mock_info("addr2", &coins(4, "token"));
        let msg = HandleMsg::Purchase {
            route: "troute".to_string(),
            content: "c2".to_string(),
        };
        let _res = handle(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetRoute {
                route: "troute".to_string(),
            },
        )
        .unwrap();
        let value: Option<ContentResponse> = from_binary(&res).unwrap();
        assert_eq!(
            value.unwrap(),
            ContentResponse {
                content: "c2".into(),
                price: Coin {
                    denom: "token".into(),
                    amount: Uint128(4)
                }
            }
        );
    }
}
