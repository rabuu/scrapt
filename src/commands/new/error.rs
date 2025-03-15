use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum NewCmdError {
    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("Path {0:?} couldn't be handled")]
    StrangePath(std::path::PathBuf),
}
