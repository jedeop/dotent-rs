use std::{
    fs::File,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::{error::Result, project::Project, util::get_new_temp_dir};

pub struct Entry {
    unpack_path: PathBuf,
    project: Option<Project>,
}

impl Entry {
    pub fn read_file(path: &str) -> Result<Entry> {
        let unpack_path = get_new_temp_dir()?;

        let file = File::open(path)?;

        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(&unpack_path)?;

        Ok(Entry {
            unpack_path,
            project: None,
        })
    }

    pub fn get_project(&mut self) -> Result<&Project> {
        Ok(self.project.get_or_insert(Project::from_file(
            Path::new(&self.unpack_path).join("temp/project.json"),
        )?))
    }
    pub fn get_project_mut(&mut self) -> Result<&mut Project> {
        Ok(self.project.get_or_insert(Project::from_file(
            Path::new(&self.unpack_path).join("temp/project.json"),
        )?))
    }

    pub fn get_unpack_dir(&self) -> &PathBuf {
        &self.unpack_path
    }
}
