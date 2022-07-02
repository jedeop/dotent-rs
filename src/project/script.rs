use serde::{Deserialize, Serialize};

use super::common::Id;

use crate::util::serde_bool_like;

#[derive(Serialize, Deserialize, Debug)]
pub struct Script(Vec<Vec<Block>>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    id: Id,
    x: f32,
    y: f32,
    #[serde(rename = "type")]
    block_type: String,
    params: Vec<Param>,
    statements: Script,
    movable: Option<bool>,
    #[serde(deserialize_with = "serde_bool_like::deserialize")]
    deletable: bool,
    emphasized: bool,
    read_only: Option<bool>,
    copyable: bool,
    extensions: Vec<String>, // not confirmed.
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Param {
    Block(Block),
    Number(f32),
    String(String),
    Bool(bool),
    Null,
}
