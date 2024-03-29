use std::fs;
use std::path::PathBuf;

use manifest_scrapt::Manifest as ScraptManifest;

pub use error::BuildError;
mod error;

mod write;

pub fn build(
    project_path: PathBuf,
    manifest_path: Option<PathBuf>,
    output_file: Option<PathBuf>,
    no_zip: bool,
) -> Result<(), BuildError> {
    tracing::debug!("Building...");

    let manifest_path = manifest_path.unwrap_or(project_path.join("project.toml"));

    let output_file = output_file.unwrap_or_else(|| {
        let mut f = PathBuf::from(".").join(
            project_path
                .file_name()
                .map(|os_str| os_str.to_str().unwrap_or("scratch-project"))
                .unwrap_or("scratch-project"),
        );

        if !no_zip {
            f.set_extension("sb3");
        }

        f
    });

    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path)?)?;

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    tracing::info!("Handle {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage)?;
    let _header_reg = lang::parse::parse_target(stage_tokens)?;

    let assets: Vec<PathBuf> = vec![];

    if no_zip {
        write::write_to_dir(output_file, &assets)?;
    } else {
        write::write_to_zip(output_file, &assets)?;
    }

    Ok(())
}
