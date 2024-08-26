use thiserror::Error;

#[derive(Debug, Error)]
pub enum NewError {
    #[error("problems regarding file system")]
    IoError(#[from] std::io::Error),

    #[error("path {0:?} couldn't be handled")]
    StrangePath(std::path::PathBuf),
}
