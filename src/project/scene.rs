use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    id: Id,
    name: String,
}
