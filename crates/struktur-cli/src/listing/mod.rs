use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Segment},
};

pub mod bullets;
pub mod presets;

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

fn get_term_width() -> usize {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(160), terminal_size::Height(0)))
        .0
        .0 as usize
}

fn to_table<I, T>(rows: I, n_columns: usize) -> String
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let cell_max_width = get_term_width() / n_columns;

    Table::new(rows)
        .with(Style::modern_rounded())
        .with(Modify::new(Segment::all()).with(Width::wrap(cell_max_width).keep_words(true)))
        .to_string()
}
