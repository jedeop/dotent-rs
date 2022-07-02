use serde::{Deserialize, Serialize};

use self::sprite::Sprite;

use super::{common::Id, script::Script};

use crate::util::serde_as_json_string;

mod sprite;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    id: Id,
    name: String,
    #[serde(with = "serde_as_json_string")]
    script: Script,
    object_type: ObjectType,
    rotate_method: RotateMethod,
    scene: Id,
    sprite: Sprite,
    selected_picture_id: Id,
    lock: bool,
    entity: Entity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ObjectType {
    Sprite,
    TextBox,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum RotateMethod {
    Free,
    Vertical,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entity {
    x: f32,
    y: f32,
    reg_x: u32,
    reg_y: u32,
    scale_x: f32,
    scale_y: f32,
    rotation: u32,
    direction: u32,
    width: u32,
    height: u32,
    font: String,
    visible: bool,
}
