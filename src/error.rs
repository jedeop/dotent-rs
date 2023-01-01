use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong while (de)serializing")]
    Serde(#[from] serde_json::Error),

    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("No Project Data found in entry file")]
    NoProjectData,

    #[error("Path is not valid utf-8")]
    PathNotUTF8,
}

pub type Result<T> = std::result::Result<T, Error>;
