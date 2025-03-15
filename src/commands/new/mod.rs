use std::fs;
use std::io::Write;
use std::path::Path;

pub use error::NewCmdError;
mod error;

const STAGE_SCR: &str = include_str!("template/stage.scr");
const BACKDROP_SVG: &str = include_str!("template/backdrop.svg");

pub fn new(path: impl AsRef<Path>) -> Result<(), NewCmdError> {
    tracing::info!("Create new project {:?}...", path.as_ref());

    let path = path.as_ref();
    let assets_dir = path.join("assets");

    fs::create_dir(path)?;
    fs::create_dir(&assets_dir)?;

    let name = path.canonicalize()?;
    let name = name
        .file_name()
        .ok_or_else(|| NewCmdError::StrangePath(path.to_path_buf()))?
        .to_str()
        .ok_or_else(|| NewCmdError::StrangePath(path.to_path_buf()))?;

    fs::File::create_new(path.join("project.toml"))?
        .write_all(format!("[project]\nname = \"{name}\"").as_bytes())?;
    fs::File::create_new(path.join("stage.scr"))?.write_all(STAGE_SCR.as_bytes())?;
    fs::File::create_new(assets_dir.join("backdrop.svg"))?.write_all(BACKDROP_SVG.as_bytes())?;

    Ok(())
}
