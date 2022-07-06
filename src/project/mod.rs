use std::{fs, path::Path};

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

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn from_file<P>(path: P) -> Result<Project>
    where
        P: AsRef<Path>,
    {
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
        println!("{:#?}", Project::from_file("tests/data/project.json")?);
        Ok(())
    }
}
