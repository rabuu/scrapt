use std::fs;
use std::path::PathBuf;

use clap::Parser;
use scrapt::cli;

use manifest_scrapt::Manifest as ScraptManifest;

fn main() {
    let cli = cli::CliArgs::parse();

    match cli.cmd {
        cli::Cmd::Build(args) => {
            let project_path = args.project_path;
            let manifest_path = args
                .manifest_path
                .unwrap_or(project_path.join("project.toml"));

            build(project_path, manifest_path);
        }
        cli::Cmd::Generate(_) => unimplemented!(),
    }
}

fn build(project_path: PathBuf, manifest_path: PathBuf) {
    let _manifest = ScraptManifest::parse(&fs::read_to_string(manifest_path).unwrap()).unwrap();

    let stage = fs::read_to_string(project_path.join("stage.scr")).unwrap();

    let stage_tokens = lang::lex::tokenize(&stage);
    println!("{:#?}", stage_tokens);

    let target = lang::parse::parse(stage_tokens.into_iter().map(|(tok, _)| tok).collect());
    println!("{:#?}", target);
}
