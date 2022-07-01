use std::fs;

use serde::{Deserialize, Serialize};

use self::{
    function::Function, interface::Interface, message::Message, object::Object, scene::Scene,
    variable::Variable,
};
use crate::error::Result;

mod common;
mod function;
mod interface;
mod message;
mod object;
mod scene;
mod script;
mod variable;

#[derive(Serialize, Deserialize)]
pub struct Project {
    objects: Vec<Object>,
    scenes: Vec<Scene>,
    variables: Vec<Variable>,
    messages: Vec<Message>,
    functions: Vec<Function>,
    speed: u32,
    interface: Interface,
}

impl Project {
    pub fn from_str(s: &str) -> Result<Project> {
        Ok(serde_json::from_str(s)?)
    }
    pub fn from_slice(v: &[u8]) -> Result<Project> {
        Ok(serde_json::from_slice(v)?)
    }
    pub fn from_file(path: &str) -> Result<Project> {
        let data = fs::read(path)?;
        Project::from_slice(&data)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;

    use super::Project;

    #[test]
    fn read_project_from_file() -> Result<()> {
        Project::from_file("tests/data/project.json")?;
        Ok(())
    }
}
