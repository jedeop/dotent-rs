use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize)]
pub struct Script(Vec<Vec<Block>>);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    id: Id,
    x: i32,
    y: i32,
    #[serde(rename = "type")]
    block_type: String,
    params: Vec<Param>,
    statements: Script,
    movable: Option<bool>, // not confirmed.
    deletable: u8, // not confirmed.
    emphasized: bool,
    read_only: Option<bool>, // not confirmed.
    copyable: bool,
    extensions: Vec<String>, // not confirmed.
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Param {
    Block(Block),
    Number(f32),
    String(String),
    Bool(bool),
    Null,
}
