use crate::core::types::{validate, PathBufWrapper};

use crate::core::registry::manager::Manager as RegistryManager;
use crate::core::registry::types::{FileVersion, Processor as RegistryProcessor};

use crate::core::business::types::{BusinessError, Definition, Processor};

#[derive(Debug, Clone)]
pub(crate) struct App<P, RP, PW>
where
    P: Processor,
    RP: RegistryProcessor,
    PW: PathBufWrapper,
{
    processor: P,
    registry: RegistryManager<RP, PW>,
}

impl<P, RP, PW> App<P, RP, PW>
where
    P: Processor,
    RP: RegistryProcessor,
    PW: PathBufWrapper,
{
    pub(crate) fn new(processor: P, registry: RegistryManager<RP, PW>) -> Self {
        App {
            processor,
            registry,
        }
    }

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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use mockall::{mock, predicate::eq};

    use crate::core::business::types::{Definition, Processor};
    use crate::core::registry::types::{Registry, RegistryError};

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

            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);
            let manager = App::new(processor, registry);
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

            let registry = RegistryManager::new(registry_processor, path_buf_wrapper);
            let manager = App::new(processor, registry);
            let result = manager.define(
                Definition::from("test_file"),
                Some(FileVersion::from("1.0.0")),
            );
            assert!(result.is_ok())
        }
    }
}
