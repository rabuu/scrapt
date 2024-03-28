use std::fs;
use std::io::Write;
use std::path::PathBuf;

use manifest_scrapt::Manifest as ScraptManifest;

pub use error::BuildError;
use zip::write::FileOptions;
use zip::ZipWriter;
mod error;

pub fn build(
    project_path: PathBuf,
    manifest_path: PathBuf,
    output_file: PathBuf,
    no_zip: bool,
) -> Result<(), BuildError> {
    tracing::debug!("Building...");

    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path)?)?;

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    tracing::info!("Handle {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage)?;
    let _header_reg = lang::parse::parse_target(stage_tokens)?;

    if no_zip {
        unimplemented!()
    } else {
        let output_file = fs::File::create(output_file)?;
        let mut zip = ZipWriter::new(output_file);
        let zip_options = FileOptions::default();
        zip.start_file("project.json", zip_options)?;
        zip.write_all(b"Hello JSON!")?;
        zip.finish()?;
    }

    Ok(())
}
