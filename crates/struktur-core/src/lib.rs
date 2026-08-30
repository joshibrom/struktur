//! # `struktur-core`
//!
//! Core data models, validation, and document persistence for the `struktur` CLI.
//!
//! ## Overview
//!
//! `struktur-core` provides the foundational data structures for structured resumes and document generation:
//!
//! * **[`Profile`](crate::profile::Profile)**: Represents personal information, education, employment history, professional service, and portfolio projects.
//! * **[`UserConfig`](crate::config::UserConfig)**: Represents tailored presets, archetypes, and reusable bullet points with referential integrity validation.
//! * **[`Document`](crate::storage::document::Document)**: A trait providing standardized persistence (`load`, `save`, `exists`) to system storage paths.
//!
//! ## Quick Start
//!
//! ```no_run
//! use struktur_core::{
//!     config::UserConfig,
//!     profile::Profile,
//!     storage::{document::Document, init_storage},
//! };
//!
//! // Ensure default configuration, profile, and template files exist in storage directories
//! init_storage().expect("Failed to initialize project files");
//!
//! // Load configuration and user profile
//! let config = UserConfig::load().expect("Failed to load user config");
//! let profile = Profile::load().expect("Failed to load user profile");
//!
//! println!("Loaded profile for {}", profile.name);
//! ```

pub mod config;
pub mod profile;
pub mod storage;
pub mod template;
