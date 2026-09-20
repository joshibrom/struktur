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
* **Local Application Tracking**: Built-in SQLite database with schema migrations to track job applications, submission dates, pipeline statuses, notes, timeline events, and historical document snapshots.
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
* **Templates**: `~/.config/struktur/templates/cover-letter/plaintext.tera` and `~/.config/struktur/templates/cv/plaintext.tera`

### Inspecting & Managing Application Data

```bash
# List configured role presets in a terminal table
cargo run -p struktur-cli -- list presets

# List modular accomplishment bullets (optionally filtered by tag)
cargo run -p struktur-cli -- list bullets
cargo run -p struktur-cli -- list bullets --tag backend

# Display a formatted summary of the candidate profile (or raw JSON)
cargo run -p struktur-cli -- profile show
cargo run -p struktur-cli -- profile show --json

# Check filesystem paths and file existence
cargo run -p struktur-cli -- status

# Open configuration, profile, or templates in your $EDITOR
cargo run -p struktur-cli -- edit config
cargo run -p struktur-cli -- edit profile
cargo run -p struktur-cli -- edit template cover-letter
cargo run -p struktur-cli -- edit template cv

# Validate configuration syntax, referential integrity, and template parsing
cargo run -p struktur-cli -- validate
```

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

### Tracking Job Applications & Pipeline Management

Track and advance applications through the hiring pipeline, linking tailored documents to job records:

```bash
# Add a new job application (optional fields: --location, --salary, --url, --contact-name, --notes, --status)
cargo run -p struktur-cli -- job add --company "Acme Corp" --role "Backend Engineer" --location Remote --status saved

# List tracked applications in a terminal table (optionally filter by status)
cargo run -p struktur-cli -- job list
cargo run -p struktur-cli -- job list --status applied

# View detailed job info, chronological status change timeline, and document snapshots
cargo run -p struktur-cli -- job show 1

# Advance application status (records a timeline event and automatically sets date_applied on 'applied')
cargo run -p struktur-cli -- job update 1 status applied --description "Submitted via company careers portal"
cargo run -p struktur-cli -- job update 1 status interviewing --description "Initial recruiter screen scheduled"

# Update specific job fields
cargo run -p struktur-cli -- job update 1 salary "$160k - $185k"
cargo run -p struktur-cli -- job update 1 contact --name "Jane Smith" --email "jane@acme.example"

# Generate tailored document using job details and automatically record a document snapshot
cargo run -p struktur-cli -- job generate 1 --preset backend

# Delete a job record and all cascaded timeline events and document snapshots
cargo run -p struktur-cli -- job rm 1
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
