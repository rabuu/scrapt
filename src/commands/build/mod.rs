use std::fs;
use std::path::PathBuf;

use asset::Asset;
pub use error::BuildCmdError;

use scratch_sb3::target::Target;

use scrapt::manifest::Manifest;
use scrapt::parsing;

mod asset;
mod error;
mod write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputType {
    #[default]
    /// Build the project to a ZIP archive (with .sb3 extension)
    Zip,

    /// Build the project to a normal directory without archiving
    Directory,
}

pub fn build(
    project_path: PathBuf,
    manifest_path: Option<PathBuf>,
    output_file: Option<PathBuf>,
    output_type: OutputType,
) -> Result<(), BuildCmdError> {
    tracing::info!("Building...");

    let project_path = project_path.canonicalize()?;

    let manifest_path = manifest_path.unwrap_or(project_path.join("project.toml"));
    let manifest_scrapt = Manifest::parse(&fs::read_to_string(manifest_path)?)?;

    let output_file = output_file.unwrap_or_else(|| {
        let mut f = PathBuf::from(
            project_path
                .file_name()
                .map(|os_str| os_str.to_str().unwrap_or("scratch-project"))
                .unwrap_or("scratch-project"),
        );

        if output_type == OutputType::Zip {
            f.set_extension("sb3");
        }

        f
    });

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    tracing::debug!("Handle {:?}...", stage_path);
    let headers: parsing::Headers = parsing::parse(&stage).unwrap();

    let mut s_builder = Target::stage_builder();
    let mut assets = Vec::new();

    for (costume_name, (filetype, path)) in &headers.costumes {
        let file_name = match path {
            Some(path) => path.clone(),
            None => PathBuf::from(costume_name.to_string()).with_extension(filetype.extension()),
        };

        let path = project_path
            .join(&manifest_scrapt.assets.directory)
            .join(file_name);

        if !path.is_file() {
            return Err(BuildCmdError::NoValidFileAt(path));
        }

        let asset = Asset::new(path, filetype.extension())?;
        s_builder = s_builder.add_costume(scratch_sb3::target::Asset::costume(
            asset.hash.clone(),
            costume_name.to_string(),
            asset.filename(manifest_scrapt.assets.auto_renaming)?,
            filetype.extension().to_string(),
        ));
        assets.push(asset);
    }

    for (sound_name, (filetype, path)) in &headers.sounds {
        let file_name = match path {
            Some(path) => path.clone(),
            None => PathBuf::from(sound_name.to_string()).with_extension(filetype.extension()),
        };

        let path = project_path
            .join(&manifest_scrapt.assets.directory)
            .join(file_name);

        if !path.is_file() {
            return Err(BuildCmdError::NoValidFileAt(path));
        }

        let asset = Asset::new(path, filetype.extension())?;
        s_builder = s_builder.add_sound(scratch_sb3::target::Asset::sound(
            asset.hash.clone(),
            sound_name.to_string(),
            asset.filename(manifest_scrapt.assets.auto_renaming)?,
            filetype.extension().to_string(),
        ));
        assets.push(asset);
    }

    let stage = s_builder
        .volume(99)
        .current_costume(headers.current_costume.map(|i| i as u32))
        .build();

    let scratch_project = scratch_sb3::Project::builder(stage).build();

    match output_type {
        OutputType::Zip => write::write_to_zip(
            output_file,
            scratch_project,
            &assets,
            manifest_scrapt.assets.auto_renaming,
        )?,
        OutputType::Directory => write::write_to_dir(
            output_file,
            scratch_project,
            &assets,
            manifest_scrapt.assets.auto_renaming,
        )?,
    }

    Ok(())
}
