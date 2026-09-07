//! Utility functions, output routing, and date helpers for the CLI.

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Result as AnyResult;

#[cfg(windows)]
const DEFAULT_EDITOR: &str = "notepad";

#[cfg(not(windows))]
const DEFAULT_EDITOR: &str = "vi";

/// Destination target for rendered application materials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputPath {
    /// Print directly to standard output.
    Terminal,
    /// Copy directly to the operating system / desktop clipboard.
    Clipboard,
    /// Write and save to a specified file path on disk.
    File(PathBuf),
}

impl OutputPath {
    /// Determines the output destination from parsed CLI flags.
    ///
    /// Precedence: `--output <path>` > `--clipboard` > `Terminal`.
    pub fn from_cmd_args(output_path: Option<PathBuf>, clipboard: bool) -> Self {
        if let Some(path) = output_path {
            return Self::File(path);
        }

        if clipboard {
            Self::Clipboard
        } else {
            Self::Terminal
        }
    }

    /// Delivers the generated content to the configured output destination.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to disk or setting clipboard contents fails.
    pub fn output(
        self,
        content: impl Into<String>,
        content_type: OutputContentType,
    ) -> AnyResult<()> {
        let content = content.into();
        match self {
            Self::Terminal => {
                println!("{content}");
                Ok(())
            }
            Self::Clipboard => {
                Self::copy_to_clipboard(content)?;
                println!("Copied {content_type} to clipboard.");
                Ok(())
            }
            Self::File(path) => {
                let mut file = std::fs::File::create(&path)?;
                file.write_all(content.as_bytes())?;
                println!("Wrote {content_type} to {}.", path.display());
                Ok(())
            }
        }
    }

    /// Cross-platform clipboard helper supporting WSL (via `clip.exe`) and native desktop environments.
    fn copy_to_clipboard(content: String) -> AnyResult<()> {
        if let Ok(mut child) = std::process::Command::new("clip.exe")
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(content.as_bytes())?;
            }
            let status = child.wait()?;
            if status.success() {
                return Ok(());
            }
        }
        cli_clipboard::set_contents(content)
            .map_err(|e| anyhow::anyhow!("Could not write to clipboard: {e}"))
    }
}

/// Category descriptor for generated document content (used in user-facing status messages).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputContentType {
    /// A cover letter document.
    CoverLetter,
}

impl std::fmt::Display for OutputContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            Self::CoverLetter => "cover letter",
        };

        write!(f, "{}", type_str)
    }
}

/// Formats the current UTC date as a human-readable string (e.g. `"30 August 2026"`).
pub fn today_as_string() -> String {
    let today = time::OffsetDateTime::now_utc().date();
    let format = time::format_description::parse_borrowed::<3>("[day] [month repr:long] [year]")
        .expect("static date format string should be valid");
    today.format(&format).unwrap_or_else(|_| today.to_string())
}

/// Resolves the user's preferred text editor and any configured command-line flags.
///
/// Precedence: `$VISUAL` -> `$EDITOR` -> platform default (`vi` on Unix, `notepad` on Windows).
/// Returns a tuple containing the editor binary name and any space-separated flag arguments.
pub fn get_editor() -> (String, Vec<String>) {
    let env_editor = std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    let mut parts = env_editor.split_whitespace();
    let editor = parts.next().unwrap_or(DEFAULT_EDITOR).to_string();
    let args = parts.map(String::from).collect();

    (editor, args)
}

/// Launches the user's default text editor targeting the specified file path.
///
/// If the file's parent directory does not exist, it is created automatically with an informational notice.
/// Inherits standard I/O to support interactive terminal editing.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if directory creation or editor process spawning fails.
pub fn open_file_in_editor(file_path: &Path) -> std::io::Result<()> {
    let (editor, args) = get_editor();

    if let Some(parent) = file_path.parent()
        && !parent.exists()
    {
        eprintln!("Notice: Created directory '{}'", parent.display());
        std::fs::create_dir_all(parent)?;
    }

    let status = std::process::Command::new(editor)
        .args(args)
        .arg(file_path)
        .status()?;

    if !status.success() {
        let code_str = status
            .code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "terminated by signal".into());
        eprintln!("Warning: Editor exited with non-zero status ({code_str})");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_editor_returns_valid_binary() {
        let (editor, _) = get_editor();
        assert!(!editor.is_empty());
    }
}
