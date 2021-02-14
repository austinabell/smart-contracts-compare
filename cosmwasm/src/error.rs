use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid funds (sent: {sent:}) (required: {required:})")]
    InsufficientFunds { sent: Uint128, required: Uint128 },

    #[error("Invalid coins sent for purchase")]
    InvalidCoins {},
}
