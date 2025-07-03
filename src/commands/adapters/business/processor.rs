use std::fs::{File, create_dir_all};

use crate::core::types::PathBufWrapper;
use crate::core::registry::types::FileVersion;
use crate::core::business::types::{Processor, BusinessError, Definition};

pub(crate) struct ProcessorAdapter<T: PathBufWrapper> {
    pathbuf: T,
}

impl<T> ProcessorAdapter<T> where T: PathBufWrapper {
    pub fn new(pathbuf: T) -> Self {
        ProcessorAdapter { pathbuf }
    }
}

impl<T> Processor for ProcessorAdapter<T> where T: PathBufWrapper {
    fn define(&self, definition: Definition, version: FileVersion) -> Result<(), BusinessError> {
        // first check if the directory exists, if not create it
        // the directory is based on the "Definition" name
        let dir_path = self.pathbuf.to_path_buf().join(definition.as_str());
        if !dir_path.exists() {
            create_dir_all(&dir_path).map_err(|err| BusinessError::FsError(err))?;
        }

        // create a file with the name format is "{version}.md"
        // we only need to create the file, not write to it, it's like using "touch" command
        let file_name = format!("{}.md", version.to_string());
        let file_path = dir_path.join(file_name);
        _ = File::create(&file_path).map_err(|err| BusinessError::FsError(err))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::path::PathBuf;

    mock!(
        FakePathBufWrapper {}

        impl PathBufWrapper for FakePathBufWrapper {
            fn to_path_buf(&self) -> PathBuf;
            fn dir_name(&self) -> Option<String>;
            fn exists(&self) -> bool;
        }
    );

    #[test]
    fn test_define_creates_directory_and_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_pathbuf = temp_dir.path().to_path_buf();
        let temp_dir_pathbuf_cloned = temp_dir_pathbuf.clone();

        let mut pathbuf = MockFakePathBufWrapper::new();
        pathbuf.expect_to_path_buf().returning(move || temp_dir_pathbuf.clone());
        pathbuf.expect_exists().returning(|| true);

        let processor = ProcessorAdapter::new(pathbuf);
        let definition = Definition::from("test_business");
        let version = FileVersion::new();
        let result = processor.define(definition.clone(), version.clone());
        assert!(result.is_ok());

        let dir_path = temp_dir_pathbuf_cloned.join(definition.as_str());
        assert!(dir_path.exists(), "Directory should be created");

        let file_name = format!("{}.md", version.to_string());
        let file_path = dir_path.join(file_name);
        assert!(file_path.exists(), "File should be created");
    }
}