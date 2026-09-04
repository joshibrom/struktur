//! Template rendering and variable interpolation subsystem.
//!
//! This module handles context resolution, pre-rendering of role/company variables
//! within preset hooks, and compiling documents via the [`RenderableTemplate`] trait.

use std::io::Write;

use serde::Serialize;
use tera::Tera;

pub mod cover_letter;
pub mod cv;

/// Errors that can occur during template loading, parsing, serialization, or rendering.
#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    /// An I/O error occurred while reading or writing a template file on disk.
    #[error("Failed to load template: {0}")]
    TemplateIoError(#[from] std::io::Error),

    /// A Tera syntax error occurred while registering or parsing the template.
    #[error("Failed to parse template: {0}")]
    TemplateAddError(tera::Error),

    /// A Tera execution error occurred while rendering the template.
    #[error("Failed to render template: {0}")]
    TemplateRenderError(tera::Error),

    /// Serialization of the template context to a Tera value failed.
    #[error("Failed to serialize template context: {0}")]
    TemplateContextSerializationError(tera::Error),
}

/// High-level document category defining template directory structure and semantic role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateArchetype {
    /// Curriculum Vitae or comprehensive candidate resume.
    Cv,
    /// Tailored job application cover letter.
    CoverLetter,
}

impl TemplateArchetype {
    /// Returns the filesystem directory name associated with this archetype.
    fn to_dirname(self) -> &'static str {
        match self {
            Self::Cv => "cv",
            Self::CoverLetter => "cover-letter",
        }
    }
}

/// A document template that can be rendered using a serializable context.
///
/// Implementations define a file name, document archetype, and an embedded default fallback template.
/// When rendering, the implementation first checks for a user-customized template
/// file on disk (in `~/.config/struktur/templates/<archetype>/<file_name>`), falling back to
/// the embedded default if no custom template exists.
pub trait RenderableTemplate {
    /// The input context type serialized and passed into the Tera template engine.
    type BaseContext: Serialize;

    /// The template file name on disk (e.g. `plaintext.tera`).
    fn file_name() -> &'static str;

    /// The archetype category that groups this template on disk.
    fn get_archetype() -> TemplateArchetype;

    /// The embedded default template used when no user file exists on disk.
    fn get_default_template() -> &'static str;

    /// Resolves the filesystem path to the template in the user configuration directory.
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if the configuration directory cannot be determined.
    fn get_path() -> Result<std::path::PathBuf, std::io::Error> {
        let path = crate::storage::get_project_dirs()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find project directories",
                )
            })?
            .config_dir()
            .to_owned();
        Ok(path
            .join("templates")
            .join(Self::get_archetype().to_dirname())
            .join(Self::file_name()))
    }

    /// Checks whether the custom template file exists on disk.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if path resolution fails.
    fn exists() -> Result<bool, TemplateError> {
        Ok(Self::get_path()?.exists())
    }

    /// Loads the custom template string from disk.
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if the file cannot be read.
    fn load() -> Result<String, std::io::Error> {
        let path = Self::get_path()?;
        std::fs::read_to_string(path)
    }

    /// Retrieves the template string to use, preferring a custom file on disk
    /// and falling back to [`Self::get_default_template()`].
    fn get_template() -> String {
        match Self::load() {
            Ok(templ) => templ,
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Warning: Failed to load custom template: {err}");
                }
                Self::get_default_template().to_string()
            }
        }
    }

    /// Renders the template using the provided context.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if template registration, context serialization,
    /// or rendering fails.
    fn render(context: &Self::BaseContext) -> Result<String, TemplateError> {
        let template = Self::get_template();

        let mut tera = Tera::new();
        tera.autoescape_on(Vec::<&str>::new());
        tera.add_raw_template("main", &template)
            .map_err(TemplateError::TemplateAddError)?;

        let tera_ctx = tera::Context::from_serialize(context)
            .map_err(TemplateError::TemplateContextSerializationError)?;

        let rendering = tera
            .render("main", &tera_ctx)
            .map_err(TemplateError::TemplateRenderError)?;

        Ok(rendering.trim().to_string())
    }

    /// Writes the embedded default template to disk if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if parent directory creation or file writing fails.
    fn write_default_template() -> Result<(), TemplateError> {
        let path = Self::get_path()?;
        if Self::exists()? {
            return Ok(());
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::File::create(path)?;
        file.write_all(Self::get_default_template().as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_archetype_to_dirname() {
        assert_eq!(TemplateArchetype::Cv.to_dirname(), "cv");
        assert_eq!(TemplateArchetype::CoverLetter.to_dirname(), "cover-letter");
    }
}
