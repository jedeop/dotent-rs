use serde::{Deserialize, Serialize};

use super::common::Id;

use crate::util::serde_bool_like;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script(pub Vec<Vec<Block>>);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: Id,
    pub x: f32,
    pub y: f32,
    #[serde(rename = "type")]
    pub block_type: String,
    pub params: Vec<Param>,
    pub statements: Script,
    pub movable: Option<bool>,
    #[serde(deserialize_with = "serde_bool_like::deserialize")]
    pub deletable: bool,
    pub emphasized: bool,
    pub read_only: Option<bool>,
    pub copyable: bool,
    pub extensions: Vec<String>, // not confirmed.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Param {
    Block(Block),
    Number(f32),
    String(String),
    Bool(bool),
    Null,
}
