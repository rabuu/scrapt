use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use tracing::trace;

use manifest_scrapt::Manifest as ScraptManifest;
use scrapt::cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::CliArgs::parse();

    match cli.cmd {
        cli::Cmd::Build(args) => {
            let project_path = args.project_path;
            let manifest_path = args
                .manifest_path
                .unwrap_or(project_path.join("project.toml"));

            build(project_path, manifest_path)?;
        }
        cli::Cmd::Generate(_) => unimplemented!(),
    }

    Ok(())
}

fn build(project_path: PathBuf, manifest_path: PathBuf) -> Result<()> {
    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path).unwrap()).unwrap();

    let stage_path = project_path.join("stage.scr");
    let stage = fs::read_to_string(&stage_path).unwrap();

    trace!("Try to tokenize {:?}...", stage_path);
    let stage_tokens = lang::lex::tokenize(stage).context("lexing")?;

    trace!("Try to parse contents of {:?}...", stage_path);
    let header_reg = lang::parse::parse_target(stage_tokens).context("parsing")?;

    dbg!(header_reg);

    Ok(())
}
