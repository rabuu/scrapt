use std::fs;
use std::path::PathBuf;

use tracing::trace;

use manifest_scrapt::Manifest as ScraptManifest;

pub use error::BuildError;
mod error;

pub fn build(project_path: PathBuf, manifest_path: PathBuf) -> Result<(), BuildError> {
    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path).unwrap()).unwrap();

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    trace!("Try to tokenize {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage)?;

    trace!("Try to parse contents of {:?}...", stage_path);
    let header_reg = lang::parse::parse_target(stage_tokens)?;

    dbg!(header_reg);

    Ok(())
}
