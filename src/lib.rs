use clap::Parser;
use tracing::{debug, info, instrument};

mod core;

mod cli;
use cli::Commands;

mod commands;
use commands::business::Handler as BusinessHandler;
use commands::project::{Handler as ProjectHandler, Project};

#[instrument]
pub fn exec() {
    debug!("initiate handlers");
    let project_handler = ProjectHandler::new();

    debug!("initiate business handler");
    let business_handler = BusinessHandler::new().expect("Failed to create business handler");

    debug!("parsing CLI arguments");
    let cli = cli::Cli::parse();

    info!("Parsing CLI commands");
    match cli.commands {
        Commands::Project(args) => match args.commands {
            Project::Init { name, desc } => {
                project_handler.init(name, desc);
            }
        },
        Commands::Business(args) => {
            info!("Handling business commands");
            let result = business_handler.define(args);
            if let Err(e) = result {
                eprintln!("Error defining business: {}", e);
            } else {
                info!("Business defined successfully");
            }
        }
    }
}
