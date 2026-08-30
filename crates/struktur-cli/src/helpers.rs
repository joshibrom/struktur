use std::{io::Write, path::PathBuf};

use anyhow::Result as AnyResult;

pub enum OutputPath {
    Terminal,
    Clipboard,
    File(PathBuf),
}

impl OutputPath {
    pub fn from_cmd_args(output_path: &Option<PathBuf>, clipboard: &bool) -> Self {
        if let Some(path) = output_path {
            return Self::File(path.to_owned());
        }

        if *clipboard {
            Self::Clipboard
        } else {
            Self::Terminal
        }
    }

    pub fn output(
        self,
        content: impl Into<String>,
        content_type: OutputContentType,
    ) -> AnyResult<()> {
        match self {
            Self::Terminal => Ok(println!("{}", content.into())),
            Self::Clipboard => {
                Self::copy_to_clipboard(content.into())?;
                Ok(println!("Copied {content_type} to clipboard."))
            }
            Self::File(path) => {
                let mut file = std::fs::File::create(&path)?;
                file.write_all(content.into().as_bytes())?;
                Ok(println!("Wrote {content_type} to {}.", path.display()))
            }
        }
    }

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
        cli_clipboard::set_contents(content.into())
            .map_err(|e| anyhow::anyhow!("Could not write to clipboard: {e}"))
    }
}

pub enum OutputContentType {
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

pub fn today_as_string() -> String {
    let today = time::OffsetDateTime::now_utc().date();
    let format = time::format_description::parse_borrowed::<3>("[day] [month repr:long] [year]")
        .expect("static date format string should be valid");
    today.format(&format).unwrap_or_else(|_| today.to_string())
}
