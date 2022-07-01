use serde::{Deserialize, Serialize};

use super::{common::Id, script::Script};

#[derive(Serialize, Deserialize)]
pub struct Function {
    id: Id,
    content: Script,
}
