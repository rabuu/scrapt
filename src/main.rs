use clap::Parser;

use scrapt::{build, cli};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::CliArgs::parse();

    match cli.cmd {
        cli::Cmd::Build(args) => {
            let project_path = args.project_path;
            let manifest_path = args.manifest.unwrap_or(project_path.join("project.toml"));

            build::build(project_path, manifest_path)?;
        }
        cli::Cmd::Generate(_) => anyhow::bail!("not yet implemented"),
        cli::Cmd::New(_) => anyhow::bail!("not yet implemented"),
    }

    Ok(())
}
