use std::{
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub fn load(file_path: &Path) -> Result<Profile, ProfileError> {
    let file = std::fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut raw_content = String::new();
    reader.read_to_string(&mut raw_content)?;

    let profile = toml::from_str(&raw_content)?;
    Ok(profile)
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub website: Option<String>,
    pub github: Option<String>,

    pub education: Vec<Education>,
    pub employment: Vec<Employment>,
    pub professional_service: Vec<ProfessionalService>,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub school: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub gpa: Option<f32>,
    pub coursework: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Employment {
    pub title: String,
    pub employer: String,
    pub location: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub bullets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfessionalService {
    pub title: String,
    pub organization: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub bullets: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub category: String,
    pub stack: Vec<String>,
    pub date: Option<String>,
    pub bullets: Vec<String>,
}

pub enum ProfileError {
    OtherDeserializationError(toml::de::Error),
    IOError(std::io::Error),
}

impl From<toml::de::Error> for ProfileError {
    fn from(value: toml::de::Error) -> Self {
        Self::OtherDeserializationError(value)
    }
}

impl From<std::io::Error> for ProfileError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}
