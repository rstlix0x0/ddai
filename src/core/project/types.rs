use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::types::{CoreError, ToJSON, Validator};

pub const PROJECT_DIR_NAME: &str = ".ddai";
pub const PROJECT_FILE_NAME: &str = "project.json";
pub const PROJECT_CREDENTIAL_NAME: &str = "credentials.json";
pub const PROJECT_BUSINESS_DIR_NAME: &str = "businesses";
pub const PROJECT_ARCHITECTURE_DIR_NAME: &str = "architectures";

#[derive(Debug, Error)]
pub(crate) enum ProjectError {
    #[error("[project error] project unable to initiate: {0}")]
    InitiateError(String),

    #[error("[project error] filesystem error: {0}")]
    FsError(#[from] std::io::Error),

    #[error("[project error] validation error: {0}")]
    ValidationError(#[from] CoreError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Name(String);

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Default for Name {
    fn default() -> Self {
        Name("".to_string())
    }
}

impl From<String> for Name {
    fn from(name: String) -> Self {
        Name(name)
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Name(name.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Desc(String);

impl Desc {
    pub fn as_str(&self) -> &str {
        &self.0.as_str()
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Default for Desc {
    fn default() -> Self {
        Desc("".to_string())
    }
}

impl From<String> for Desc {
    fn from(desc: String) -> Self {
        Desc(desc)
    }
}

impl From<&str> for Desc {
    fn from(desc: &str) -> Self {
        Desc(desc.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub(crate) name: Name,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<Desc>,

    pub(crate) created_at: DateTime<Utc>,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: Name::default(),
            description: None,
            created_at: Utc::now(),
        }
    }
}

impl Project {
    pub fn new(name: Name, description: Option<Desc>) -> Self {
        Project {
            name,
            description,
            created_at: Utc::now(),
        }
    }
}

impl Validator for Project {
    fn validate(&self) -> Result<(), CoreError> {
        if self.name.as_str().is_empty() {
            return Err(CoreError::ValidationError(
                "Project name cannot be empty".to_string(),
            ));
        }

        if let Some(desc) = &self.description {
            if desc.as_str().is_empty() {
                return Err(CoreError::ValidationError(
                    "Project description cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl ToJSON for Project {}

pub(crate) trait Builder {
    fn initiate(&self, project: Project) -> Result<(), ProjectError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new(
            Name::from("Test Project"),
            Some(Desc::from("This is a test project.")),
        );
        assert_eq!(project.name.as_str(), "Test Project");
        assert_eq!(
            project.description.unwrap().as_str(),
            "This is a test project."
        );
    }

    #[test]
    fn test_default_project() {
        let project: Project = Default::default();
        assert_eq!(project.name.as_str(), "");
        assert!(project.description.is_none());
    }

    #[test]
    fn test_to_json() {
        let project = Project::new(
            Name::from("Sample Project"),
            Some(Desc::from("A sample project description.")),
        );

        let json = project.to_json().unwrap();
        assert!(!json.is_empty())
    }

    mod validation {
        use super::*;
        use crate::core::types::validate;

        mod expect_errors {
            use super::*;

            #[test]
            fn empty_name() {
                let project = Project::new(Name::default(), None);
                let result = validate(&project);
                assert!(result.is_err());

                let err = result.unwrap_err();
                assert!(matches!(err, CoreError::ValidationError(_)));
                assert!(err.to_string().contains("Project name cannot be empty"));
            }

            #[test]
            fn empty_description() {
                let project = Project::new(Name::from("Valid Name"), Some(Desc::default()));
                let result = validate(&project);
                assert!(result.is_err());

                let err = result.unwrap_err();
                assert!(matches!(err, CoreError::ValidationError(_)));
                assert!(err
                    .to_string()
                    .contains("Project description cannot be empty"));
            }
        }
    }
}
