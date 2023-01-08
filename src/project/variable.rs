use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub name: String,
    pub id: Id,
    pub visible: bool,
    pub value: Value,
    pub variable_type: VariableType,
    pub is_cloud: bool,
    pub is_real_time: bool,
    pub object: Option<Id>,
    pub x: f32,
    pub y: f32,
    pub max_value: Option<Value>,
    pub min_value: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    Variable,
    Timer,
    Answer,
    List,
    Slide,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    Number(f32),
    String(String),
}
