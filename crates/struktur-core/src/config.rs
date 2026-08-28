//! User configuration models, presets, archetypes, bullet points, and referential validation.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::storage::{
    document::{Document, DocumentError},
    get_project_dirs,
};

/// Unique identifier for presets, archetypes, and bullet points.
pub type ConfigIdT = String;

/// The root user configuration containing role presets and reusable bullet points.
///
/// `UserConfig` is indexed in memory using [`HashMap`] for $O(1)$ lookups by ID.
/// During deserialization, it enforces referential integrity to ensure that every bullet ID
/// referenced in [`Preset::default_bullets`] exists in [`UserConfig::bullets`].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "RawUserConfig", into = "RawUserConfig")]
pub struct UserConfig {
    /// Map of preset ID to [`Preset`] definition.
    pub presets: HashMap<ConfigIdT, Preset>,
    /// Map of bullet ID to [`Bullet`] definition.
    pub bullets: HashMap<ConfigIdT, Bullet>,
}

/// A tailored preset configuration for a specific role or application context.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preset {
    /// Unique identifier for this preset (e.g., `"backend"`, `"frontend"`).
    pub id: ConfigIdT,
    /// Human-readable title of the target role (e.g., `"Backend Engineer"`).
    pub title: String,
    /// Description explaining the focus and intent of this preset.
    pub description: String,
    /// Stylistic or tonal direction for generated content (e.g., `"Direct, technical"`).
    pub default_tone: String,
    /// Introductory paragraph or hook template, supporting `{{.Role}}` and `{{.Company}}` placeholders.
    pub opening_hook: String,
    /// Concluding paragraph or call-to-action template.
    pub closing_hook: String,
    /// List of [`Bullet`] IDs to include by default when this preset is active.
    pub default_bullets: Vec<ConfigIdT>,
    /// Additional persona modifiers or archetypes associated with this preset.
    pub additional_archetypes: Vec<Archetype>,
}

/// An archetype modifier that shapes the emphasis and persona of generated content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Archetype {
    /// Unique identifier for this archetype (e.g., `"startup_generalist"`).
    pub id: ConfigIdT,
    /// Human-readable title of the archetype (e.g., `"Startup Generalist"`).
    pub title: String,
    /// Guiding prompt or instructions describing the persona to emphasize.
    pub prompt: String,
}

/// A reusable bullet point highlighting a specific achievement, project, or skill.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bullet {
    /// Unique identifier for this bullet (e.g., `"high_throughput_apis"`).
    pub id: ConfigIdT,
    /// Brief descriptive label for the bullet point.
    pub title: String,
    /// Categorical tags for filtering and grouping (e.g., `["backend", "api"]`).
    pub tags: Vec<String>,
    /// The formatted text describing the accomplishment or capability.
    pub text: String,
}

impl std::default::Default for UserConfig {
    fn default() -> Self {
        let bullets = vec![
            Bullet {
                id: "high_throughput_apis".into(),
                title: "High Throughput API Design".into(),
                tags: vec!["backend".into(), "api".into(), "performance".into()],
                text: "Designed and implemented high-throughput REST and gRPC services capable of handling millions of daily requests with low latency.".into(),
            },
            Bullet {
                id: "database_optimization".into(),
                title: "Database Optimization & Schema Design".into(),
                tags: vec!["backend".into(), "database".into(), "sql".into()],
                text: "Optimized relational database schemas, indexes, and queries to reduce latency and ensure ACID compliance at scale.".into(),
            },
        ];

        let presets = vec![Preset {
            id: "backend".into(),
            title: "Backend Engineer".into(),
            description: "Focus on API design, high throughput, and database optimization.".into(),
            default_tone: "Direct, technical, emphasizing concurrency, latency reduction, and data integrity.".into(),
            opening_hook: "I am interested in the {{.Role}} position at {{.Company}}. With a strong background building high-throughput backend services and concurrent data processing pipelines, I focus on delivering scalable, reliable systems.".into(),
            closing_hook: "I look forward to discussing how my experience in backend architecture and pipeline optimization can support engineering initiatives at {{.Company}}.".into(),
            default_bullets: vec![
                "high_throughput_apis".into(),
                "database_optimization".into(),
            ],
            additional_archetypes: vec![Archetype {
                id: "startup_generalist".into(),
                title: "Startup Generalist".into(),
                prompt: "Emphasize 0-to-1 velocity, building pragmatic solutions across multiple stacks, proactive problem solving, and ownership.".into(),
            }],
        }];

        RawUserConfig { bullets, presets }
            .try_into()
            .expect("default user config must be valid")
    }
}

/// Raw on-disk TOML representation of user configuration using sequences of tables.
#[derive(Serialize, Deserialize)]
struct RawUserConfig {
    pub presets: Vec<Preset>,
    pub bullets: Vec<Bullet>,
}

impl std::default::Default for RawUserConfig {
    fn default() -> Self {
        UserConfig::default().into()
    }
}

impl Document for UserConfig {
    fn file_name() -> &'static str {
        "config.toml"
    }

    fn get_path() -> Result<std::path::PathBuf, DocumentError> {
        let path = get_project_dirs()
            .ok_or(DocumentError::ConfigPathNotFound)?
            .config_dir()
            .to_owned();
        Ok(path.join(Self::file_name()))
    }
}

impl TryFrom<RawUserConfig> for UserConfig {
    type Error = UserConfigError;

    fn try_from(value: RawUserConfig) -> Result<Self, Self::Error> {
        let bullets = value
            .bullets
            .iter()
            .map(|bullet| (bullet.id.clone(), bullet.clone()))
            .collect::<HashMap<_, _>>();
        let presets = value
            .presets
            .into_iter()
            .map(|preset| (preset.id.clone(), preset))
            .collect::<HashMap<_, _>>();

        // Ensure that all referenced bullets exist
        for (preset_id, preset) in presets.iter() {
            for bullet_id in &preset.default_bullets {
                if !bullets.contains_key(bullet_id) {
                    return Err(UserConfigError::MissingReferencedBullet {
                        preset_id: preset_id.clone(),
                        bullet_id: bullet_id.clone(),
                    });
                }
            }
        }

        Ok(Self { presets, bullets })
    }
}

impl From<UserConfig> for RawUserConfig {
    fn from(value: UserConfig) -> Self {
        RawUserConfig {
            presets: value.presets.into_values().collect(),
            bullets: value.bullets.into_values().collect(),
        }
    }
}

/// Errors that can occur during semantic validation of a [`UserConfig`].
#[derive(Debug)]
pub enum UserConfigError {
    /// A preset referenced a bullet point ID in `default_bullets` that was not defined in `bullets`.
    MissingReferencedBullet {
        /// The ID of the preset containing the invalid reference.
        preset_id: ConfigIdT,
        /// The bullet ID that could not be found.
        bullet_id: ConfigIdT,
    },
}

impl std::error::Error for UserConfigError {}

impl std::fmt::Display for UserConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingReferencedBullet {
                preset_id,
                bullet_id,
            } => write!(
                f,
                "MissingReferencedBullet: Preset '{}' references Bullet with ID '{}' but it is not defined.",
                preset_id, bullet_id
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validity() {
        let raw = RawUserConfig::default();
        let config_res = UserConfig::try_from(raw);
        assert!(config_res.is_ok());

        let config = config_res.unwrap();
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.bullets.len(), 2);
        assert!(config.presets.contains_key("backend"));
    }

    #[test]
    fn test_missing_referenced_bullet_error() {
        let raw = RawUserConfig {
            presets: vec![Preset {
                id: "test".into(),
                title: "Test".into(),
                description: "Test".into(),
                default_tone: "Test".into(),
                opening_hook: "Test".into(),
                closing_hook: "Test".into(),
                default_bullets: vec!["non_existent_bullet".into()],
                additional_archetypes: vec![],
            }],
            bullets: vec![],
        };

        let result = UserConfig::try_from(raw);
        assert!(matches!(
            result,
            Err(UserConfigError::MissingReferencedBullet { .. })
        ));
    }

    #[test]
    fn test_default_toml_roundtrip() {
        let raw = RawUserConfig::default();
        let serialized = toml::to_string_pretty(&raw).expect("serialization should succeed");
        let deserialized: RawUserConfig =
            toml::from_str(&serialized).expect("deserialization should succeed");
        let config: UserConfig = deserialized
            .try_into()
            .expect("conversion to UserConfig should succeed");
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.bullets.len(), 2);
    }
}
