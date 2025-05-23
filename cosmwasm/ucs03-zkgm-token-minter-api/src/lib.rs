use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};
use enumorph::Enumorph;

pub const CW20_TOKEN_CREATION_EVENT: &str = "cw20_token_creation";
pub const CW20_TOKEN_ADDRESS: &str = "cw20_token_address";
pub const CW20_QUOTE_TOKEN: &str = "quote_token";

#[cw_serde]
pub enum LocalTokenMsg {
    Escrow {
        from: String,
        denom: String,
        recipient: String,
        amount: Uint128,
    },
    Unescrow {
        denom: String,
        recipient: String,
        amount: Uint128,
    },
}

#[cw_serde]
pub struct Metadata {
    /// name defines the name of the token (eg: Cosmos Atom)
    pub name: String,
    /// symbol is the token symbol usually shown on exchanges (eg: ATOM). This can
    /// be the same as the display.
    pub symbol: String,
}

#[cw_serde]
pub enum WrappedTokenMsg {
    CreateDenom {
        subdenom: String,
        // TODO: upgrade tokenfactory to handle this
        metadata: Metadata,
        path: Binary,
        channel: u32,
        token: Binary,
    },
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    BurnTokens {
        denom: String,
        amount: Uint128,
        burn_from_address: String,
        sender: Addr,
    },
}

#[cw_serde]
#[derive(Enumorph)]
pub enum ExecuteMsg {
    Wrapped(WrappedTokenMsg),
    Local(LocalTokenMsg),
}

#[cw_serde]
pub enum QueryMsg {
    /// Query the metadata of a token.
    Metadata {
        /// `denom` is either a normal token denom, or a cosmwasm contract address of a cw20 token that was created through the `cw20-token-minter`.
        denom: String,
    },
    PredictWrappedToken {
        path: String,
        channel: u32,
        token: Binary,
    },
}

#[cw_serde]
pub struct MetadataResponse {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct PredictWrappedTokenResponse {
    pub wrapped_token: String,
}
