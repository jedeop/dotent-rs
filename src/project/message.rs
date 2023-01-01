use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub name: String,
    pub id: Id,
}
