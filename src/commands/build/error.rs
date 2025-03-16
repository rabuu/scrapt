use miette::Diagnostic;
use thiserror::Error;

use scrapt::manifest;
use scrapt::parsing::ParsingError;

#[derive(Debug, Error, Diagnostic)]
pub enum BuildCmdError {
    #[error("Path `{0}` couldn't be handled")]
    StrangePath(std::path::PathBuf),

    #[error("No valid file at `{0}`")]
    NoValidFileAt(std::path::PathBuf),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Failed creating the ZIP archive")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Couldn't parse TOML manifest")]
    TomlError(#[from] manifest::TomlDeserializationError),

    #[error("Parsing failed")]
    ParsingError(#[related] Vec<ParsingError>, #[source_code] String),
}
