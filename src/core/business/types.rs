use std::io::Error as IoError;
use thiserror::Error;

use crate::core::registry::types::{FileName, FileVersion, RegistryError};
use crate::core::types::{CoreError, Validator};

#[allow(dead_code)]
pub const BUSINESS_DIR_NAME: &str = "businesses";

#[derive(Debug, Error)]
pub(crate) enum BusinessError {
    #[allow(dead_code)]
    #[error("[business error] business definition not found: {0}")]
    NotFound(String),

    #[allow(dead_code)]
    #[error("[business error] business definition already exists: {0}")]
    AlreadyExists(String),

    #[allow(dead_code)]
    #[error("[business error] invalid business definition: {0}")]
    InvalidDefinition(String),

    #[error("[business error] core error: {0}")]
    CoreError(#[from] CoreError),

    #[error("[business error] fs error: {0}")]
    FsError(#[from] IoError),

    #[error("[business error] registry error: {0}")]
    RegistryError(#[from] RegistryError),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Definition(String);

impl Definition {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn to_filename(&self) -> FileName {
        FileName::from(self.as_str())
    }
}

impl From<String> for Definition {
    fn from(def: String) -> Self {
        Definition(def)
    }
}

impl From<&str> for Definition {
    fn from(def: &str) -> Self {
        Definition(def.to_string())
    }
}

impl Validator for Definition {
    fn validate(&self) -> Result<(), CoreError> {
        if self.0.is_empty() {
            return Err(CoreError::ValidationError(
                "Definition cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub(crate) trait Processor {
    fn define(&self, definition: Definition, version: FileVersion) -> Result<(), BusinessError>;
}
