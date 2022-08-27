use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub canvas_width: u32,
    pub menu_width: u32,
    pub object: Id,
}
