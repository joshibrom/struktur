use serde::{Deserialize, Serialize};

use super::{
    document::{XDGDocument, XDGError},
    get_project_dirs,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Education {
    pub degree: String,
    pub school: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub gpa: Option<f32>,
    pub coursework: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Employment {
    pub title: String,
    pub employer: String,
    pub location: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub bullets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfessionalService {
    pub title: String,
    pub organization: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub bullets: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

impl XDGDocument for Profile {
    fn file_name() -> &'static str {
        "profile.toml"
    }

    fn get_path() -> Result<std::path::PathBuf, super::document::XDGError> {
        let path = get_project_dirs()
            .ok_or(XDGError::DataPathNotFound)?
            .data_dir()
            .to_owned();
        Ok(path.join(Self::file_name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile_validity() {
        let profile = Profile::default();
        assert_eq!(profile.name, "Jane Doe");
        assert!(profile.website.is_some());
        assert!(profile.github.is_some());
        assert_eq!(profile.education.len(), 1);
        assert_eq!(profile.employment.len(), 2);
        assert_eq!(profile.professional_service.len(), 1);
        assert_eq!(profile.projects.len(), 1);
    }

    #[test]
    fn test_default_profile_toml_roundtrip() {
        let default_profile = Profile::default();
        let serialized =
            toml::to_string_pretty(&default_profile).expect("serialization should succeed");
        let deserialized: Profile =
            toml::from_str(&serialized).expect("deserialization should succeed");
        assert_eq!(deserialized, default_profile);
    }

    #[test]
    fn test_minimal_profile_deserialization() {
        let raw = r#"
            name = "Minimal User"
            email = "min@example.com"
            phone = "123-456-7890"
            location = "Anytown, USA"
            education = []
            employment = []
            professional_service = []
            projects = []
        "#;

        let profile: Profile = toml::from_str(raw).expect("should deserialize minimal profile");
        assert_eq!(profile.name, "Minimal User");
        assert!(profile.website.is_none());
        assert!(profile.github.is_none());
    }
}
