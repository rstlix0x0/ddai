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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Language(String);

impl Language {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for Language {
    fn from(lang: String) -> Self {
        Language(lang)
    }
}

impl From<&str> for Language {
    fn from(lang: &str) -> Self {
        Language(lang.to_string())
    }
}

impl Validator for Language {
    fn validate(&self) -> Result<(), CoreError> {
        if self.0.is_empty() {
            return Err(CoreError::ValidationError(
                "Language cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Architecture(String);

impl From<String> for Architecture {
    fn from(arch: String) -> Self {
        Architecture(arch)
    }
}

impl From<&str> for Architecture {
    fn from(arch: &str) -> Self {
        Architecture(arch.to_string())
    }
}

impl Validator for Architecture {
    fn validate(&self) -> Result<(), CoreError> {
        if self.0.is_empty() {
            return Err(CoreError::ValidationError(
                "Architecture cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

pub(crate) struct AdditionalPrompt(String);

impl AdditionalPrompt {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for AdditionalPrompt {
    fn from(prompt: String) -> Self {
        AdditionalPrompt(prompt)
    }
}

impl From<&str> for AdditionalPrompt {
    fn from(prompt: &str) -> Self {
        AdditionalPrompt(prompt.to_string())
    }
}

impl Validator for AdditionalPrompt {
    fn validate(&self) -> Result<(), CoreError> {
        if self.0.is_empty() {
            return Err(CoreError::ValidationError(
                "Additional prompt cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[allow(dead_code)]
/// AnalyzeParameters is a struct that holds the parameters required for analyzing a business definition.
/// 
/// All of these parameters used to get the file content and send it to the AI model for analysis.
/// When sending to the AI model, the parameters will be used to generate a prompt that includes
/// the definition, version, language, architecture, and any additional prompt provided.
#[derive(Clone, Debug, PartialEq)]
pub struct AnalyzeParameters {
    pub(crate) definition: Definition,
    pub(crate) version: FileVersion,
    pub(crate) language: Language,
    pub(crate) architecture: Architecture,
    pub(crate) additional_prompt: Option<String>,
    pub(crate) use_c4: bool,
    pub(crate) only_json: bool,
}

impl AnalyzeParameters {
    #[allow(dead_code)]
    pub(crate) fn new(
        definition: Definition,
        version: FileVersion,
        language: Language,
        architecture: Architecture,
    ) -> Self {
        AnalyzeParameters {
            definition,
            version,
            language,
            architecture,
            additional_prompt: None,
            use_c4: false,
            only_json: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_additional_prompt(
        mut self,
        additional_prompt: String,
    ) -> Self {
        self.additional_prompt = Some(additional_prompt);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_use_c4(mut self, use_c4: bool) -> Self {
        self.use_c4 = use_c4;
        self
    }
    
    #[allow(dead_code)]
    pub(crate) fn with_only_json(mut self, only_json: bool) -> Self {
        self.only_json = only_json;
        self
    }
}

impl Validator for AnalyzeParameters {
    fn validate(&self) -> Result<(), CoreError> {
        self.definition.validate()?;
        self.version.validate()?;
        self.language.validate()?;
        self.architecture.validate()?;

        if let Some(prompt) = &self.additional_prompt {
            AdditionalPrompt::from(prompt.as_str()).validate()?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
pub(crate) trait Processor {
    /// define is a method that defines a business definition with the given parameters.
    /// 
    /// This method should be used to create a business definition in the system. 
    fn define(&self, definition: Definition, version: FileVersion) -> Result<(), BusinessError>;
}
