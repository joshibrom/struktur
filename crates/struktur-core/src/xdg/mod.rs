use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use directories::ProjectDirs;

pub mod config;
pub mod profile;

#[derive(Debug)]
pub enum XDGError {
    ConfigPathNotFound,
    DataPathNotFound,
    GeneralIOError(std::io::Error),
    FormatError(toml::ser::Error),
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
    enum FileType {
        Config,
        UserData,
    }

    let paths_to_create = [
        (get_config_path()?, FileType::Config),
        (get_user_data_path()?, FileType::UserData),
    ]
    .into_iter()
    .map(|p| p.0.try_exists().map(|exists| (p, exists)))
    .filter_map(Result::ok)
    .filter_map(|(p, exists)| if exists { None } else { Some(p) })
    .collect::<Vec<_>>();

    for (p, ft) in paths_to_create {
        std::fs::create_dir_all(&p.parent().unwrap_or(Path::new("")))
            .map_err(|err| XDGError::GeneralIOError(err))?;

        let file = std::fs::File::create_new(&p).map_err(|err| XDGError::GeneralIOError(err))?;
        match ft {
            FileType::Config => {
                let mut writer = BufWriter::new(file);
                let content = toml::to_string_pretty(&config::RawUserConfig::default())
                    .map_err(|err| XDGError::FormatError(err))?;
                writer
                    .write_all(content.as_bytes())
                    .map_err(|err| XDGError::GeneralIOError(err))?;
            }
            FileType::UserData => {}
        };
    }

    Ok(())
}

fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "joshibrom", "struktur")
}
