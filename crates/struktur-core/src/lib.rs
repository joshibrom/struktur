//! # `struktur-core`
//!
//! Core data models, validation, and XDG-compliant document persistence for the `struktur` CLI.
//!
//! ## Overview
//!
//! `struktur-core` provides the foundational data structures for structured resumes and document generation:
//!
//! * **[`Profile`](crate::profile::Profile)**: Represents personal information, education, employment history, professional service, and portfolio projects.
//! * **[`UserConfig`](crate::config::UserConfig)**: Represents tailored presets, archetypes, and reusable bullet points with referential integrity validation.
//! * **[`XDGDocument`](crate::document::XDGDocument)**: A trait providing standardized persistence (`load`, `save`, `exists`) to XDG-compliant system paths.
//!
//! ## Quick Start
//!
//! ```no_run
//! use struktur_core::{
//!     config::UserConfig,
//!     document::XDGDocument,
//!     ensure_project_files,
//!     profile::Profile,
//! };
//!
//! // Ensure default configuration and profile files exist in XDG directories
//! ensure_project_files().expect("Failed to initialize project files");
//!
//! // Load configuration and user profile
//! let config = UserConfig::load().expect("Failed to load user config");
//! let profile = Profile::load().expect("Failed to load user profile");
//!
//! println!("Loaded profile for {}", profile.name);
//! ```

pub mod xdg;

pub use xdg::*;

