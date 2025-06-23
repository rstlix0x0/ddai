use std::env;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Subcommand};
use tracing::{debug, error, info, instrument};

use crate::core::project::app::App as ProjectApp;
use crate::core::project::types::{
    Builder, Project as CoreProject, ProjectError, PROJECT_ARCHITECTURE_DIR_NAME,
    PROJECT_BUSINESS_DIR_NAME, PROJECT_CREDENTIAL_NAME, PROJECT_DIR_NAME, PROJECT_FILE_NAME,
    PROJECT_KNOWLEDGE_DIR_NAME,
};
use crate::core::types::ToJSON;

#[derive(Args)]
pub(crate) struct ProjectArgs {
    #[command(subcommand)]
    pub commands: Project,
}

#[derive(Subcommand)]
pub(crate) enum Project {
    /// Create a new project
    Init {
        /// The name of the project to initiate
        #[arg(long, required = true)]
        name: String,

        /// The description of the project
        #[arg(long)]
        desc: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct ProjectBuilderImpl;

impl ProjectBuilderImpl {
    #[instrument(skip_all, err)]
    fn create_project_dir(&self, current_dir: PathBuf) -> Result<(), ProjectError> {
        let project_dir = current_dir.join(PROJECT_DIR_NAME);
        debug!("Creating project directory at: {:?}", project_dir);

        debug!(
            "Checking if project directory exists: {:?}",
            project_dir.exists()
        );
        if !project_dir.exists() {
            let _ = create_dir(&project_dir).map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    fn create_project_file(&self, current_dir: PathBuf, json: String) -> Result<(), ProjectError> {
        let file_path = current_dir.join(format!("{}/{}", PROJECT_DIR_NAME, PROJECT_FILE_NAME));
        debug!("Project file path: {:?}", file_path);

        if !file_path.exists() {
            let mut file = File::create(&file_path).map_err(|e| ProjectError::FsError(e))?;
            file.write_all(json.as_bytes())
                .map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    fn manage_gitignore(&self, current_dir: PathBuf) -> Result<(), ProjectError> {
        let gitignore_path = current_dir.join(PROJECT_DIR_NAME).join(".gitignore");
        debug!("Creating .gitignore at: {:?}", gitignore_path);

        debug!(
            "Checking if .gitignore exists: {:?}",
            gitignore_path.exists()
        );
        if !gitignore_path.exists() {
            let mut file = File::create(gitignore_path).map_err(|e| ProjectError::FsError(e))?;
            file.write_all(PROJECT_CREDENTIAL_NAME.as_bytes())
                .map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    fn create_business_dir(&self, current_dir: PathBuf) -> Result<(), ProjectError> {
        let business_dir = current_dir.join(PROJECT_BUSINESS_DIR_NAME);
        debug!("Creating business directory at: {:?}", business_dir);

        if !business_dir.exists() {
            let _ = create_dir(&business_dir).map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    fn create_knowledge_dir(&self, current_dir: PathBuf) -> Result<(), ProjectError> {
        let knowledge_dir = current_dir.join(PROJECT_KNOWLEDGE_DIR_NAME);
        debug!("Creating knowledge directory at: {:?}", knowledge_dir);

        if !knowledge_dir.exists() {
            let _ = create_dir(&knowledge_dir).map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }

    #[instrument(skip_all, err)]
    fn create_architecture_dir(&self, current_dir: PathBuf) -> Result<(), ProjectError> {
        let architecture_dir = current_dir.join(PROJECT_ARCHITECTURE_DIR_NAME);
        debug!("Creating architecture directory at: {:?}", architecture_dir);

        if !architecture_dir.exists() {
            let _ = create_dir(&architecture_dir).map_err(|e| ProjectError::FsError(e))?;
        }

        Ok(())
    }
}

impl Builder for ProjectBuilderImpl {
    #[instrument(skip_all, err)]
    fn initiate(&self, project: CoreProject) -> Result<(), ProjectError> {
        info!("Initiating project: {}", project.name.as_str());
        let current_dir = env::current_dir().map_err(|e| ProjectError::FsError(e))?;
        debug!("Current directory: {:?}", current_dir);

        let json = project
            .to_json()
            .map_err(|e| ProjectError::InitiateError(e.to_string()))?;

        let _ = self.create_project_dir(current_dir.clone())?;
        let _ = self.create_project_file(current_dir.clone(), json)?;
        let _ = self.manage_gitignore(current_dir.clone())?;
        let _ = self.create_business_dir(current_dir.clone())?;
        let _ = self.create_knowledge_dir(current_dir.clone())?;
        let _ = self.create_architecture_dir(current_dir)?;

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct Handler {
    app: ProjectApp<ProjectBuilderImpl>,
}

impl Handler {
    #[instrument]
    pub fn new() -> Self {
        Handler {
            app: ProjectApp::new(ProjectBuilderImpl),
        }
    }

    #[instrument(skip_all)]
    pub fn init(&self, name: String, desc: Option<String>) {
        self.app
            .init(name.into(), desc.map(|d| d.into()))
            .unwrap_or_else(|err| {
                error!("Failed to initiate project: {}", err);
            });
    }
}
