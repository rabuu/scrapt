use std::path::PathBuf;

use clap::Parser;

use scrapt::{build, cli};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::CliArgs::parse();

    match cli.cmd {
        cli::Cmd::Build(args) => {
            let project_path = args.project_path;
            let manifest_path = args.manifest.unwrap_or(project_path.join("project.toml"));
            let output_file = args.output.unwrap_or_else(|| {
                let mut f = PathBuf::from(".").join(
                    project_path
                        .file_name()
                        .map(|os_str| os_str.to_str().unwrap_or("scratch-project"))
                        .unwrap_or("scratch-project"),
                );
                f.set_extension("sb3");
                f
            });

            build::build(project_path, manifest_path, output_file, args.no_zip)?;
        }
        cli::Cmd::Generate(_) => unimplemented!(),
        cli::Cmd::New(_) => unimplemented!(),
    }

    Ok(())
}
