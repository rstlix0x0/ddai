use clap::Parser;
use tracing::{debug, info, instrument};

mod core;

mod cli;
use cli::Commands;

mod commands;
use commands::project::{Handler as ProjectHandler, Project};

#[instrument]
pub fn exec() {
    debug!("initiate handlers");
    let project_handler = ProjectHandler::new();

    debug!("parsing CLI arguments");
    let cli = cli::Cli::parse();

    info!("Parsing CLI commands");
    match cli.commands {
        Commands::Project(args) => match args.commands {
            Project::Init { name, desc } => {
                project_handler.init(name, desc);
            }
        },
    }
}
