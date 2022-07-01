use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize)]
pub struct Message {
    name: String,
    id: Id,
}
