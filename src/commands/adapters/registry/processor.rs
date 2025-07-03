use std::io::{BufWriter, BufReader};
use std::fs::File;
use std::path::PathBuf;

use crate::core::registry::types::{Processor, RegistryError, Registry};

pub(crate) struct ProcessorAdapter {}

impl Processor for ProcessorAdapter {
    fn build(&self, file_path: PathBuf, registry: Registry) -> Result<(), RegistryError> {
        let file = File::create(file_path).map_err(|e| RegistryError::FsError(e))?;

        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &registry)
            .map_err(|e| RegistryError::FsError(e.into()))?;

        Ok(())
    }

    fn parse(&self, file_path: PathBuf) -> Result<Registry, RegistryError> {
        let file = File::open(file_path).map_err(|e| RegistryError::FsError(e))?;
        let reader = BufReader::new(file);
        let registry: Registry = serde_json::from_reader(reader)
            .map_err(|e| RegistryError::FsError(e.into()))?;
        Ok(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::registry::types::{Directory, FileItem, FileName, REGISTRY_VERSION_GENESIS};

    // Test ProcessorAdapter build method
    #[test]
    fn test_processor_adapter_build() {
        let temp_dir = tempfile::tempdir().unwrap();

        let mut registry = Registry::new(Directory::from("businesses"));
        registry.add_file(FileItem::new(FileName::from("test_file.md")));

        let processor = ProcessorAdapter {};
        let file_path = temp_dir.path().join("registry.json");
        let result = processor.build(file_path.clone(), registry);
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    // Test ProcessorAdapter parse method
    #[test]
    fn test_processor_adapter_parse() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("registry.json");
    
        let mut registry = Registry::new(Directory::from("businesses"));
        registry.add_file(FileItem::new(FileName::from("test_file.md")));

        let processor = ProcessorAdapter {};
        let build_result = processor.build(file_path.clone(), registry);
        assert!(build_result.is_ok());

        let parsed_registry = processor.parse(file_path);
        assert!(parsed_registry.is_ok());

        let parsed_registry = parsed_registry.unwrap();
        assert_eq!(parsed_registry.files.len(), 1);

        let file_item = parsed_registry.get_file(&FileName::from("test_file.md"));
        assert!(file_item.is_some());
        assert_eq!(file_item.unwrap().name, FileName::from("test_file.md"));

        let file_version = file_item.unwrap().get_last_version();
        assert!(file_version.is_some());
        assert_eq!(file_version.unwrap().to_string(), REGISTRY_VERSION_GENESIS.to_string());
    }
}