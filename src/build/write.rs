use std::fs;
use std::io::Write;
use std::path::Path;

use zip::write::{FileOptions, ZipWriter};

use super::BuildError;

pub fn write_to_zip(
    output_path: impl AsRef<Path>,
    assets: &[impl AsRef<Path>],
) -> Result<(), BuildError> {
    tracing::debug!("Writing ZIP file...");

    let output_file = fs::File::create(output_path)?;
    let mut zip = ZipWriter::new(output_file);
    let zip_options = FileOptions::default();

    zip.start_file("project.json", zip_options)?;
    zip.write_all(b"Hello JSON!")?;

    for asset_path in assets {
        let basename = asset_path
            .as_ref()
            .file_name()
            .ok_or(BuildError::StrangePath(asset_path.as_ref().to_path_buf()))?
            .to_str()
            .ok_or(BuildError::StrangePath(asset_path.as_ref().to_path_buf()))?;

        zip.start_file(basename, zip_options)?;

        let file = fs::read(asset_path.as_ref())?;
        zip.write_all(&file)?;
    }

    zip.finish()?;

    Ok(())
}

pub fn write_to_dir(
    output_dir: impl AsRef<Path>,
    assets: &[impl AsRef<Path>],
) -> Result<(), BuildError> {
    tracing::debug!("Writing output directory...");

    fs::create_dir_all(output_dir.as_ref())?;
    let mut manifest = fs::File::create_new(output_dir.as_ref().join("project.json"))?;
    manifest.write_all(b"Hello JSON!")?;

    for asset_path in assets {
        let basename = asset_path
            .as_ref()
            .file_name()
            .ok_or(BuildError::StrangePath(asset_path.as_ref().to_path_buf()))?
            .to_str()
            .ok_or(BuildError::StrangePath(asset_path.as_ref().to_path_buf()))?;
        fs::copy(asset_path, output_dir.as_ref().join(basename))?;
    }

    Ok(())
}
