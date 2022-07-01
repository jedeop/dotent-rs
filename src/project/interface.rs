use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    canvas_width: u32,
    menu_width: u32,
    object: Id,
}
