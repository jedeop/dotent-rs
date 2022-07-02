use serde::{Deserialize, Serialize};

use super::{common::Id, script::Script};

use crate::util::serde_as_json_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    id: Id,
    #[serde(with = "serde_as_json_string")]
    content: Script,
}
