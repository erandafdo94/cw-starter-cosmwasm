use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>, //[("Juno", 2), ("Osmosis", 1), ("Cosmos Hub", 3)] this is like a list
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

// A map with a String key and Poll value.
// The key will be a UUID generated clientside
//string here is the unique id of the poll
pub const POLLS: Map<String, Poll> = Map::new("polls");
//This composite key will be in the format of (Addr, String).
//Where Addr is the address of the voter and String is the Poll UUID this vote is for.
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
pub const CONFIG: Item<Config> = Item::new("config");
