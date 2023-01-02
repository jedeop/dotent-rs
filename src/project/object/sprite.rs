use serde::{Deserialize, Serialize};

use crate::project::common::Id;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sprite {
    pub pictures: Vec<Picture>,
    pub sounds: Vec<Sound>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub id: Id,
    pub dimension: Dimension,
    pub fileurl: String,
    pub filename: Option<String>,
    pub name: String,
    #[serde(default = "return_100")]
    pub scale: f32,
    pub image_type: String,
}

fn return_100() -> f32 {
    100.0
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sound {
    pub id: Id,
    pub duration: f32,
    pub ext: String,
    pub fileurl: String,
    pub name: String,
}
