use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Segment},
};

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

    let cell_max_width = (super::get_term_width() as f32 / 6.0).ceil() as usize;

    Table::new(rows)
        .with(Style::modern_rounded())
        .with(Modify::new(Segment::all()).with(Width::wrap(cell_max_width).keep_words(true)))
        .to_string()
}

struct StringList(Vec<String>);

impl From<Vec<String>> for StringList {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for StringList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.0.iter() {
            writeln!(f, "- {s}")?;
        }
        Ok(())
    }
}
