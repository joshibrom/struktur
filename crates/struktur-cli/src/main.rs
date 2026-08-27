fn main() {
    // TODO: Handle error better
    struktur_core::ensure_project_files().expect("project files should be creatable");

    println!("Hello, world!");
}
