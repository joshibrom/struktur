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
    pub website: Option<ProfileLink>,
    pub github: Option<ProfileLink>,

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileLink {
    pub title: String,
    pub href: String,
}

impl std::default::Default for Profile {
    fn default() -> Self {
        let education = vec![Education {
            degree: "B.S. in Computer Science".into(),
            school: "University of Example".into(),
            start_date: "2018-08".into(),
            end_date: Some("2022-05".into()),
            gpa: Some(3.85),
            coursework: vec![
                "Distributed Systems".into(),
                "Data Structures & Algorithms".into(),
                "Database Systems".into(),
                "Operating Systems".into(),
                "Reticulating Splines & Cache Invalidation".into(),
            ],
        }];

        let employment = vec![
            Employment {
                title: "Senior Backend Engineer".into(),
                employer: "Acme Corp".into(),
                location: "San Francisco, CA (Remote)".into(),
                start_date: "2024-01".into(),
                end_date: None,
                bullets: vec![
                    "Architected event-driven microservices processing 50M+ events daily with 99.99% availability.".into(),
                    "Led migration to Rust-based microservices, reducing p99 latency and spline reticulation overhead by 45%.".into(),
                    "Successfully centered a <div> without crashing production or consulting Stack Overflow.".into(),
                ],
            },
            Employment {
                title: "Software Engineer".into(),
                employer: "Beta Technologies".into(),
                location: "Austin, TX".into(),
                start_date: "2022-06".into(),
                end_date: Some("2023-12".into()),
                bullets: vec![
                    "Designed and maintained RESTful APIs and background worker pipelines in Rust and Python.".into(),
                    "Optimized PostgreSQL query performance, eliminating critical bottlenecks in reporting pipelines.".into(),
                    "Eliminated off-by-one errors across the entire codebase (on 2 out of 3 occasions).".into(),
                ],
            },
        ];

        let professional_service = vec![ProfessionalService {
            title: "Open Source Contributor & Maintainer".into(),
            organization: "Rust Community Projects".into(),
            start_date: "2023-01".into(),
            end_date: None,
            bullets: vec![
                "Maintain core open source crates with over 100k downloads, reviewing PRs and triaging issues.".into(),
                "Vigorously debated tabs vs. spaces in RFC discussions, ultimately settling on 4-space indentations.".into(),
            ],
        }];

        let projects = vec![Project {
            title: "Struktur CLI".into(),
            category: "Developer Tools".into(),
            stack: vec!["Rust".into(), "Clap".into(), "Serde".into(), "TOML".into()],
            date: Some("2026".into()),
            bullets: vec![
                "Developed an extensible CLI tool for structured resume and document generation.".into(),
                "Implemented schema validation and configuration management conforming to XDG standards.".into(),
                "Automated the reticulation of splines across build pipelines to maximize developer happiness.".into(),
            ],
        }];

        Self {
            name: "Jane Doe".into(),
            email: "jane.doe@example.com".into(),
            phone: "+1 (555) 019-2834".into(),
            location: "San Francisco, CA".into(),
            website: Some(ProfileLink {
                title: "janedoe.dev".into(),
                href: "https://janedoe.dev".into(),
            }),
            github: Some(ProfileLink {
                title: "janedoe".into(),
                href: "https://github.com/janedoe".into(),
            }),

            education,
            employment,
            professional_service,
            projects,
        }
    }
}

#[derive(Debug)]
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
