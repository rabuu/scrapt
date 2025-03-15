mod cli;
mod commands;

use clap::Parser;

use cli::{CliArgs, Cmd};
use commands::build::OutputType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = CliArgs::parse();

    match cli.cmd {
        Cmd::Build(args) => {
            let output_type = match args.no_zip {
                true => OutputType::Directory,
                false => OutputType::Zip,
            };

            commands::build(args.project_path, args.manifest, args.output, output_type)?
        }
        Cmd::Generate(_) => unimplemented!(),
        Cmd::New(args) => commands::new(args.path)?,
    }

    Ok(())
}
