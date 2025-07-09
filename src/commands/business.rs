use std::env;

use clap::{Args, Subcommand};

use crate::core::registry::manager::Manager as RegistryManager;
use crate::core::registry::types::{FileVersion, REGISTRY_VERSION_GENESIS};

use crate::core::business::app::App as BusinessApp;
use crate::core::business::types::{BusinessError, Definition, BUSINESS_DIR_NAME};

use crate::commands::adapters::business::processor::ProcessorAdapter as BusinessProcessorAdapter;
use crate::commands::adapters::path_buf_wrapper::PathBufAdapter;
use crate::commands::adapters::registry::processor::ProcessorAdapter as RegistryProcessorAdapter;

#[derive(Args)]
pub(crate) struct BusinessArgs {
    #[command(subcommand)]
    pub commands: Business,
}

#[derive(Subcommand)]
pub(crate) enum Business {
    /// Define a new business file
    Define {
        /// The name of the business to define
        #[arg(long, required = true)]
        business_name: String,

        /// The business file version
        #[arg(long, default_value = REGISTRY_VERSION_GENESIS)]
        business_version: Option<String>,

        /// The chosen programming language for the technical architecture stack
        #[arg(long, default_value = "Rust")]
        language: Option<String>,

        /// The name of the architect responsible for the business file
        /// Exampple: "Modular Monolith"
        #[arg(long, default_value = "Modular Monolith")]
        architect: Option<String>,

        /// The additional prompt message used to additional context to the LLM models
        #[arg(long, default_value = "")]
        additional_prompt: Option<String>,

        /// Whether to use C4 model for the business file
        #[arg(long, default_value = "false")]
        use_c4: Option<bool>,

        /// Whether to only output the JSON representation of the business file
        #[arg(long, default_value = "false")]
        only_json: Option<bool>,
    },
}

type TRegistryProcessor = RegistryProcessorAdapter;
type TPathBufWrapper = PathBufAdapter;
type TBusinessProcessor = BusinessProcessorAdapter<TPathBufWrapper>;

#[derive(Debug, Clone)]
pub(crate) struct Handler {
    app: BusinessApp<TBusinessProcessor, TRegistryProcessor, TPathBufWrapper>,
}

impl Handler {
    pub(crate) fn new() -> Result<Self, BusinessError> {
        let current_dir = env::current_dir().map_err(|err| BusinessError::FsError(err.into()))?;
        
        let registry_path_buf = PathBufAdapter::new(current_dir.join(BUSINESS_DIR_NAME));
        let registry_processor = RegistryProcessorAdapter::new();
        let registry_manager = RegistryManager::new(registry_processor, registry_path_buf);

        let business_path_buf = PathBufAdapter::new(current_dir.join(BUSINESS_DIR_NAME));
        let business_processor = BusinessProcessorAdapter::new(business_path_buf);
        let business_app = BusinessApp::new(business_processor, registry_manager);

        Ok(Self { app: business_app })
    }

    pub(crate) fn define(&self, args: BusinessArgs) -> Result<(), BusinessError> {
        match args.commands {
            Business::Define {
                business_name,
                business_version,
                ..
            } => self.app.define(
                Definition::from(business_name),
                business_version.map(|val| FileVersion::from(val)),
            ),
        }
    }
}
