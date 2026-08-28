//! User profile models representing personal information, education, work experience, service, and projects.

use serde::{Deserialize, Serialize};

use crate::storage::{
    document::{Document, DocumentError},
    get_project_dirs,
};

/// Complete personal and professional profile of the user.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Profile {
    /// Full name of the user.
    pub name: String,
    /// Primary contact email address.
    pub email: String,
    /// Contact telephone number.
    pub phone: String,
    /// Location or residence (e.g., `"San Francisco, CA"`).
    pub location: String,
    /// Optional link to a personal website or portfolio.
    pub website: Option<ProfileLink>,
    /// Optional link to a GitHub profile.
    pub github: Option<ProfileLink>,

    /// Chronological list of educational degrees, schools, and coursework.
    pub education: Vec<Education>,
    /// Chronological list of employment and professional work history.
    pub employment: Vec<Employment>,
    /// List of professional service, community, or open-source contributions.
    pub professional_service: Vec<ProfessionalService>,
    /// List of notable personal, academic, or professional projects.
    pub projects: Vec<Project>,
}

/// Academic degree or educational institution record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Education {
    /// Degree or certification earned (e.g., `"B.S. in Computer Science"`).
    pub degree: String,
    /// Institution or university name.
    pub school: String,
    /// Starting date in `YYYY-MM` or `YYYY` format.
    pub start_date: String,
    /// Optional end date or graduation date. If `None`, indicates ongoing studies.
    pub end_date: Option<String>,
    /// Optional Grade Point Average (GPA).
    pub gpa: Option<f32>,
    /// List of relevant course titles or academic concentrations.
    pub coursework: Vec<String>,
}

/// Professional employment or job record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Employment {
    /// Job title or position held (e.g., `"Senior Backend Engineer"`).
    pub title: String,
    /// Company or organization name.
    pub employer: String,
    /// Work location or remote status (e.g., `"Austin, TX"` or `"Remote"`).
    pub location: String,
    /// Starting date in `YYYY-MM` or `YYYY` format.
    pub start_date: String,
    /// Optional end date in `YYYY-MM` or `YYYY` format. If `None`, indicates current employment.
    pub end_date: Option<String>,
    /// Bullet points describing key accomplishments and responsibilities.
    pub bullets: Vec<String>,
}

/// Professional service, open-source maintainership, or community leadership record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfessionalService {
    /// Role or position (e.g., `"Open Source Contributor & Maintainer"`).
    pub title: String,
    /// Organization, foundation, or working group name.
    pub organization: String,
    /// Starting date in `YYYY-MM` or `YYYY` format.
    pub start_date: String,
    /// Optional end date. If `None`, indicates ongoing service.
    pub end_date: Option<String>,
    /// Bullet points describing responsibilities and contributions.
    pub bullets: Vec<String>,
}

/// Notable software, engineering, or portfolio project.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Project {
    /// Name or title of the project.
    pub title: String,
    /// Category or domain of the project (e.g., `"Developer Tools"`).
    pub category: String,
    /// Key technologies, frameworks, or languages used (e.g., `["Rust", "Clap", "TOML"]`).
    pub stack: Vec<String>,
    /// Optional project date or year of release.
    pub date: Option<String>,
    /// Bullet points detailing project features, impact, and architecture.
    pub bullets: Vec<String>,
}

/// A web link with a display title and destination URL.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileLink {
    /// Human-readable link title or handle (e.g., `"janedoe.dev"` or `"janedoe"`).
    pub title: String,
    /// Full web URL (e.g., `"https://janedoe.dev"` or `"https://github.com/janedoe"`).
    pub href: String,
}

impl ProfileLink {
    /// Creates a new instance of a `ProfileLink` by casting the input types
    /// into `String`s.
    pub fn new(title: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            href: href.into(),
        }
    }
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
            website: Some(ProfileLink::new("janedoe.dev", "https://janedoe.dev")),
            github: Some(ProfileLink::new("janedoe", "https://github.com/janedoe")),

            education,
            employment,
            professional_service,
            projects,
        }
    }
}

impl Document for Profile {
    fn file_name() -> &'static str {
        "profile.toml"
    }

    fn get_path() -> Result<std::path::PathBuf, DocumentError> {
        let path = get_project_dirs()
            .ok_or(DocumentError::DataPathNotFound)?
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
