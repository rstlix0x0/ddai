use crate::core::types::{validate, PathBufWrapper};

use crate::core::registry::manager::Manager as RegistryManager;
use crate::core::registry::types::{FileVersion, Processor as RegistryProcessor};

use crate::core::business::types::{AnalyzeParameters, BusinessError, Definition, Processor, FsOperator};

#[allow(dead_code)]
pub(crate) struct Manager<P, RP, PW, FSP>
where
    P: Processor,
    RP: RegistryProcessor,
    PW: PathBufWrapper,
    FSP: FsOperator,
{
    processor: P,
    registry: RegistryManager<RP, PW>,
    fs_operator: FSP,
}

impl<P, RP, PW, FSP> Manager<P, RP, PW, FSP>
where
    P: Processor,
    RP: RegistryProcessor,
    PW: PathBufWrapper,
    FSP: FsOperator,
{
    #[allow(dead_code)]
    pub(crate) fn new(processor: P, registry: RegistryManager<RP, PW>, fs_operator: FSP) -> Self {
        Manager {
            processor,
            registry,
            fs_operator,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn define(
        &self,
        definition: Definition,
        version: Option<FileVersion>,
    ) -> Result<(), BusinessError> {
        let _ =
            validate(&definition).map_err(|e| BusinessError::InvalidDefinition(e.to_string()))?;

        // if user does not provide a version, we will use the default version
        let file_version = match version {
            Some(v) => v,
            None => FileVersion::new(),
        };

        // start defining the business definition with its version
        let _ = self
            .processor
            .define(definition.clone(), file_version.clone())?;

        // once the business def defined, we need to update registry
        self.registry
            .update_registry(definition.to_filename(), file_version)
            .map_err(|err| BusinessError::RegistryError(err))
    }

    #[allow(dead_code)]
    pub(crate) fn analyze(&self, params: AnalyzeParameters) -> Result<(), BusinessError> {
        let _ = validate(&params).map_err(|e| BusinessError::InvalidParameters(e.to_string()))?;
        self.processor.analyze(params)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use mockall::{mock, predicate::eq};

    use crate::core::registry::types::{Directory, FileName, Registry, RegistryError};
    use crate::core::business::types::{Definition, Processor, AnalyzeParameters, Architecture};

    mock!(
        FakeFsOperator{}

        impl FsOperator for FakeFsOperator {
            type DirPath = Directory;

            fn read_file(&self, name: FileName, version: FileVersion) -> Result<String, BusinessError>;
            fn write_file(&self, name: FileName, version: FileVersion, content: &str) -> Result<(), BusinessError>;
        }
    );

    mock!(
        FakeRegistryProcessor{}

        impl RegistryProcessor for FakeRegistryProcessor {
            fn build(&self, path: PathBuf, registry: Registry) -> Result<(), RegistryError>;
            fn parse(&self, path: PathBuf) -> Result<Registry, RegistryError>;
        }
    );

    mock!(
        FakePathBufWrapper{}

        impl PathBufWrapper for FakePathBufWrapper {
            fn to_path_buf(&self) -> PathBuf;
            fn dir_name(&self) -> Option<String>;
            fn exists(&self) -> bool;
        }
    );

    mock!(
        FakeProcessor{}

        impl Processor for FakeProcessor {
            fn define(&self, definition: Definition, version: FileVersion) -> Result<(), BusinessError>;
            fn analyze(&self, params: AnalyzeParameters) -> Result<(), BusinessError>;
        }
    );

    mod test_define {
        use super::*;
        
        use crate::core::registry::types::{Directory, FileItem, FileName};

        #[test]
        fn test_define_new_business_def() {
            let mut processor = MockFakeProcessor::new();
            processor
                .expect_define()
                .with(eq(Definition::from("test_file")), eq(FileVersion::new()))
                .returning(|_, _| Ok(()));

            let expected_registry = Registry::new(Directory::from("output"));
            let mut registry_processor = MockFakeRegistryProcessor::new();
            registry_processor.expect_build().returning(|_, _| Ok(()));
            registry_processor
                .expect_parse()
                .returning(move |_| Ok(expected_registry.clone()));

            let mut path_buf_wrapper = MockFakePathBufWrapper::new();
            path_buf_wrapper.expect_exists().returning(|| true);
            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper
                .expect_dir_name()
                .returning(|| Some("output".to_string()));

            let fs_operator = MockFakeFsOperator::new();
            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);
            let manager = Manager::new(processor, registry, fs_operator);
            let result = manager.define(Definition::from("test_file"), None);
            assert!(result.is_ok())
        }

        #[test]
        fn test_define_existed_business_def() {
            let mut processor = MockFakeProcessor::new();
            processor
                .expect_define()
                .with(
                    eq(Definition::from("test_file")),
                    eq(FileVersion::from("1.0.0")),
                )
                .returning(|_, _| Ok(()));

            let mut expected_file_item = FileItem::new(FileName::from("test_file"));
            expected_file_item.update(FileVersion::from("1.0.0"));

            let mut expected_registry = Registry::new(Directory::from("output"));
            expected_registry.add_file(expected_file_item);

            let mut registry_processor = MockFakeRegistryProcessor::new();
            registry_processor.expect_build().returning(|_, _| Ok(()));
            registry_processor
                .expect_parse()
                .returning(move |_| Ok(expected_registry.clone()));

            let mut path_buf_wrapper = MockFakePathBufWrapper::new();
            path_buf_wrapper.expect_exists().returning(|| true);
            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper
                .expect_dir_name()
                .returning(|| Some("output".to_string()));

            let fs_operator = MockFakeFsOperator::new();
            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);
            let manager = Manager::new(processor, registry, fs_operator);
            let result = manager.define(
                Definition::from("test_file"),
                Some(FileVersion::from("1.0.0")),
            );
            assert!(result.is_ok())
        }
    }

    mod test_analyze {
        use super::*;
        
        use crate::core::business::types::Language;

        mod test_validation {
            use  super::*;
            use crate::core::{business::types::{Architecture, Definition, Language}, types::CoreError};

            // Test for the validation of AnalyzeParameters

            // Test all required fields are provided
            #[test]
            fn test_analyze_parameters_validation_success() {
                let params = AnalyzeParameters::new(
                    Definition::from("test_definition"), 
                    FileVersion::new(), 
                    Language::from("Rust"), 
                    Architecture::from("Microservices")
                );

                let result = validate(&params);
                assert!(result.is_ok());
            }

            // Test missing definition
            #[test]
            fn test_analyze_parameters_validation_missing_definition() {
                let params = AnalyzeParameters::new(
                    Definition::from(""),
                    FileVersion::new(), 
                    Language::from("Rust"),
                    Architecture::from("Microservices"));

                let result = validate(&params);
                assert!(result.is_err());
                assert!(result.as_ref()
                    .unwrap_err()
                    .to_string()
                    .contains("Definition cannot be empty"));
                assert!(matches!(
                    result.unwrap_err(),
                    CoreError::ValidationError(_)
                ));
            }

            // Test missing version
            #[test]
            fn test_analyze_parameters_validation_missing_version() {
                let params = AnalyzeParameters::new(
                    Definition::from("test_definition"),
                    FileVersion::from(""),
                    Language::from("Rust"),
                    Architecture::from("Microservices"),
                );
                let result = validate(&params);
                assert!(result.is_err());
                assert!(result.as_ref().unwrap_err()
                    .to_string()
                    .contains("File version cannot be empty"));
                assert!(matches!(
                    result.unwrap_err(),
                    CoreError::ValidationError(_)))
            }

            // Test missing language
            #[test]
            fn test_analyze_parameters_validation_missing_language() {
                let params = AnalyzeParameters::new(
                    Definition::from("test_definition"),
                    FileVersion::new(),
                    Language::from(""),
                    Architecture::from("Microservices"),
                );
                let result = validate(&params);
                assert!(result.is_err());
                assert!(result.as_ref().unwrap_err()
                    .to_string()
                    .contains("Language cannot be empty"));
                assert!(matches!(
                    result.unwrap_err(),
                    CoreError::ValidationError(_)
                ));
            }

            // Test missing architecture
            #[test]
            fn test_analyze_parameters_validation_missing_architecture() {
                let params = AnalyzeParameters::new(
                    Definition::from("test_definition"),
                    FileVersion::new(),
                    Language::from("Rust"),
                    Architecture::from(""),
                );
                let result = validate(&params);
                assert!(result.is_err());
                assert!(result.as_ref().unwrap_err()
                    .to_string()
                    .contains("Architecture cannot be empty"));
                assert!(matches!(
                    result.unwrap_err(),
                    CoreError::ValidationError(_)
                ));
            }

            // Test using additional prompt but empty
            #[test]
            fn test_analyze_parameters_validation_empty_additional_prompt() {
                let params = AnalyzeParameters::new(
                    Definition::from("test_definition"),
                    FileVersion::new(),
                    Language::from("Rust"),
                    Architecture::from("Microservices"),
                )
                .with_additional_prompt("".to_string());

                let result = validate(&params);
                assert!(result.is_err());
                assert!(result.as_ref().unwrap_err()
                    .to_string()
                    .contains("Additional prompt cannot be empty"));
                assert!(matches!(
                    result.unwrap_err(),
                    CoreError::ValidationError(_)
                ));
            }
        }

        // Test analyze method

        // Test analyze success
        #[test]
        fn test_analyze_success() {
            let mut processor = MockFakeProcessor::new();
            processor
                .expect_analyze()
                .with(
                    eq(AnalyzeParameters::new(
                        Definition::from("test_definition"),
                        FileVersion::new(),
                        Language::from("Rust"),
                        Architecture::from("Microservices"),
                    )),
                )
                .returning(|_| Ok(()));

            let registry_processor = MockFakeRegistryProcessor::new();
            let path_buf_wrapper = MockFakePathBufWrapper::new();

            let fs_operator = MockFakeFsOperator::new();
            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);
            let manager = Manager::new(processor, registry, fs_operator);
            let params = AnalyzeParameters::new(
                Definition::from("test_definition"),
                FileVersion::new(),
                Language::from("Rust"),
                Architecture::from("Microservices"),
            );

            let result = manager.analyze(params);
            assert!(result.is_ok());
        }

        // Test analyze error on validation
        #[test]
        fn test_analyze_error_on_validation() {
            let processor = MockFakeProcessor::new();
            let registry_processor = MockFakeRegistryProcessor::new();
            let path_buf_wrapper = MockFakePathBufWrapper::new();
            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);

            let fs_operator = MockFakeFsOperator::new(); 
            let manager = Manager::new(processor, registry, fs_operator);
            let params = AnalyzeParameters::new(
                Definition::from(""),
                FileVersion::new(),
                Language::from("Rust"),
                Architecture::from("Microservices"),
            );

            let result = manager.analyze(params);
            assert!(result.is_err());
            assert!(result.as_ref().unwrap_err()
                .to_string()
                .contains("Definition cannot be empty"));
            assert!(matches!(
                result.unwrap_err(),
                BusinessError::InvalidParameters(_)
            ));
        }
    }
}
