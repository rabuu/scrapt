use clap::Parser;

use scrapt::cli::{CliArgs, Cmd};
use scrapt::commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = CliArgs::parse();

    match cli.cmd {
        Cmd::Build(args) => {
            commands::build(args.project_path, args.manifest, args.output, args.no_zip)?
        }
        Cmd::Generate(_) => unimplemented!(),
        Cmd::New(args) => commands::new(args.path)?,
    }

    Ok(())
}
