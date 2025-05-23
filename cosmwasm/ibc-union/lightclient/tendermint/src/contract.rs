use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc_union_light_client::{
    msg::{InitMsg, QueryMsg},
    IbcClientError,
};
use unionlabs_cosmwasm_upgradable::UpgradeMsg;

use crate::client::TendermintLightClient;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    ibc_union_light_client::query::<TendermintLightClient>(deps, env, msg).map_err(Into::into)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MigrateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: UpgradeMsg<InitMsg, MigrateMsg>,
) -> Result<Response, IbcClientError<TendermintLightClient>> {
    msg.run(
        deps,
        |deps, init_msg| {
            let res = ibc_union_light_client::init(deps, init_msg)?;

            Ok((res, None))
        },
        |_deps, _migrate_msg, _current_version| Ok((Response::default(), None)),
    )
}
