use directories::ProjectDirs;

use crate::{
    config::UserConfig,
    profile::Profile,
    xdg::document::{XDGDocument, XDGError},
};

pub mod config;
pub mod document;
pub mod profile;

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

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
