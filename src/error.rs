use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong while (de)serializing")]
    Serde(#[from] serde_json::Error),

    #[error("I/O error")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
