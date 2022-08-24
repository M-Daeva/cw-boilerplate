#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, Deps, Env, MessageInfo, StdResult};

use crate::{messages::response::CountResponse, state::STATE};

pub fn query_state(deps: Deps, _env: Env, _info: MessageInfo) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;

    to_binary(&CountResponse { count: state.count })
}
