use std::path::{Path, PathBuf};

use directories::ProjectDirs;

pub mod config;
pub mod profile;

#[derive(Debug)]
pub enum XDGError {
    ConfigPathNotFound,
    DataPathNotFound,
    GeneralIOError(std::io::Error),
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

pub fn ensure_project_files() -> Result<(), XDGError> {
    let paths_to_create = [get_config_path()?, get_user_data_path()?]
        .into_iter()
        .map(|p| p.try_exists().map(|exists| (p, exists)))
        .filter_map(Result::ok)
        .filter_map(|(p, exists)| if exists { None } else { Some(p) })
        .collect::<Vec<_>>();

    for p in paths_to_create {
        std::fs::create_dir_all(&p.parent().unwrap_or(Path::new("")))
            .map_err(|err| XDGError::GeneralIOError(err))?;

        // TODO: Append default content to file
        std::fs::File::create_new(&p).map_err(|err| XDGError::GeneralIOError(err))?;
    }

    Ok(())
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
