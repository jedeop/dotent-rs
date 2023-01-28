use serde::{Deserialize, Serialize};

use self::sprite::Sprite;

use super::{common::Id, script::Script};

use crate::util::serde_as_json_string;

mod sprite;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub id: Id,
    pub name: String,
    #[serde(with = "serde_as_json_string")]
    pub script: Script,
    pub object_type: ObjectType,
    pub rotate_method: RotateMethod,
    pub scene: Id,
    pub sprite: Sprite,
    pub selected_picture_id: Id,
    pub lock: bool,
    pub entity: Entity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ObjectType {
    Sprite,
    TextBox,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RotateMethod {
    Free,
    Vertical,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub reg_x: f32,
    pub reg_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub direction: f32,
    pub width: f32,
    pub height: f32,
    pub font: String,
    pub visible: bool,
}
