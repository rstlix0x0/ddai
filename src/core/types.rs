use std::path::PathBuf;

use serde::Serialize;
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CoreError {
    #[error("[core error] json error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("[core error] validation error: {0}")]
    ValidationError(String),
}

impl CoreError {
    pub fn to_string(&self) -> String {
        match self {
            CoreError::JSONError(err) => format!("[core error] json error: {}", err),
            CoreError::ValidationError(msg) => format!("[core error] validation error: {}", msg),
        }
    }
}

pub fn validate<T: Validator>(item: &T) -> Result<(), CoreError> {
    item.validate()
}

pub(crate) trait Validator {
    fn validate(&self) -> Result<(), CoreError>;
}

pub(crate) trait ToJSON {
    fn to_json(&self) -> Result<String, CoreError>
    where
        Self: Serialize,
    {
        let out = serde_json::to_string_pretty(self).map_err(|err| CoreError::JSONError(err))?;
        Ok(out)
    }
}

/// `PathBufWrapper` trait provides a way to work with `PathBuf` in a more abstract manner.
///
/// Since we are using `PathBuf` in multiple places, this trait allows us to define common behaviors
/// for path handling, such as converting to `PathBuf`, getting directory names, and checking existence.
///
/// The objects implementing this trait can be used interchangeably in the codebase,
/// providing a consistent interface for path operations.
pub(crate) trait PathBufWrapper {
    fn to_path_buf(&self) -> PathBuf;
    fn dir_name(&self) -> Option<String>;
    fn exists(&self) -> bool;
}
