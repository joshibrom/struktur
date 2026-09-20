//! Subcommand execution handlers for the CLI.

use anyhow::Result as AnyResult;
use struktur_core::{storage::document::Document, template::RenderableTemplate};

use crate::helpers::open_file_in_editor;

pub mod generate;
pub mod job;
pub mod profile;
pub mod system;

pub type ActionResult = AnyResult<()>;

fn edit_document<D: Document>() -> ActionResult {
    let path = D::get_path()?;
    Ok(open_file_in_editor(&path)?)
}

fn edit_template<T: RenderableTemplate>() -> ActionResult {
    let path = T::get_path()?;
    Ok(open_file_in_editor(&path)?)
}
