use std::{
    collections::HashMap,
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub fn load(file_path: &Path) -> Result<UserConfig, UserConfigError> {
    let file = std::fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut raw_content = String::new();
    reader.read_to_string(&mut raw_content)?;

    let config = toml::from_str::<RawUserConfig>(&raw_content)?;
    Ok(config.try_into()?)
}

pub type ConfigIdT = String;

pub struct UserConfig {
    pub presets: HashMap<ConfigIdT, Preset>,
    pub bullets: HashMap<ConfigIdT, Bullet>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Preset {
    pub id: ConfigIdT,
    pub title: String,
    pub description: String,
    pub default_tone: String,
    pub opening_hook: String,
    pub closing_hook: String,
    pub default_bullets: Vec<ConfigIdT>,

    pub additional_archetypes: Vec<Archetype>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Archetype {
    pub id: ConfigIdT,
    pub title: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
    pub id: ConfigIdT,
    pub title: String,
    pub tags: Vec<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct RawUserConfig {
    pub presets: Vec<Preset>,
    pub bullets: Vec<Bullet>,
}

impl std::default::Default for RawUserConfig {
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

        Self { presets, bullets }
    }
}

#[derive(Debug)]
pub enum UserConfigError {
    MissingReferencedBullet {
        preset_id: ConfigIdT,
        bullet_id: ConfigIdT,
    },
    OtherDeserializationError(toml::de::Error),
    IOError(std::io::Error),
}

impl From<toml::de::Error> for UserConfigError {
    fn from(value: toml::de::Error) -> Self {
        Self::OtherDeserializationError(value)
    }
}

impl From<std::io::Error> for UserConfigError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
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
                if let None = bullets.get(bullet_id) {
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

impl Into<RawUserConfig> for UserConfig {
    fn into(self) -> RawUserConfig {
        RawUserConfig {
            presets: self.presets.clone().into_values().collect(),
            bullets: self.bullets.into_values().collect(),
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
