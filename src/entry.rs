use std::{collections::HashMap, fs::File, io::Read, path::{Path, PathBuf}};

use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use regex::Regex;
use tar::Archive;

use crate::{
    asset::{Asset, AssetData},
    error::{Error, Result},
    project::Project,
};

pub struct Entry {
    project: Project,
    assets: HashMap<String, Asset>,
}

impl Entry {

    /// read entry project from file
    pub fn read_file(path: &str) -> Result<Entry> {
        let file = File::open(path)?;
        Entry::read(file)
    }

    /// read entry project
    pub fn read<R>(r: R) -> Result<Entry>
    where
        R: Read,
    {
        let tar = GzDecoder::new(r);
        let mut archive = Archive::new(tar);

        let mut project: Option<Project> = None;
        let mut assets = HashMap::<String, Asset>::new();

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            let path_str = path.to_str().ok_or(Error::PathNotUTF8)?;
            if path_str == "temp/project.json" {
                let mut buf = Vec::new();
                entry.read_to_end(&mut buf)?;
                project = Some(Project::from_slice(&buf)?);
            } else {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r"^temp/\w{2}/\w{2}/(image|sound)/\w{32}.\w{3,4}$").unwrap();
                }
                if let Some(caps) = RE.captures(path_str) {
                    let asset_type = caps[1].to_string();
                    let name = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .ok_or(Error::PathNotUTF8)?
                        .to_string();
                    let ext = path
                        .extension()
                        .unwrap()
                        .to_str()
                        .ok_or(Error::PathNotUTF8)?
                        .to_string();
                    let mut data = Vec::new();
                    entry.read_to_end(&mut data)?;
                    let asset_data = AssetData {
                        data,
                        name: name.clone(),
                        ext,
                    };
                    let asset = match &asset_type[..] {
                        "image" => Asset::Image(asset_data),
                        "sound" => Asset::Sound(asset_data),
                        _ => unreachable!(),
                    };
                    assets.insert(name, asset);
                };
            }
        }

        let project = match project {
            Some(project) => project,
            None => return Err(Error::NoProjectData),
        };

        Ok(Entry { project, assets })
    }

    /// unpack entry file at specified path
    pub fn unpack<P>(path: &str, unpack_path: P) -> Result<()>
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        let file = File::open(path)?;

        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(&unpack_path)?;

        Ok(())
    }

    pub fn project(&self) -> &Project {
        &self.project
    }
    pub fn project_mut(&mut self) -> &mut Project {
        &mut self.project
    }

    pub fn assets(&self) -> &HashMap<String, Asset> {
        &self.assets
    }
}
