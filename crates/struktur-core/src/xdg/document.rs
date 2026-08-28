use std::path::PathBuf;

use serde::{Serialize, de::DeserializeOwned};

pub trait XDGDocument: Serialize + DeserializeOwned + Default {
    fn file_name() -> &'static str;
    fn get_path() -> Result<PathBuf, XDGError>;

    fn exists() -> Result<bool, XDGError> {
        let path = Self::get_path()?;
        Ok(path.try_exists()?)
    }

    fn load() -> Result<Self, XDGError> {
        let path = Self::get_path()?;
        let content = std::fs::read_to_string(path)?;
        let doc = toml::from_str(&content)?;

        Ok(doc)
    }

    fn save(&self) -> Result<(), XDGError> {
        let path = Self::get_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum XDGError {
    ConfigPathNotFound,
    DataPathNotFound,
    IOError(std::io::Error),
    SerializationError(toml::ser::Error),
    DeserializationError(toml::de::Error),
}

impl std::error::Error for XDGError {}

impl std::fmt::Display for XDGError {
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

impl From<std::io::Error> for XDGError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<toml::ser::Error> for XDGError {
    fn from(value: toml::ser::Error) -> Self {
        Self::SerializationError(value)
    }
}

impl From<toml::de::Error> for XDGError {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value)
    }
}
