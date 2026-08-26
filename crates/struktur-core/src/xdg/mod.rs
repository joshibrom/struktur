use std::path::PathBuf;

use directories::ProjectDirs;

pub mod config;
pub mod profile;

pub enum XDGError {
    ConfigPathNotFound,
    DataPathNotFound,
}

pub fn get_config_path() -> Result<PathBuf, XDGError> {
    let path = get_project_dirs()
        .ok_or(XDGError::ConfigPathNotFound)?
        .config_dir()
        .to_owned();
    Ok(path.join("config.toml"))
}

pub fn get_user_data_path() -> Result<PathBuf, XDGError> {
    let path = get_project_dirs()
        .ok_or(XDGError::DataPathNotFound)?
        .data_dir()
        .to_owned();
    Ok(path.join("profile.toml"))
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
