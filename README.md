# struktur

A local-first CLI and terminal workstation for generating tailored job application materials and tracking application pipelines.

`struktur` is designed to streamline the job application workflow. Instead of maintaining dozens of disparate cover letter documents or relying on cloud platforms, `struktur` maintains a single structured profile and reusable bullet repository locally, compiling tailored application materials on demand.

---

## Features

* **Master Candidate Profile**: A centralized TOML profile capturing personal details, education, work history, open-source service, and portfolio projects.
* **Modular Bullet Repository**: Reusable, tagged accomplishment bullets that can be dynamically selected and assembled into tailored materials.
* **Role Presets & Archetypes**: Configurable presets (e.g., *Backend Engineer*, *Frontend Engineer*) with custom tone, opening/closing hooks, and persona modifiers.
* **Dual Generation Modes**:
  * *Deterministic Mode*: Fast template-based assembly using dynamic variable interpolation (`{{ role }}`, `{{ company }}`). Supports output to terminal stdout, file writing, or system clipboard.
  * *LLM-Assisted Mode* (Planned): AI-augmented drafting for generating multiple tailored variations for review.
* **Typst-Powered PDF Export* (Planned): High-performance, reproducible PDF document compilation via Typst without heavy TeX distributions.
* **Local Application Tracking** (Planned): Built-in SQLite database to track job applications, submission dates, notes, and the exact materials used.
* **Multi-Interface Design**: Scriptable CLI for automation and an interactive [Ratatui](https://github.com/ratatui/ratatui) TUI (Planned) for managing applications and drafting.

---

## Workspace Structure

The project is structured as a multi-crate Cargo workspace:

```text
struktur/
├── crates/
│   ├── struktur-core/    # Core domain models, validation, template rendering, and persistence
│   ├── struktur-cli/     # Command-line interface for scripting and document generation
│   └── struktur-tui/     # (Planned) Interactive terminal UI dashboard using Ratatui
├── ARCHITECTURE.md       # Technical design, data models, and storage architecture
└── ROADMAP.md            # Phased implementation plan and feature milestones
```

---

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/) (stable toolchain, 2024 edition or later)
* `cargo`

### Installation & Build

Clone the repository and build all workspace crates:

```bash
git clone https://github.com/joshibrom/struktur.git
cd struktur
cargo build
```

Run test suite:

```bash
cargo test
```

### Initial Setup

Initialize default configuration, profile, and template files in standard system directories:

```bash
cargo run -p struktur-cli -- init
```

Default files created:
* **Configuration**: `~/.config/struktur/config.toml` (Presets, archetypes, bullet library)
* **Profile**: `~/.local/share/struktur/profile.toml` (Work history, education, projects, contact links)
* **Templates**: `~/.config/struktur/templates/plaintext.tera` (Customizable cover letter layout)

### Generating Cover Letters

Generate tailored materials using defined presets and target job parameters:

```bash
# Print to terminal stdout
cargo run -p struktur-cli -- generate --preset backend --company Stripe --role "Senior Backend Engineer"

# Copy directly to clipboard (supports native desktop & WSL)
cargo run -p struktur-cli -- generate --preset backend --company Stripe --role "Senior Backend Engineer" --clipboard

# Save directly to a file
cargo run -p struktur-cli -- generate --preset backend --company Stripe --role "Senior Backend Engineer" --output cover_letter.txt
```

---

## Documentation

* [Architecture & System Design](ARCHITECTURE.md)
* [Development Roadmap & Milestones](ROADMAP.md)

---

## License

Licensed under either of:

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

at your option.
