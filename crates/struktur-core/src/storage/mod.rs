//! Storage and document persistence subsystem.
//!
//! This module handles standard configuration and data directory resolution,
//! document persistence via the [`Document`] trait, and initialization of
//! default project configuration and user profile files.

use directories::ProjectDirs;

use self::document::{Document, DocumentError};
use crate::{
    config::UserConfig,
    profile::Profile,
    template::{
        RenderableTemplate, TemplateError, cv::plaintext::PlaintextCvTemplate,
        plaintext::PlaintextTemplate,
    },
};

pub mod document;

/// Errors that can occur during storage subsystem initialization.
#[derive(thiserror::Error, Debug)]
pub enum StorageInitError {
    /// An error occurred while initializing or saving a project document (`config.toml` or `profile.toml`).
    #[error("DocumentError: {0}")]
    DocumentError(#[from] DocumentError),

    /// An error occurred while creating or writing default template files.
    #[error("TemplateError: {0}")]
    TemplateError(#[from] TemplateError),
}

/// Ensures that the default project configuration (`config.toml`), user profile
/// (`profile.toml`), and template files exist in their respective standard system directories.
///
/// If any required file does not already exist on disk, a default instance is created and saved.
///
/// # Errors
///
/// Returns a [`StorageInitError`] if directory creation or file writing fails, or if the system
/// directories cannot be determined.
pub fn init_storage() -> Result<(), StorageInitError> {
    if !UserConfig::exists()? {
        let config = UserConfig::default();
        config.save()?;
    }
    if !Profile::exists()? {
        let profile = Profile::default();
        profile.save()?;
    }
    if !PlaintextTemplate::exists()? {
        PlaintextTemplate::write_default_template()?;
    }
    if !PlaintextCvTemplate::exists()? {
        PlaintextCvTemplate::write_default_template()?;
    }
    Ok(())
}

/// Resolves standard platform-specific project directories for `dev.joshibrom.struktur`.
///
/// Uses standard XDG Base Directory locations on Linux/Unix, and corresponding standard
/// application directories on macOS and Windows.
pub fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
