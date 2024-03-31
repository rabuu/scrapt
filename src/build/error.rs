use thiserror::Error;

use lang::lex::LexError;
use lang::parse::ParseError;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("path `{0}` couldn't be handled")]
    StrangePath(std::path::PathBuf),

    #[error("no valid file at `{0}`")]
    NoValidFileAt(std::path::PathBuf),

    #[error("lexing unsuccessful")]
    LexError(#[from] LexError),

    #[error("parsing unsuccessful")]
    ParseError(#[from] ParseError),

    #[error("problems regarding file system")]
    IoError(#[from] std::io::Error),

    #[error("failed creating the ZIP archive")]
    ZipError(#[from] zip::result::ZipError),

    #[error("couldn't parse TOML manifest")]
    TomlError(#[from] manifest_scrapt::TomlDeserializationError),
}
