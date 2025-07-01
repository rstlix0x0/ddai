use std::io::Error;
use std::path::PathBuf;

use crate::core::registry::types::{
    Directory, FileItem, FileName, FileVersion, Processor, Registry, RegistryError,
    REGISTRY_FILE_NAME,
};

use crate::core::types::{validate, PathBufWrapper};

pub(crate) struct Manager<T, P>
where
    T: Processor,
    P: PathBufWrapper,
{
    processor: T,
    path_buf_wrapper: P,
}

impl<T, P> Manager<T, P>
where
    T: Processor,
    P: PathBufWrapper,
{
    /// `new` is a constructor for the `Manager` struct.
    ///
    /// It initializes a new instance of `Manager` with the provided processor and path buffer wrapper.
    /// Specifically for the [`PathBufWrapper`] trait, which provides a way to work with `PathBuf` in a more abstract manner.
    ///
    /// The wrapper must contain the output directory path where the registry file will be created or updated.
    #[allow(dead_code)]
    pub(crate) fn new(processor: T, path_buf_wrapper: P) -> Self {
        Manager {
            processor,
            path_buf_wrapper,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_file(&self, file: FileName) -> Result<Option<FileItem>, RegistryError> {
        let registry_file_path = self._build_registry_file_path()?;
        let registry = self.processor.parse(registry_file_path)?;
        let file_item = registry
            .get_file(&file)
            .and_then(|val| Some(val.to_owned()));

        Ok(file_item)
    }

    /// `build_registry` used to create a new registry file in the specified output directory.
    /// It takes a file name as input, constructs a [`FileItem`] from it
    ///
    /// This method initializes a new [`Registry`] with the directory name derived from the output directory,
    /// adds the file item to the registry, and then calls the processor's `build` method
    ///
    /// This method should be used when you want to create a registry file for the first time
    #[allow(dead_code)]
    pub(crate) fn build_registry(&self, file: FileName) -> Result<(), RegistryError> {
        if !self.path_buf_wrapper.exists() {
            return Err(RegistryError::FsError(Error::new(
                std::io::ErrorKind::NotFound,
                "Output directory is missing or invalid",
            )));
        }

        let registry_file_path = self._build_registry_file_path()?;
        if registry_file_path.exists() {
            return Err(RegistryError::FsError(Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Registry file already exists",
            )));
        }

        let dir_name = self
            .path_buf_wrapper
            .dir_name()
            .ok_or(RegistryError::FsError(Error::new(
                std::io::ErrorKind::NotFound,
                "Output directory is missing or invalid",
            )))?;

        let file_item = FileItem::new(file);
        let _ = validate(&file_item).map_err(|e| RegistryError::CoreError(e))?;

        let mut registry = Registry::new(Directory::from(dir_name));
        registry.add_file(file_item);

        self.processor.build(registry_file_path, registry)
    }

    /// `update_registry` is used to update an existing registry file with a new file version
    ///
    /// It takes a file name and a version string as input, validates the given version
    /// and updates the corresponding file item in the registry
    ///
    /// This method should be used when you want to update the version of an existing file in the registry
    /// When updating the registry, it should not add a new file item if it already exists
    #[allow(dead_code)]
    pub(crate) fn update_registry(
        &self,
        file: FileName,
        version: FileVersion,
    ) -> Result<(), RegistryError> {
        let registry_file_path = self._build_registry_file_path()?;
        if !registry_file_path.exists() {
            return self.build_registry(file);
        }

        let _ = validate(&version).map_err(|e| RegistryError::CoreError(e))?;

        let mut registry = self.processor.parse(registry_file_path.clone())?;
        let mut file_item = registry
            .get_file(&FileName::from(file.clone()))
            .and_then(|val| Some(val.to_owned()));

        let file_item_mut = file_item.as_mut();
        match file_item_mut {
            Some(fi) => {
                fi.update(version);
                registry.add_file(fi.to_owned());
            }
            None => {
                registry.add_file(FileItem::new(file));
            }
        }

        self.processor.build(registry_file_path, registry)
    }

    fn _build_registry_file_path(&self) -> Result<PathBuf, RegistryError> {
        let file_path = self.path_buf_wrapper.to_path_buf().join(REGISTRY_FILE_NAME);

        Ok(file_path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use mockall::{mock, predicate::*};

    mock!(
        FakeProcessor{}

        impl Processor for FakeProcessor {
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

    mod test_build_registry {
        use super::*;
        use std::fs::File;
        use tempfile::Builder;

        #[test]
        fn test_build_registry() {
            let expected_file_path = PathBuf::from("/tmp/output/registry.json");
            let mut expected_registry = Registry::new(Directory::from("output"));
            expected_registry.add_file(FileItem::new(FileName::from("test_file")));

            let mut processor = MockFakeProcessor::new();
            processor
                .expect_build()
                .with(eq(expected_file_path), eq(expected_registry))
                .returning(|_, _| Ok(()));

            let mut path_buf_wrapper = MockFakePathBufWrapper::new();
            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper
                .expect_dir_name()
                .returning(|| Some("output".to_string()));

            path_buf_wrapper.expect_exists().returning(|| true);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result = manager.build_registry(FileName::from("test_file"));

            assert!(result.is_ok());
        }

        #[test]
        fn test_build_registry_file_exists() {
            let temp_dir_object = Builder::new().prefix("output").tempdir().unwrap();

            let temp_dir_path_buf = temp_dir_object.path().to_path_buf();
            let temp_file_path_buf = temp_dir_path_buf.join("registry.json");
            let _ = File::create(temp_file_path_buf).unwrap();

            let processor = MockFakeProcessor::new();
            let mut path_buf_wrapper = MockFakePathBufWrapper::new();

            let cloned_temp_dir_path_buf = temp_dir_path_buf.clone();
            path_buf_wrapper
                .expect_to_path_buf()
                .returning(move || cloned_temp_dir_path_buf.clone());

            let temp_dir = temp_dir_path_buf
                .file_name()
                .and_then(|file_name| file_name.to_str().map(|s| s.to_string()));

            path_buf_wrapper
                .expect_dir_name()
                .returning(move || temp_dir.clone());

            path_buf_wrapper.expect_exists().returning(|| true);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result = manager.build_registry(FileName::from("test_file"));

            assert!(result.is_err());
            assert!(matches!(result, Err(RegistryError::FsError(_))));

            match result {
                Err(RegistryError::FsError(err)) => {
                    assert_eq!(err.kind(), std::io::ErrorKind::AlreadyExists);
                    assert_eq!(err.to_string(), "Registry file already exists");
                }
                _ => panic!("Expected FsError"),
            }
        }

        #[test]
        fn test_build_registry_missing_dir_name() {
            let processor = MockFakeProcessor::new();
            let mut path_buf_wrapper = MockFakePathBufWrapper::new();

            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper.expect_dir_name().returning(|| None);

            path_buf_wrapper.expect_exists().returning(|| false);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result = manager.build_registry(FileName::from("test_file"));

            assert!(result.is_err());
            assert!(matches!(result, Err(RegistryError::FsError(_))));

            match result {
                Err(RegistryError::FsError(err)) => {
                    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
                    assert_eq!(err.to_string(), "Output directory is missing or invalid");
                }
                _ => panic!("Expected FsError"),
            }
        }

        #[test]
        fn test_build_registry_invalid_file_name() {
            let processor = MockFakeProcessor::new();
            let mut path_buf_wrapper = MockFakePathBufWrapper::new();

            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper
                .expect_dir_name()
                .returning(|| Some("output".to_string()));

            path_buf_wrapper.expect_exists().returning(|| true);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result = manager.build_registry(FileName::from("")); // Empty file name
            assert!(result.is_err());
            assert!(matches!(result, Err(RegistryError::CoreError(_))));

            match result {
                Err(RegistryError::CoreError(err)) => {
                    assert_eq!(
                        err.to_string(),
                        "[core error] validation error: File name cannot be empty"
                    );
                }
                _ => panic!("Expected CoreError"),
            }
        }
    }

    mod test_update_registry {
        use super::*;
        use std::fs::File;

        #[test]
        fn test_update_registry() {
            let temp_dir_object = tempfile::Builder::new().prefix("output").tempdir().unwrap();

            let temp_dir_path_buf = temp_dir_object.path().to_path_buf();
            let temp_file_path_buf = temp_dir_path_buf.join("registry.json");
            let temp_dir_name = temp_dir_path_buf
                .file_name()
                .and_then(|file_name| file_name.to_str().map(|s| s.to_string()));

            let cloned_temp_dir_name = temp_dir_name.clone().unwrap();
            let _ = File::create(temp_file_path_buf.clone()).unwrap();

            let expected_file_path = temp_file_path_buf.clone();
            let mut registry = Registry::new(Directory::from(cloned_temp_dir_name));

            let file_name = FileName::from("test_file");
            let file_version = FileVersion::from("1.0.0");
            let mut file_item = FileItem::new(file_name.clone());
            file_item.update(file_version.clone());
            registry.add_file(file_item);

            let cloned_registry = registry.clone();
            let mut processor = MockFakeProcessor::new();
            processor
                .expect_parse()
                .with(eq(expected_file_path.clone()))
                .returning(move |_| Ok(registry.clone()));

            processor
                .expect_build()
                .with(eq(expected_file_path), eq(cloned_registry))
                .returning(|_, _| Ok(()));

            let mut path_buf_wrapper = MockFakePathBufWrapper::new();
            path_buf_wrapper
                .expect_to_path_buf()
                .returning(move || temp_dir_path_buf.clone());

            path_buf_wrapper
                .expect_dir_name()
                .returning(move || temp_dir_name.clone());

            path_buf_wrapper.expect_exists().returning(|| true);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result =
                manager.update_registry(FileName::from("test_file"), FileVersion::from("1.0.0"));
            assert!(result.is_ok());
        }

        #[test]
        fn test_update_registry_file_not_exist() {
            let expected_file_path = PathBuf::from("/tmp/output/registry.json");
            let mut expected_registry = Registry::new(Directory::from("output"));
            expected_registry.add_file(FileItem::new(FileName::from("test_file")));

            let mut processor = MockFakeProcessor::new();
            processor
                .expect_build()
                .with(eq(expected_file_path), eq(expected_registry))
                .returning(|_, _| Ok(()));

            let mut path_buf_wrapper = MockFakePathBufWrapper::new();

            path_buf_wrapper
                .expect_to_path_buf()
                .returning(|| PathBuf::from("/tmp/output"));

            path_buf_wrapper
                .expect_dir_name()
                .returning(|| Some("output".to_string()));

            path_buf_wrapper.expect_exists().returning(|| true);

            let manager = Manager::new(processor, path_buf_wrapper);
            let result =
                manager.update_registry(FileName::from("test_file"), FileVersion::from("1.0.0"));
            assert!(result.is_ok());
        }

        mod test_update_registry_validation {
            use super::*;

            #[test]
            fn test_update_registry_invalid_version() {
                let temp_dir_object = tempfile::Builder::new().prefix("output").tempdir().unwrap();

                let temp_dir_path_buf = temp_dir_object.path().to_path_buf();
                let temp_file_path_buf = temp_dir_path_buf.join("registry.json");
                let temp_dir_name = temp_dir_path_buf
                    .file_name()
                    .and_then(|file_name| file_name.to_str().map(|s| s.to_string()));

                let _ = File::create(temp_file_path_buf.clone()).unwrap();

                let processor = MockFakeProcessor::new();
                let mut path_buf_wrapper = MockFakePathBufWrapper::new();

                path_buf_wrapper
                    .expect_to_path_buf()
                    .returning(move || temp_dir_path_buf.clone());

                path_buf_wrapper
                    .expect_dir_name()
                    .returning(move || temp_dir_name.clone());

                path_buf_wrapper.expect_exists().returning(|| true);

                let manager = Manager::new(processor, path_buf_wrapper);
                let result = manager.update_registry(
                    FileName::from("test_file"),
                    FileVersion::from("invalid_version"),
                );

                assert!(result.is_err());
                assert!(matches!(result, Err(RegistryError::CoreError(_))));

                match result {
                    Err(RegistryError::CoreError(err)) => {
                        assert_eq!(
                            err.to_string(),
                            "[core error] validation error: File version can only contain digit characters & dots"
                        );
                    }
                    _ => panic!("Expected CoreError"),
                }
            }
        }
    }
}
