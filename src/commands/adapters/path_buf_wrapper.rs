use std::path::PathBuf;

use crate::core::types::PathBufWrapper;

#[derive(Debug, Clone)]
pub(crate) struct PathBufAdapter {
    pathbuf: PathBuf,
}

impl PathBufAdapter {
    pub fn new(pathbuf: PathBuf) -> Self {
        PathBufAdapter { pathbuf }
    }
}

impl PathBufWrapper for PathBufAdapter {
    fn to_path_buf(&self) -> PathBuf {
        self.pathbuf.clone()
    }

    fn dir_name(&self) -> Option<String> {
        self.pathbuf
            .file_name()
            .and_then(|name| name.to_str().map(String::from))
            .or_else(|| {
                self.pathbuf
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .and_then(|name| name.to_str().map(String::from))
            })
    }

    fn exists(&self) -> bool {
        self.pathbuf.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_buf_adapter() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().to_path_buf();

        let adapter = PathBufAdapter::new(path.clone());

        assert_eq!(adapter.to_path_buf(), path);
        assert_eq!(
            adapter.dir_name(),
            path.file_name()
                .and_then(|name| name.to_str())
                .map(String::from)
        );
        assert!(adapter.exists());
    }
}
