use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub(crate) enum DocumentError {
    #[error("[doc] document not found at path: {0}")]
    NotFound(String),

    #[error("[doc] failed to read document: {0}")]
    ReadError(String),

    #[error("[doc] failed to write a document: {0}")]
    WriteError(String),
}

#[allow(dead_code)]
pub(crate) struct FilePath(String);

impl FilePath {
    #[allow(dead_code)]
    pub(crate) fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }
}

impl From<String> for FilePath {
    fn from(path: String) -> Self {
        FilePath(path)
    }
}

impl From<&str> for FilePath {
    fn from(path: &str) -> Self {
        FilePath(path.to_string())
    }
}

pub(crate) type FileReader = BufReader<File>;
pub(crate) type FileWriter = BufWriter<File>;

#[allow(dead_code)]
/// FsProcessor trait is an abstraction of a document processor,
/// specifically to handle filesystem operations and document processing tasks.
pub(crate) trait FsProcessor {
    /// Reads a document from the specified path.
    ///
    /// The read operator should not return diretyly the document,
    /// but a buffered reader that can be used to read the document in chunks.
    fn read(&self, path: FilePath) -> Result<FileReader, DocumentError>;

    /// Write a document to the specified path.
    fn write(&self, path: FilePath, content: FileWriter) -> Result<(), DocumentError>;
}
