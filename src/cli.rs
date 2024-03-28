use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = None, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Build a Scrapt project
    #[command(alias = "b")]
    Build(BuildArgs),

    /// Generate a Scrapt project from a Scratch file
    #[command(alias = "gen", alias = "g")]
    Generate(GenerateArgs),

    /// Create a new Scrapt project
    #[command(alias = "n")]
    New(NewArgs),
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Location of project directory
    pub project_path: PathBuf,

    /// Location of output file
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Path to a custom manifest (project.toml) location
    #[arg(short, long)]
    pub manifest: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct GenerateArgs;

#[derive(Args, Debug)]
pub struct NewArgs;
