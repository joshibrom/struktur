use tabled::Tabled;

use super::{StringList, to_table};
use struktur_core::config::{ConfigIdT, Preset, UserConfig};

#[derive(Tabled)]
struct PresetTableRow {
    #[tabled(rename = "ID")]
    id: ConfigIdT,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Tone")]
    default_tone: String,
    #[tabled(rename = "Bullets")]
    bullets: StringList,
    #[tabled(rename = "Archetypes")]
    archetypes: StringList,
}

impl From<Preset> for PresetTableRow {
    fn from(value: Preset) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            default_tone: value.default_tone,
            bullets: value.default_bullets.into(),
            archetypes: value
                .additional_archetypes
                .into_iter()
                .map(|arch| arch.id)
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

pub fn list_presets_as_table(config: &UserConfig) -> String {
    let mut rows = config
        .presets
        .values()
        .map(|preset| preset.clone().into())
        .collect::<Vec<PresetTableRow>>();
    rows.sort_by(|a, b| a.id.cmp(&b.id));

    to_table(rows, 6)
}
