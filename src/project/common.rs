use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Id(String);
