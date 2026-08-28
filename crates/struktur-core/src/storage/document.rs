//! Document persistence trait and error types for configuration and data files.

use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize};

/// A document that can be persisted to and loaded from a standard system path.
///
/// Types implementing `Document` must be serializable and deserializable via Serde,
/// and provide a default fallback representation.
pub trait Document: Serialize + DeserializeOwned + Default {
    /// Returns the standard filename for this document (e.g., `"config.toml"` or `"profile.toml"`).
    fn file_name() -> &'static str;

    /// Resolves the absolute filesystem path where this document should be stored.
    ///
    /// # Errors
    ///
    /// Returns a [`DocumentError`] if the appropriate system directory cannot be located.
    fn get_path() -> Result<PathBuf, DocumentError>;

    /// Checks whether the document file currently exists on the filesystem.
    ///
    /// # Errors
    ///
    /// Returns a [`DocumentError`] if path resolution fails or if an I/O error occurs while checking existence.
    fn exists() -> Result<bool, DocumentError> {
        let path = Self::get_path()?;
        Ok(path.try_exists()?)
    }

    /// Loads and deserializes the document from its standard path on disk.
    ///
    /// # Errors
    ///
    /// Returns a [`DocumentError::IOError`] if the file cannot be read, or a [`DocumentError::DeserializationError`]
    /// if the file content cannot be parsed into `Self`.
    fn load() -> Result<Self, DocumentError> {
        let path = Self::get_path()?;
        let content = std::fs::read_to_string(path)?;
        let doc = toml::from_str(&content)?;

        Ok(doc)
    }

    /// Serializes and writes the document to its standard path on disk.
    ///
    /// Parent directories are automatically created if they do not already exist.
    ///
    /// # Errors
    ///
    /// Returns a [`DocumentError::SerializationError`] if serialization fails, or a [`DocumentError::IOError`]
    /// if directory creation or file writing fails.
    fn save(&self) -> Result<(), DocumentError> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        Ok(())
    }
}

/// Errors that can occur during storage path resolution, serialization, deserialization, or file I/O.
#[derive(Debug)]
pub enum DocumentError {
    /// The user configuration directory could not be determined on the current system.
    ConfigPathNotFound,
    /// The user data directory could not be determined on the current system.
    DataPathNotFound,
    /// An underlying filesystem I/O error occurred.
    IOError(std::io::Error),
    /// TOML serialization failed.
    SerializationError(toml::ser::Error),
    /// TOML deserialization or schema validation failed.
    DeserializationError(toml::de::Error),
}

impl std::error::Error for DocumentError {}

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigPathNotFound => write!(f, "ConfigPathNotFound"),
            Self::DataPathNotFound => write!(f, "DataPathNotFound"),
            Self::IOError(err) => write!(f, "IOError: {}", err),
            Self::SerializationError(err) => write!(f, "SerializationError: {}", err),
            Self::DeserializationError(err) => write!(f, "DeserializationError: {}", err),
        }
    }
}

impl From<std::io::Error> for DocumentError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<toml::ser::Error> for DocumentError {
    fn from(value: toml::ser::Error) -> Self {
        Self::SerializationError(value)
    }
}

impl From<toml::de::Error> for DocumentError {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value)
    }
}
