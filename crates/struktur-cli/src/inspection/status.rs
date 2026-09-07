//! Filesystem status and diagnostics inspection.

use std::path::PathBuf;

use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::Document,
    template::{
        RenderableTemplate, cover_letter::plaintext::PlaintextTemplate,
        cv::plaintext::PlaintextCvTemplate,
    },
};

/// Health and existence check for a single project file.
pub struct FileCheck {
    name: &'static str,
    path: Option<PathBuf>,
    exists: bool,
}

impl std::fmt::Display for FileCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if self.exists { "✓" } else { "✗" };
        let path_str = match &self.path {
            Some(p) => p.display().to_string(),
            None => "path unknown".to_string(),
        };
        let hint = if self.exists {
            String::new()
        } else {
            " (missing - run 'struktur init')".to_string()
        };

        write!(f, "  {mark}  {:<16}  {path_str}{hint}", self.name)
    }
}

fn check_document<D: Document>() -> FileCheck {
    let path = D::get_path().ok();
    let exists = path.as_ref().map(|p| p.exists()).unwrap_or(false);
    FileCheck {
        name: D::file_name(),
        path,
        exists,
    }
}

fn check_template<T: RenderableTemplate>() -> FileCheck {
    let path = T::get_path().ok();
    let exists = path.as_ref().map(|p| p.exists()).unwrap_or(false);
    FileCheck {
        name: T::file_name(),
        path,
        exists,
    }
}

pub fn check() -> Vec<FileCheck> {
    vec![
        check_document::<UserConfig>(),
        check_document::<Profile>(),
        check_template::<PlaintextTemplate>(),
        check_template::<PlaintextCvTemplate>(),
    ]
}
