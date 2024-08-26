use std::fs;
use std::path::PathBuf;

use md5::{Digest, Md5};

use super::BuildError;

#[derive(Debug, Clone)]
pub struct Asset {
    pub path: PathBuf,
    pub hash: String,
}

impl Asset {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let path = path.canonicalize()?;
        let buf = fs::read(&path)?;

        let mut md5_hasher = Md5::new();
        md5_hasher.update(&buf);
        let hash = md5_hasher.finalize();

        let hash = format!("{hash:0x}");

        Ok(Self { path, hash })
    }

    pub fn filename(&self, rename: bool) -> Result<String, BuildError> {
        let ext = self.path.extension().map(|os_str| os_str.to_str());

        Ok(if rename {
            if let Some(Some(ext)) = ext {
                format!("{}.{ext}", self.hash)
            } else {
                self.hash.clone()
            }
        } else {
            self.path
                .file_name()
                .ok_or_else(|| BuildError::StrangePath(self.path.clone()))?
                .to_str()
                .ok_or_else(|| BuildError::StrangePath(self.path.clone()))?
                .to_string()
        })
    }
}
