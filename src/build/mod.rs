use std::fs;
use std::path::PathBuf;

use manifest_scrapt::Manifest as ScraptManifest;

pub use error::BuildError;

use crate::build::asset::Asset;
mod error;

mod asset;
mod write;

pub fn build(
    project_path: PathBuf,
    manifest_path: Option<PathBuf>,
    output_file: Option<PathBuf>,
    no_zip: bool,
) -> Result<(), BuildError> {
    tracing::info!("Building...");

    let project_path = project_path.canonicalize()?;

    let manifest_path = manifest_path.unwrap_or(project_path.join("project.toml"));
    let manifest_scrapt = ScraptManifest::parse(&fs::read_to_string(manifest_path)?)?;

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

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    tracing::debug!("Handle {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage)?;
    let header_reg = lang::parse::parse_target(stage_tokens)?;

    let mut s_builder = manifest_scratch::Target::stage_builder();
    let mut assets = Vec::new();

    for (name, costume) in header_reg.costumes.iter() {
        let path = project_path
            .join(&manifest_scrapt.assets.directory)
            .join(&costume.path);

        if !path.is_file() {
            return Err(BuildError::NoValidFileAt(path));
        }

        let asset = Asset::new(path)?;
        assets.push(asset.clone());

        s_builder = s_builder.add_costume(manifest_scratch::Asset::costume(
            asset.hash.clone(),
            name.clone(),
            asset.filename(manifest_scrapt.assets.auto_renaming)?,
            costume.img_type.file_extension().to_string(),
        ));
    }

    let stage = s_builder.volume(99).build();

    let scratch_project = manifest_scratch::Manifest::builder(stage).build();

    if no_zip {
        write::write_to_dir(
            output_file,
            scratch_project,
            &assets,
            manifest_scrapt.assets.auto_renaming,
        )?;
    } else {
        write::write_to_zip(
            output_file,
            scratch_project,
            &assets,
            manifest_scrapt.assets.auto_renaming,
        )?;
    }

    Ok(())
}
