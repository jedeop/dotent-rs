use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub id: Id,
    pub name: String,
}
