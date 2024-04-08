use clap::Parser;

use scrapt::{build, cli, new};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::CliArgs::parse();

    match cli.cmd {
        cli::Cmd::Build(args) => {
            build::build(args.project_path, args.manifest, args.output, args.no_zip)?
        }
        cli::Cmd::Generate(_) => unimplemented!(),
        cli::Cmd::New(args) => new::new(args.path)?,
    }

    Ok(())
}
