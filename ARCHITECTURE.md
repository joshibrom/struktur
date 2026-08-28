# Architecture & Technical Design

This document details the architectural design, domain models, persistence strategy, and engineering trade-offs of `struktur`.

---

## 1. System Overview

`struktur` is architected as a local-first application built around a modular multi-crate workspace. The design separates core domain logic, persistence, rendering, and database operations from user-facing presentation layers (CLI and TUI).

```text
┌────────────────────────────────────────────────────────┐
│                   Presentation Layer                   │
├───────────────────────────┬────────────────────────────┤
│   crates/struktur-cli     │    crates/struktur-tui     │
│   (Scriptable CLI Engine) │ (Interactive Ratatui TUI)  │
└─────────────┬─────────────┴─────────────┬──────────────┘
              │                           │
              └─────────────┬─────────────┘
                            ▼
┌────────────────────────────────────────────────────────┐
│                   Core Engine Layer                    │
│                 crates/struktur-core                   │
├───────────────────────────┬────────────────────────────┤
│   Domain Data Models      │    Template Engine         │
│   (Profile, UserConfig)   │    (Variable Interpolation)│
├───────────────────────────┼────────────────────────────┤
│   Validation Subsystem    │    Document Generators     │
│   (Referential Integrity) │    (Deterministic & LLM)   │
├───────────────────────────┼────────────────────────────┤
│   Storage Layer           │    Typst PDF Compiler      │
│   (XDG Document Trait)    │    (Document Export)       │
└─────────────┬─────────────┴─────────────┬──────────────┘
              │                           │
              ▼                           ▼
┌───────────────────────────┐ ┌──────────────────────────┐
│   Configuration & Data    │ │  Application Tracker DB  │
│   (TOML via Document API) │ │  (Local-First SQLite)    │
│   ~/.config/struktur/     │ │  ~/.local/share/struktur/│
│   ~/.local/share/struktur/│ │  jobs.db                 │
└───────────────────────────┘ └──────────────────────────┘
```

---

## 2. Workspace Crate Boundaries

To enforce loose coupling and maintain clear boundaries, the codebase is partitioned into distinct crates:

| Crate | Responsibility | Dependencies |
| :--- | :--- | :--- |
| **`struktur-core`** | Domain models, schema validation, persistence traits, template rendering, Typst compilation, and LLM integrations. Contains no CLI or UI dependencies. | `serde`, `toml`, `directories` |
| **`struktur-cli`** | Command-line interface providing subcommands (`init`, `generate`, `list`, `profile`, `status`). Suitable for scripting and quick operations. | `struktur-core`, `clap` |
| **`struktur-tui`** *(Planned)* | Interactive terminal user interface for managing job applications, browsing tailored drafts, and status board tracking. | `struktur-core`, `ratatui`, `crossterm` |

---

## 3. Domain Model & Validation

The core domain consists of two primary configuration documents and an application tracking schema:

### 3.1 `Profile` (Candidate Master Record)
Stored in `profile.toml` (XDG data directory). Represents the candidate's complete professional record:
* **Contact Details**: Name, email, phone, location, and web/GitHub links.
* **Education**: Degree, institution, date ranges, GPA, coursework.
* **Employment**: Job title, employer, location, dates, and accomplishment bullets.
* **Professional Service**: Community leadership, open-source maintainership, and speaking.
* **Projects**: Portfolio projects with technology stack tags and bullet descriptions.

### 3.2 `UserConfig` (Presets & Modular Bullets)
Stored in `config.toml` (XDG config directory). Contains tailoring configurations:
* **`Preset`**: Role-specific configuration (e.g., `backend`, `frontend`, `devops`). Defines the target title, tone direction, opening/closing hook templates, and default bullet point references.
* **`Archetype`**: Persona modifiers (e.g., `startup_generalist`, `enterprise_architect`) that alter document tone and emphasis.
* **`Bullet`**: A modular accomplishment bullet with a unique ID, tags (e.g., `["backend", "api"]`), and formatted text.

### 3.3 Referential Integrity & Load-Time Validation
`UserConfig` relies on Serde's `#[serde(try_from = "RawUserConfig", into = "RawUserConfig")]` pattern:
1. **Separation of Representations**: On disk, TOML uses sequences of tables (`Vec<Preset>`, `Vec<Bullet>`). In memory, `UserConfig` indexes entries into `HashMap<ConfigIdT, T>` for $O(1)$ lookups.
2. **Invariant Guarantee**: During deserialization, `TryFrom` validates that every ID in `Preset::default_bullets` exists in the bullet library.
3. **"Parse, Don't Validate"**: Any initialized `UserConfig` struct is guaranteed to be semantically valid. Invalid configurations fail immediately at load time.

---

## 4. Storage & Persistence Architecture

Persistence is abstracted via the `Document` trait in `struktur_core::storage::document`:

```rust
pub trait Document: Serialize + DeserializeOwned + Default {
    fn file_name() -> &'static str;
    fn get_path() -> Result<PathBuf, DocumentError>;
    fn exists() -> Result<bool, DocumentError>;
    fn load() -> Result<Self, DocumentError>;
    fn save(&self) -> Result<(), DocumentError>;
}
```

### Path Resolution
Paths are resolved using standard platform conventions via the `directories` crate:
* **Linux/Unix**: XDG Base Directory Specification (`~/.config/struktur/`, `~/.local/share/struktur/`).
* **macOS**: Standard Application Support directories (`~/Library/Application Support/dev.joshibrom.struktur`).
* **Windows**: Known folder directories (`%APPDATA%/dev/joshibrom/struktur`).

---

## 5. Document Generation Pipeline *(Target Design)*

`struktur` is designed to support two generation workflows for application materials:

```text
                     ┌───────────────────────────┐
                     │     Generation Request    │
                     │  (Preset, Company, Role)  │
                     └─────────────┬─────────────┘
                                   │
                                   ▼
                     ┌───────────────────────────┐
                     │    Context Resolution     │
                     │  (Profile + Preset Match) │
                     └─────────────┬─────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │                             │
                    ▼                             ▼
        [ Deterministic Mode ]          [ LLM-Assisted Mode ]
        ┌───────────────────────┐       ┌───────────────────────┐
        │ Template Engine       │       │ Prompt Assembly       │
        │ Interpolates Hooks &  │       │ (Context + Persona)   │
        │ Assembles Bullets     │       ├───────────────────────┤
        │                       │       │ Provider Request      │
        │                       │       │ Generates Variations  │
        └───────────┬───────────┘       └───────────┬───────────┘
                    │                               │
                    └──────────────┬────────────────┘
                                   │
                                   ▼
                     ┌───────────────────────────┐
                     │     Tailored Document     │
                     └─────────────┬─────────────┘
                                   │
                 ┌─────────────────┼─────────────────┐
                 │                 │                 │
                 ▼                 ▼                 ▼
          [ Plaintext/MD ]    [ Clipboard ]    [ Typst PDF ]
```

1. **Deterministic Mode**: Directly interpolates template variables (`{{.Role}}`, `{{.Company}}`) into opening/closing hooks and formats selected bullet points.
2. **LLM-Assisted Mode**: Constructs a structured prompt incorporating profile data, selected archetypes, tone directives, and role information to generate multiple variations for user selection.
3. **Export Targets**:
   * Clipboard (for copy-pasting into job application forms).
   * Markdown file.
   * Compiled PDF generated via Typst.

---

## 6. Local Job Application Tracking (SQLite) *(Target Design)*

Application tracking will be implemented using an embedded SQLite database (`jobs.db`) stored in the local data directory.

### Status State Machine
```text
[ Saved / Draft ] ──> [ Applied ] ──> [ Interviewing ] ──> [ Offer / Rejected / Withdrawn ]
```

### Key Tracked Entities
* **Job Record**: Company, role, job post URL, location, salary range, status, applied date.
* **Document Snapshot**: Foreign key link or snapshot of the exact preset, bullets, and generated letter used for the application.
* **Activity Log / Notes**: Interview rounds, recruiter contact information, and follow-up deadlines.

---

## 7. Key Technical Trade-offs

| Decision | Chosen Approach | Alternative Considered | Rationale |
| :--- | :--- | :--- | :--- |
| **PDF Compilation** | **Typst** | LaTeX (`pdflatex` / `xelatex`) | Typst compiles in milliseconds, has clean modern markup, and eliminates the requirement for multi-gigabyte TeX distributions. |
| **Persistence Engine** | **Local TOML + SQLite** | Remote Cloud / REST API | Guarantees complete privacy for personal data, zero latency, offline usability, and no recurring hosting overhead. |
| **Configuration Validation** | **Load-Time `TryFrom`** | Runtime / On-demand Checks | Illegal states become unrepresentable in memory; configuration errors are caught immediately before execution begins. |
| **UI Architecture** | **Split CLI + Ratatui TUI** | Monolithic Interactive CLI | Keeps the CLI lean and scriptable for terminal workflows while enabling a full-featured terminal dashboard for application management. |
