use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Coin, HumanAddr, Storage};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";
pub static ROUTE_KEY: &[u8] = b"routes";

/// Stores config for the contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
}

pub(crate) fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub(crate) fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

/// Stores single record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContentRecord {
    pub price: Coin,
    pub content: String,
    pub owner: HumanAddr,
}

pub(crate) fn resolver(storage: &mut dyn Storage) -> Bucket<ContentRecord> {
    bucket(storage, ROUTE_KEY)
}

pub(crate) fn resolver_read(storage: &dyn Storage) -> ReadonlyBucket<ContentRecord> {
    bucket_read(storage, ROUTE_KEY)
}
