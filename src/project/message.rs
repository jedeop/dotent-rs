use serde::{Deserialize, Serialize};

use super::common::Id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    name: String,
    id: Id,
}
