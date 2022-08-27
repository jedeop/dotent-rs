use serde::{Deserialize, Serialize};

use crate::project::common::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprite {
    pictures: Vec<Picture>,
    sounds: Vec<Sound>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub id: Id,
    pub dimension: Dimension,
    pub fileurl: String,
    pub filename: Option<String>,
    pub name: String,
    #[serde(default = "return_100")]
    pub scale: u32,
    pub image_type: String,
}

fn return_100() -> u32 {
    100
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sound {
    pub id: Id,
    pub duration: f32,
    pub ext: String,
    pub fileurl: String,
    pub name: String,
}
