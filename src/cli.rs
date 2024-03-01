use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Simple program to greet a person
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

    /// Generate a Scrapt project
    #[command(alias = "gen", alias = "g")]
    Generate(GenerateArgs),
}

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Path to the project folder
    pub project_path: PathBuf,

    /// Path to a custom manifest (project.toml) location
    pub manifest_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct GenerateArgs;
