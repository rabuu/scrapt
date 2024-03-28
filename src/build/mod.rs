use std::fs;
use std::path::PathBuf;

use manifest_scrapt::Manifest as ScraptManifest;

pub use error::BuildError;
mod error;

pub fn build(project_path: PathBuf, manifest_path: PathBuf) -> Result<(), BuildError> {
    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path).unwrap()).unwrap();

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    tracing::info!("Handle {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage)?;
    let header_reg = lang::parse::parse_target(stage_tokens)?;

    dbg!(header_reg);

    Ok(())
}
