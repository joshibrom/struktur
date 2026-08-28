//! XDG-compliant storage and document persistence subsystem.
//!
//! This module handles standard configuration and data directory resolution,
//! document persistence via the [`XDGDocument`] trait, and initialization of
//! default project configuration and user profile files.

use directories::ProjectDirs;

use crate::{
    config::UserConfig,
    profile::Profile,
    xdg::document::{XDGDocument, XDGError},
};

pub mod config;
pub mod document;
pub mod profile;

/// Ensures that the default project configuration (`config.toml`) and user profile
/// (`profile.toml`) exist in their respective standard system directories.
///
/// If either file does not already exist on disk, a default instance is created and saved.
///
/// # Errors
///
/// Returns an [`XDGError`] if directory creation or file writing fails, or if the system
/// directories cannot be determined.
pub fn ensure_project_files() -> Result<(), XDGError> {
    if !UserConfig::exists()? {
        let config = UserConfig::default();
        config.save()?;
    }
    if !Profile::exists()? {
        let profile = Profile::default();
        profile.save()?;
    }
    Ok(())
}

/// Resolves standard platform-specific project directories for `dev.joshibrom.struktur`.
///
/// Uses standard XDG Base Directory locations on Linux/Unix, and corresponding standard
/// application directories on macOS and Windows.
fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
