use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::{Document, DocumentError},
    template::{RenderableTemplate, TemplateError, plaintext::PlaintextTemplate},
};
use tabled::Tabled;

enum FileStatus {
    Exists,
    DoesNotExist,
    CouldNotVerify(String),
}

impl From<Result<bool, DocumentError>> for FileStatus {
    fn from(value: Result<bool, DocumentError>) -> Self {
        match value {
            Ok(b) => {
                if b {
                    Self::Exists
                } else {
                    Self::DoesNotExist
                }
            }
            Err(err) => Self::CouldNotVerify(err.to_string()),
        }
    }
}

impl From<Result<bool, TemplateError>> for FileStatus {
    fn from(value: Result<bool, TemplateError>) -> Self {
        match value {
            Ok(b) => {
                if b {
                    Self::Exists
                } else {
                    Self::DoesNotExist
                }
            }
            Err(err) => Self::CouldNotVerify(err.to_string()),
        }
    }
}

impl std::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exists => write!(f, "✓"),
            Self::DoesNotExist => write!(f, "✗"),
            Self::CouldNotVerify(msg) => write!(f, "? ({msg})"),
        }
    }
}

#[derive(Tabled)]
pub struct CheckValue {
    #[tabled(rename = "Filename")]
    filename: &'static str,
    #[tabled(rename = "Path")]
    path: String,
    #[tabled(rename = "Status")]
    status: FileStatus,
}

fn check_config() -> CheckValue {
    CheckValue {
        filename: UserConfig::file_name(),
        path: match UserConfig::get_path() {
            Ok(p) => p.to_str().unwrap_or("path unknown").into(),
            Err(_) => "path unknown".into(),
        },
        status: UserConfig::exists().into(),
    }
}

fn check_profile() -> CheckValue {
    CheckValue {
        filename: Profile::file_name(),
        path: match Profile::get_path() {
            Ok(p) => p.to_str().unwrap_or("path unknown").into(),
            Err(_) => "path unknown".into(),
        },
        status: Profile::exists().into(),
    }
}

fn check_plaintext_template() -> CheckValue {
    CheckValue {
        filename: PlaintextTemplate::file_name(),
        path: match PlaintextTemplate::get_path() {
            Ok(p) => p.to_str().unwrap_or("path unknown").into(),
            Err(_) => "path unknown".into(),
        },
        status: PlaintextTemplate::exists().into(),
    }
}

pub fn check() -> Vec<CheckValue> {
    vec![check_config(), check_profile(), check_plaintext_template()]
}

pub fn check_as_table() -> String {
    super::to_table(check(), 3)
}
