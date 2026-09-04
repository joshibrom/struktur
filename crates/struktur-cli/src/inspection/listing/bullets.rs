use tabled::Tabled;

use super::{StringList, to_table};
use struktur_core::config::{Bullet, ConfigIdT, UserConfig};

#[derive(Tabled)]
struct BulletTableRow {
    #[tabled(rename = "ID")]
    id: ConfigIdT,
    #[tabled(rename = "Tags")]
    tags: StringList,
    #[tabled(rename = "Text")]
    text: String,
}

impl From<Bullet> for BulletTableRow {
    fn from(value: Bullet) -> Self {
        Self {
            id: value.id,
            text: value.text,
            tags: value.tags.into(),
        }
    }
}

pub fn list_bullets_as_table(config: &UserConfig, tag_filter: Option<String>) -> String {
    let mut rows = config
        .bullets
        .values()
        .map(|bullet| bullet.clone().into())
        .collect::<Vec<BulletTableRow>>();
    rows.sort_by(|a, b| a.id.cmp(&b.id));

    if let Some(tag) = tag_filter {
        rows.retain(|row| row.tags.0.contains(&tag.trim().to_string()));
    }

    to_table(rows, 3)
}
