pub mod presets;

fn get_term_width() -> usize {
    terminal_size::terminal_size()
        .unwrap_or((terminal_size::Width(160), terminal_size::Height(0)))
        .0
        .0 as usize
}
