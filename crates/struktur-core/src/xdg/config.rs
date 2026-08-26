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
struct RawUserConfig {
    pub presets: Vec<Preset>,
    pub bullets: Vec<Bullet>,
}

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
