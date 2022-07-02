use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    name: String,
    id: Id,
    visible: bool,
    value: Value,
    variable_type: VariableType,
    is_cloud: bool,
    is_real_time: bool,
    object: Option<Id>,
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum VariableType {
    Variable,
    Timer,
    Answer,
    List,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Value {
    Number(i32),
    String(String),
}
