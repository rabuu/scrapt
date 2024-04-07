use std::fs;
use std::io::Write;
use std::path::Path;

use zip::write::{FileOptions, ZipWriter};

use super::{asset::Asset, BuildError};

pub fn write_to_zip(
    output_path: impl AsRef<Path>,
    scratch_project: manifest_scratch::Manifest,
    assets: &[Asset],
    rename: bool,
) -> Result<(), BuildError> {
    tracing::info!("Writing ZIP file {:?}...", output_path.as_ref());

    let output_file = fs::File::create(output_path)?;
    let mut zip = ZipWriter::new(output_file);
    let zip_options = FileOptions::default();

    zip.start_file("project.json", zip_options)?;
    zip.write_all(scratch_project.to_json().as_bytes())?;

    for asset in assets {
        zip.start_file(asset.filename(rename)?, zip_options)?;

        let file = fs::read(&asset.path)?;
        zip.write_all(&file)?;
    }

    zip.finish()?;

    Ok(())
}

pub fn write_to_dir(
    output_dir: impl AsRef<Path>,
    scratch_project: manifest_scratch::Manifest,
    assets: &[Asset],
    rename: bool,
) -> Result<(), BuildError> {
    tracing::info!("Writing output directory {:?}...", output_dir.as_ref());

    fs::create_dir_all(output_dir.as_ref())?;
    let mut manifest = fs::File::create_new(output_dir.as_ref().join("project.json"))?;
    manifest.write_all(scratch_project.to_json().as_bytes())?;

    for asset in assets {
        fs::copy(
            &asset.path,
            output_dir.as_ref().join(&asset.filename(rename)?),
        )?;
    }

    Ok(())
}
