pub mod bullets;
pub mod presets;

use super::to_table;

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
