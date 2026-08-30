use anyhow::Result as AnyResult;
use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::Document,
    template::{RenderableTemplate, TemplateContext, plaintext::PlaintextTemplate},
};

use crate::helpers::{OutputContentType, OutputPath};

pub fn init() -> AnyResult<()> {
    struktur_core::storage::init_storage()?;
    println!("Created project files.");
    Ok(())
}

pub fn generate(
    preset_name: String,
    company: String,
    role: String,
    date: String,
    output_path: OutputPath,
) -> AnyResult<()> {
    let config = UserConfig::load()?;
    let profile = Profile::load()?;

    let preset = config
        .presets
        .get(&preset_name)
        .ok_or(anyhow::anyhow!("Unknown preset: {preset_name}"))?;

    let context = TemplateContext::new(role, company, date, profile, preset, &config)?;

    let content = PlaintextTemplate::render(&context)?;

    output_path.output(content, OutputContentType::CoverLetter)
}
