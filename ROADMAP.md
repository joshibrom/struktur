# Development Roadmap & Milestones

This roadmap outlines the phased development plan for `struktur`, tracking completed work and upcoming milestones.

---

## Milestone Overview

```text
[ Phase 1: Core Foundation ]          ✅ Completed
             │
             ▼
[ Phase 2: Template & Generation ]    ✅ Completed
             │
             ▼
[ Phase 3: CLI Inspection & Mgmt ]    🔄 Next Milestone
             │
             ▼
[ Phase 4: SQLite Job Tracker ]       ⏳ Planned
             │
             ▼
[ Phase 5: Typst PDF Generation ]     ⏳ Planned
             │
             ▼
[ Phase 6: LLM Generation Layer ]     ⏳ Planned
             │
             ▼
[ Phase 7: Ratatui TUI Dashboard ]    ⏳ Planned
```

---

## Phase 1: Core Foundation & Persistence (Completed)

* [x] **Workspace Setup**: Cargo multi-crate workspace initialized with `struktur-core` and `struktur-cli`.
* [x] **Domain Models**: Data structures for candidate `Profile` (education, employment, service, projects) and `UserConfig` (presets, archetypes, bullets).
* [x] **Storage Abstraction**: `Document` trait for standardized path resolution and document loading/saving (`profile.toml` and `config.toml`).
* [x] **Load-Time Validation**: Referential integrity validation enforcing bullet existence for presets.
* [x] **Default Datasets**: Sensible starter templates for `config.toml` and `profile.toml` generated on `struktur init`.
* [x] **Testing & Documentation**: Unit tests for round-trip serialization, error conditions, and complete Rustdoc documentation.

---

## Phase 2: Template Interpolation & Document Generation (Completed)

* [x] **Template Engine**: Integrated Tera templating supporting Jinja2 syntax (`{{ role }}`, `{{ company }}`, `{{ date }}`, `{{ profile.* }}`).
* [x] **Hook Pre-Rendering**: Dynamic evaluation and pre-rendering of preset opening and closing hooks inside `TemplateContext`.
* [x] **Document Template Trait**: `RenderableTemplate` trait managing user filesystem overrides (`~/.config/struktur/templates/`) with embedded static fallbacks.
* [x] **Plaintext & Markdown Template**: `PlaintextTemplate` formatting candidate header, date, recipient, opening hook, bullet points, and closing hook.
* [x] **CLI `generate` Command**: `struktur generate --preset <id> --company <name> --role <title> [--date <date>] [--output <file>] [--clipboard]`.
* [x] **Multi-Target Output Routing**: Clean output delivery to terminal stdout, file writing, or system clipboard (with WSL `clip.exe` bridge).
* [x] **Testing & Verification**: Unit tests for context pre-rendering, template fallback, and full cover letter compilation.

---

## Phase 3: CLI Inspection & Profile Management

* [ ] **Inspection Subcommands**:
  * `struktur list presets`: Formatted table of available role presets and tone descriptions.
  * `struktur list bullets [--tag <name>]`: Table of reusable bullet points with tag filtering.
  * `struktur profile show`: Human-readable summary of the current user profile.
  * `struktur status`: Path diagnostics showing active config, profile, and data file locations.
* [ ] **Editor Integration**:
  * `struktur edit [config|profile]`: Helper to open configuration files in the user's `$EDITOR`.
* [ ] **Validation Subcommand**:
  * `struktur validate`: Checks existing configuration and profile files for syntax or reference errors.

---

## Phase 4: Local Job Application Tracking (SQLite Integration)

* [ ] **Database Schema**:
  * Define schema for job applications: `company`, `role`, `status` (Saved, Applied, Interviewing, Offer, Rejected), `date_applied`, `salary_range`, `job_url`, `notes`.
  * Snapshot table storing the exact rendered text and preset parameters used for each application.
* [ ] **Database Engine (`struktur-core::db` or `struktur-db`)**:
  * Embedded SQLite management via `rusqlite` or `sqlx`.
  * Automatic schema migrations on initialization.
  * Database file stored in XDG data directory (`~/.local/share/struktur/jobs.db`).
* [ ] **CLI Tracking Commands**:
  * `struktur job add --company <name> --role <title> [--status <status>]`: Log a new application.
  * `struktur job list [--status <status>]`: Display active applications and pipeline statuses.
  * `struktur job update <id> --status <status>`: Advance an application through the pipeline.

---

## Phase 5: Typst-Powered PDF Compilation

* [ ] **Typst Template Engine**:
  * Create clean, professional Typst cover letter and resume templates in `struktur-core`.
  * Define configurable style properties (margins, typography, accent color, header layout).
* [ ] **Compilation Pipeline**:
  * Inject structured document data into Typst templates.
  * Compile directly to PDF (via `typst` Rust crate or CLI invocation).
* [ ] **CLI Export Flag**:
  * `struktur generate ... --output pdf --out-file letter.pdf`: Direct PDF compilation.

---

## Phase 6: LLM-Assisted Generation & Variation Management

* [ ] **Provider Abstraction**:
  * Trait in `struktur-core` for LLM providers (supporting OpenAI, Anthropic, Gemini, and local Ollama).
* [ ] **Prompt Formulation**:
  * Structured prompt constructor combining candidate profile, target role context, preset tone directives, and archetype prompts.
* [ ] **Variation Generator**:
  * Request generation of 2–3 distinct drafts with varying emphasis.
* [ ] **CLI Review Workflow**:
  * Output numbered variations to stdout for selection, editing, or direct PDF export.

---

## Phase 7: Interactive Terminal Dashboard (`struktur-tui`)

* [ ] **New Binary Crate**: Initialize `crates/struktur-tui` built on [Ratatui](https://github.com/ratatui/ratatui) and `crossterm`.
* [ ] **Job Pipeline Kanban Board**:
  * Visual status board with columns for *Saved*, *Applied*, *Interviewing*, *Offered*, and *Archived*.
* [ ] **Interactive Generation Wizard**:
  * Step-by-step form to select presets, toggle bullets, input target company/role, and preview rendered output.
* [ ] **Profile & Preset Editor**:
  * In-terminal browsing and quick editing of bullets, presets, and profile details.
* [ ] **Clipboard & PDF Actions**:
  * Single-key shortcuts to compile PDF, copy to clipboard, or log to database.
