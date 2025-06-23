use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::types::{CoreError, ToJSON, Validator};

#[allow(dead_code)]
pub(crate) const REGISTRY_VERSION_GENESIS: &str = "0.1.0";

#[allow(dead_code)]
pub(crate) const REGISTRY_FILE_NAME: &str = "registry.json";

#[derive(Error, Debug)]
#[allow(dead_code)]
pub(crate) enum RegistryError {
    #[error("[registry error] invalid version: {0}")]
    InvalidVersion(String),

    #[error("[registry error] file not found")]
    FileNotFound,

    #[error("[registry error] filesystem error: {0}")]
    FsError(#[from] std::io::Error),

    #[error("[registry error] core error: {0}")]
    CoreError(#[from] CoreError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct FileName(String);

impl FileName {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl From<String> for FileName {
    fn from(name: String) -> Self {
        FileName(name)
    }
}

impl From<&str> for FileName {
    fn from(name: &str) -> Self {
        FileName(name.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct FileVersion(String);

impl FileVersion {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FileVersion::from(REGISTRY_VERSION_GENESIS)
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl Validator for FileVersion {
    fn validate(&self) -> Result<(), CoreError> {
        if self.0.is_empty() {
            return Err(CoreError::ValidationError(
                "File version cannot be empty".to_string(),
            ));
        }

        if !self.0.chars().all(|c| c.is_ascii_digit() || c == '.') {
            return Err(CoreError::ValidationError(
                "File version can only contain digit characters & dots".to_string(),
            ));
        }

        let parts: Vec<&str> = self.0.split('.').collect();
        if parts.is_empty() || parts.iter().any(|part| part.is_empty()) {
            return Err(CoreError::ValidationError(
                "File version must contain at least one non-empty part".to_string(),
            ));
        }

        if parts.len() != 3 {
            return Err(CoreError::ValidationError(
                "File version can have at most three parts".to_string(),
            ));
        }

        for part in parts.clone() {
            if let Err(_) = part.parse::<u32>() {
                return Err(CoreError::ValidationError(
                    "Each part of the file version must be a valid unsigned integer".to_string(),
                ));
            }
        }

        if parts[0].parse::<u32>().unwrap() == 0
            && parts[1].parse::<u32>().unwrap() == 0
            && parts[2].parse::<u32>().unwrap() == 0
        {
            return Err(CoreError::ValidationError(
                "File version cannot be zero".to_string(),
            ));
        }

        if parts[0].parse::<u32>().unwrap() > 255
            || parts[1].parse::<u32>().unwrap() > 255
            || parts[2].parse::<u32>().unwrap() > 255
        {
            return Err(CoreError::ValidationError(
                "Each part of the file version must be between 0 and 255".to_string(),
            ));
        }
        Ok(())
    }
}

impl From<String> for FileVersion {
    fn from(version: String) -> Self {
        FileVersion(version)
    }
}

impl From<&str> for FileVersion {
    fn from(version: &str) -> Self {
        FileVersion(version.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Directory(String);

impl Directory {
    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    #[allow(dead_code)]
    pub(crate) fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl From<String> for Directory {
    fn from(dir: String) -> Self {
        Directory(dir)
    }
}

impl From<&str> for Directory {
    fn from(dir: &str) -> Self {
        Directory(dir.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct FileItem {
    pub(crate) name: FileName,
    pub(crate) versions: Vec<FileVersion>,
}

impl FileItem {
    #[allow(dead_code)]
    pub(crate) fn new(name: FileName) -> Self {
        let mut versions = Vec::<FileVersion>::new();
        versions.push(FileVersion::new());

        FileItem {
            name,
            versions: versions,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_last_version(&self) -> Option<FileVersion> {
        self.versions.last().and_then(|val| Some(val.to_owned()))
    }

    #[allow(dead_code)]
    pub(crate) fn update(&mut self, version: FileVersion) {
        if !self.versions.contains(&version) {
            self.versions.push(version);
        }
    }
}

impl From<&FileItem> for FileItem {
    fn from(file_item: &FileItem) -> Self {
        file_item.clone()
    }
}

impl Validator for FileItem {
    fn validate(&self) -> Result<(), CoreError> {
        if self.name.as_str().is_empty() {
            return Err(CoreError::ValidationError(
                "File name cannot be empty".to_string(),
            ));
        }

        if self.versions.is_empty() {
            return Err(CoreError::ValidationError(
                "File must have at least one version".to_string(),
            ));
        }

        for version in &self.versions {
            version.validate()?;
        }

        Ok(())
    }
}

/// `Registry` struct represents a collection of files and their versions within a specific directory.
///
/// It contains a directory path and a list of file items, each with its name and associated versions.
/// It will be saved as a JSON file in the specified directory.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct Registry {
    pub(crate) directory: Directory,
    pub(crate) files: Vec<FileItem>,
}

impl Registry {
    #[allow(dead_code)]
    pub(crate) fn new(directory: Directory) -> Self {
        Registry {
            directory,
            files: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove_file(&mut self, file_name: &FileName) {
        self.files.retain(|file| &file.name != file_name);
    }

    /// Adds a file to the registry. If the file already exists, it updates its versions.
    #[allow(dead_code)]
    pub(crate) fn add_file(&mut self, file: FileItem) {
        let filter_file = self.files.iter().find(|f| f.name == file.name);

        match filter_file {
            Some(_) => {
                // If the file already exists, update its versions
                self.remove_file(&file.name);
                self.files.push(file.clone());
            }
            None => {
                // If the file does not exist, add it to the registry
                self.files.push(file);
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_file(&self, file_name: &FileName) -> Option<&FileItem> {
        self.files.iter().find(|file| &file.name == file_name)
    }
}

impl ToJSON for Registry {}

/// This trait defines the interface for processing registry files.
///
/// It includes methods for building a registry from a file path and a registry object,
/// as well as parsing a registry file to create a [`Registry`] object.
#[allow(dead_code)]
pub(crate) trait Processor {
    fn build(&self, file_path: PathBuf, registry: Registry) -> Result<(), RegistryError>;
    fn parse(&self, file_path: PathBuf) -> Result<Registry, RegistryError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_file_version {
        use super::*;

        #[test]
        fn test_file_version_validation() {
            let valid_version = FileVersion::from("1.0.0");
            assert!(valid_version.validate().is_ok());

            let invalid_version = FileVersion::from("1.0");
            assert!(invalid_version.validate().is_err());

            let invalid_version_empty = FileVersion::from("");
            assert!(invalid_version_empty.validate().is_err());

            let invalid_version_non_digit = FileVersion::from("1.a.0");
            assert!(invalid_version_non_digit.validate().is_err());

            let invalid_version_zero = FileVersion::from("0.0.0");
            assert!(invalid_version_zero.validate().is_err());

            let invalid_version_out_of_range = FileVersion::from("256.0.0");
            assert!(invalid_version_out_of_range.validate().is_err());
        }

        #[test]
        fn test_file_version_to_string() {
            let version = FileVersion::from("1.0.0");
            assert_eq!(version.to_string(), "1.0.0");
            assert_eq!(version.as_str(), "1.0.0");
        }

        #[test]
        fn test_file_version_from_string() {
            let version = FileVersion::from("2.1.3".to_string());
            assert_eq!(version.to_string(), "2.1.3");
            assert_eq!(version.as_str(), "2.1.3");
        }

        #[test]
        fn test_file_version_from_str() {
            let version = FileVersion::from("3.2.1");
            assert_eq!(version.to_string(), "3.2.1");
            assert_eq!(version.as_str(), "3.2.1");
        }

        #[test]
        fn test_file_version_new() {
            let version = FileVersion::new();
            assert_eq!(version.to_string(), REGISTRY_VERSION_GENESIS);
            assert_eq!(version.as_str(), REGISTRY_VERSION_GENESIS);
        }
    }

    mod test_file_item {
        use super::*;

        #[test]
        fn test_file_item_validation() {
            let valid_file = FileItem::new(FileName::from("test_file"));
            assert!(valid_file.validate().is_ok());

            let invalid_file_empty_name = FileItem {
                name: FileName::from(""),
                versions: vec![FileVersion::new()],
            };
            assert!(invalid_file_empty_name.validate().is_err());

            let invalid_file_no_versions = FileItem {
                name: FileName::from("test_file"),
                versions: Vec::new(),
            };
            assert!(invalid_file_no_versions.validate().is_err());
            let invalid_file_version = FileItem {
                name: FileName::from("test_file"),
                versions: vec![FileVersion::from("1.0.0"), FileVersion::from("invalid")],
            };
            assert!(invalid_file_version.validate().is_err());
        }

        #[test]
        fn test_file_item_update() {
            let mut file_item = FileItem::new(FileName::from("test_file"));
            let new_version = FileVersion::from("1.0.0");
            file_item.update(new_version.clone());
            assert!(file_item.versions.contains(&new_version));

            let another_version = FileVersion::from("1.0.1");
            file_item.update(another_version);
            assert_eq!(file_item.versions.len(), 3);
            assert!(file_item
                .versions
                .contains(&FileVersion::from(REGISTRY_VERSION_GENESIS)));
            assert!(file_item.versions.contains(&FileVersion::from("1.0.0")));
            assert!(file_item.versions.contains(&FileVersion::from("1.0.1")));
        }
    }

    mod test_registry {
        use super::*;

        #[test]
        fn test_registry_add_file() {
            let mut registry = Registry::new(Directory::from("test_dir"));
            let file_item = FileItem::new(FileName::from("test_file"));

            registry.add_file(file_item.clone());
            assert_eq!(registry.files.len(), 1);
            assert!(registry.files.contains(&file_item));

            // Adding the same file again should not duplicate it
            registry.add_file(file_item.clone());
            assert_eq!(registry.files.len(), 1);

            // Adding a file with a new version should update the existing file
            let new_version = FileVersion::from("1.0.0");
            let mut updated_file_item = file_item.clone();
            updated_file_item.update(new_version.clone());
            assert!(updated_file_item.versions.contains(&new_version));
            assert!(updated_file_item.versions.len() == 2);

            registry.add_file(updated_file_item);
            assert_eq!(registry.files.len(), 1);
            assert!(registry.files[0].versions.contains(&FileVersion::new()));
            assert!(registry.files[0].versions.contains(&new_version));
        }

        #[test]
        fn test_registry_remove_file() {
            let mut registry = Registry::new(Directory::from("test_dir"));
            let file_name = FileName::from("test_file");
            let file_item = FileItem::new(file_name.clone());

            registry.add_file(file_item.clone());
            assert_eq!(registry.files.len(), 1);

            registry.remove_file(&file_name);
            assert_eq!(registry.files.len(), 0);

            // Removing a non-existing file should not panic
            registry.remove_file(&file_name);
            assert_eq!(registry.files.len(), 0);
        }

        #[test]
        fn test_registry_get_file() {
            let mut registry = Registry::new(Directory::from("test_dir"));
            let file_name = FileName::from("test_file");
            let file_item = FileItem::new(file_name.clone());

            registry.add_file(file_item.clone());
            assert_eq!(registry.get_file(&file_name), Some(&file_item));

            // Getting a non-existing file should return None
            let non_existing_file_name = FileName::from("non_existing_file");
            assert_eq!(registry.get_file(&non_existing_file_name), None);
        }

        mod test_registry_json {
            use super::*;

            #[test]
            fn test_registry_to_json() {
                let registry = Registry::new(Directory::from("test_dir"));
                let json = registry.to_json().unwrap();
                assert!(!json.is_empty());
            }

            #[test]
            fn test_registry_from_json() {
                let registry = Registry::new(Directory::from("test_dir"));
                let json = registry.to_json().unwrap();
                let deserialized_registry: Registry = serde_json::from_str(&json).unwrap();
                assert_eq!(registry, deserialized_registry);
            }
        }
    }
}
