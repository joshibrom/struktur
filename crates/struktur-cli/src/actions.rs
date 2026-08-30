use anyhow::Result as AnyResult;
use struktur_core::{
    config::UserConfig,
    profile::Profile,
    storage::document::Document,
    template::{RenderableTemplate, TemplateContext, plaintext::PlaintextTemplate},
};

pub fn init() -> AnyResult<()> {
    // TODO: Handle error better
    struktur_core::storage::init_storage()?;
    println!("Created project files."); // TODO: Better logging
    Ok(())
}

pub fn generate(preset_name: String, company: String, role: String, date: String) -> AnyResult<()> {
    let config = UserConfig::load()?;
    let profile = Profile::load()?;

    let preset = config
        .presets
        .get(&preset_name)
        .ok_or(anyhow::anyhow!("Unknown preset: {preset_name}"))?;

    let context = TemplateContext::new(role, company, date, profile, preset, &config)?;

    let content = PlaintextTemplate::render(&context)?;

    println!("{content}"); // TODO: Write to file / clipboard

    Ok(())
}
