use serde::Serialize;
use tera::Tera;

use crate::{
    config::{Bullet, Preset, UserConfig},
    profile::Profile,
};

pub mod plaintext;

#[derive(Serialize)]
pub struct TemplateContext {
    pub role: String,
    pub company: String,
    pub date: String,

    pub bullets: Vec<Bullet>,
    pub opening_hook: String,
    pub closing_hook: String,

    pub profile: Profile,
}

impl TemplateContext {
    pub fn new(
        role: String,
        company: String,
        date: String,
        profile: Profile,
        preset: &Preset,
        config: &UserConfig,
    ) -> Result<Self, TemplateError> {
        let bullets = preset
            .default_bullets
            .iter()
            .filter_map(|id| config.bullets.get(id))
            .cloned()
            .collect();

        let this = Self {
            opening_hook: Self::pre_render(
                &preset.opening_hook,
                &role,
                &company,
                &date,
                &profile,
                &bullets,
            )?,
            closing_hook: Self::pre_render(
                &preset.closing_hook,
                &role,
                &company,
                &date,
                &profile,
                &bullets,
            )?,

            role,
            company,
            date,
            bullets,
            profile,
        };

        Ok(this)
    }

    fn pre_render(
        item: &str,
        role: &String,
        company: &String,
        date: &String,
        profile: &Profile,
        bullets: &Vec<Bullet>,
    ) -> Result<String, TemplateError> {
        let mut tera = Tera::default();
        tera.autoescape_on(Vec::<&str>::new());

        let mut ctx = tera::Context::new();
        ctx.insert("role", role);
        ctx.insert("company", company);
        ctx.insert("date", date);
        ctx.insert("profile", profile);
        ctx.insert("bullets", bullets);

        tera.render_str(item, &ctx, false)
            .map_err(TemplateError::TemplateRenderError)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error("Failed to load template: {0}")]
    TemplateLoadError(std::io::Error),
    #[error("Failed to parse template: {0}")]
    TemplateAddError(tera::Error),
    #[error("Failed to render template: {0}")]
    TemplateRenderError(tera::Error),
    #[error("Failed to serialize template context: {0}")]
    TemplateContextSerializationError(tera::Error),
}

pub trait RenderableTemplate {
    fn file_name() -> &'static str;

    fn get_default_template() -> &'static str;

    fn get_path() -> Result<std::path::PathBuf, std::io::Error> {
        let path = crate::storage::get_project_dirs()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find project directories",
            ))?
            .config_dir()
            .to_owned();
        Ok(path.join(format!("templates/{}", Self::file_name())))
    }

    fn load() -> Result<String, std::io::Error> {
        let path = Self::get_path()?;
        std::fs::read_to_string(path)
    }

    fn get_template() -> String {
        match Self::load() {
            Ok(templ) => templ,
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Warning: Failed to load custom template: {err}");
                }
                Self::get_default_template().to_string()
            }
        }
    }

    fn render(context: &TemplateContext) -> Result<String, TemplateError> {
        let template = Self::get_template();

        let mut tera = Tera::new();
        tera.autoescape_on(Vec::<&str>::new());
        tera.add_raw_template("main", &template)
            .map_err(TemplateError::TemplateAddError)?;

        let tera_ctx = tera::Context::from_serialize(context)
            .map_err(TemplateError::TemplateContextSerializationError)?;

        tera.render("main", &tera_ctx)
            .map_err(TemplateError::TemplateRenderError)
    }
}
