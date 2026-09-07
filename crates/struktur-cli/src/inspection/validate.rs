//! Project configuration and template validation inspection.

use std::path::PathBuf;

use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::{Document, DocumentError},
    template::{
        RenderableTemplate, cover_letter::plaintext::PlaintextTemplate,
        cv::plaintext::PlaintextCvTemplate,
    },
};

/// Health and format validation status for a single project file.
pub struct FileValidation {
    name: String,
    path: Option<PathBuf>,
    result: Result<(), FileValidationError>,
}

impl FileValidation {
    /// Returns `true` if the file passed validation.
    pub fn is_valid(&self) -> bool {
        self.result.is_ok()
    }
}

impl std::fmt::Display for FileValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if self.result.is_ok() { "✓" } else { "✗" };
        let path_str = match &self.path {
            Some(p) => p.display().to_string(),
            None => "path unknown".to_string(),
        };
        write!(f, "  {mark}  {:<28}  {path_str}", self.name)?;

        if let Err(err) = &self.result {
            write!(f, "\n     {err}")
        } else {
            Ok(())
        }
    }
}

/// Errors that can occur when validating project files.
pub enum FileValidationError {
    /// The file contains syntax errors or broken references.
    FormatError(String),
    /// The file could not be read or accessed from disk.
    IoError(std::io::Error),
}

impl std::fmt::Display for FileValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatError(err) => write!(f, "Format Error: {err}"),
            Self::IoError(err) => match err.kind() {
                std::io::ErrorKind::NotFound => write!(f, "(missing - run 'struktur init')"),
                _ => write!(f, "Could Not Validate: {err}"),
            },
        }
    }
}

fn validate_document<D: Document>() -> FileValidation {
    let name = D::file_name().to_string();
    let path = D::get_path().ok();

    match D::load() {
        Ok(_) => FileValidation {
            name,
            path,
            result: Ok(()),
        },
        Err(err) => match err {
            DocumentError::DeserializationError(ser_err) => FileValidation {
                name,
                path,
                result: Err(FileValidationError::FormatError(ser_err.to_string())),
            },
            DocumentError::ConfigPathNotFound | DocumentError::DataPathNotFound => FileValidation {
                name,
                path,
                result: Err(FileValidationError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "document not found",
                ))),
            },
            DocumentError::IOError(io_err) => FileValidation {
                name,
                path,
                result: Err(FileValidationError::IoError(io_err)),
            },
            _ => unreachable!("Error does not pertain to read-only operations"),
        },
    }
}

fn validate_template<T: RenderableTemplate>() -> FileValidation {
    let name = format!("{}/{}", T::get_archetype().to_dirname(), T::file_name());
    let path = T::get_path().ok();

    match T::load() {
        Ok(templ) => {
            let mut tera = tera::Tera::default();
            match tera.add_raw_template("validationTest", &templ) {
                Ok(_) => FileValidation {
                    name,
                    path,
                    result: Ok(()),
                },
                Err(err) => FileValidation {
                    name,
                    path,
                    result: Err(FileValidationError::FormatError(err.to_string())),
                },
            }
        }
        Err(io_err) => FileValidation {
            name,
            path,
            result: Err(FileValidationError::IoError(io_err)),
        },
    }
}

/// Performs validation checks across all project configuration, profile, and template files.
pub fn check() -> Vec<FileValidation> {
    vec![
        validate_document::<UserConfig>(),
        validate_document::<Profile>(),
        validate_template::<PlaintextTemplate>(),
        validate_template::<PlaintextCvTemplate>(),
    ]
}
