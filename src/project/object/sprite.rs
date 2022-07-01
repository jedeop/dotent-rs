use serde::{Deserialize, Serialize};

use crate::project::common::Id;

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pictures: Vec<Picture>,
    sounds: Vec<Sound>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    id: Id,
    dimension: Dimension,
    fileurl: String,
    name: String,
    scale: u32,
    image_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    width: u32,
    height: u32,
    scale_x: Option<f32>,
    scale_y: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Sound {
    id: Id,
    duration: f32,
    ext: String,
    fileurl: String,
    name: String,
}
