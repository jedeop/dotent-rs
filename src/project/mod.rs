use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use self::{
    function::Function, interface::Interface, message::Message, object::Object, scene::Scene,
    variable::Variable,
};
use crate::error::Result;

pub mod common;
pub mod function;
pub mod interface;
pub mod message;
pub mod object;
pub mod scene;
pub mod script;
pub mod variable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub objects: Vec<Object>,
    pub scenes: Vec<Scene>,
    pub variables: Vec<Variable>,
    pub messages: Vec<Message>,
    pub functions: Vec<Function>,
    pub speed: u32,
    pub interface: Interface,
    pub name: String,
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

    #[test]
    fn read_project_from_file2() -> Result<()> {
        println!("{:#?}", Project::from_file("tests/data/project2.json")?);
        Ok(())
    }
}
