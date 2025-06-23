use clap::{Parser, Subcommand};

use crate::commands::project;

#[derive(Parser)]
#[command(
    name = "ddai",
    version = "0.1.0",
    about = "A command-line tool to integrate DDD and AI principles"
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Project(project::ProjectArgs),
}
