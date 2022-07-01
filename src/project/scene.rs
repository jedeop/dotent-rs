use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    id: Id,
    name: String,
}
