//! Cover letter template rendering models and definitions.

use serde::Serialize;
use tera::Tera;

use crate::{
    config::{Bullet, Preset, UserConfig},
    profile::Profile,
    template::TemplateError,
};

pub mod plaintext;

/// Context data passed into template engines for cover letter document rendering.
#[derive(Serialize, Debug, Clone)]
pub struct CoverLetterTemplateContext {
    /// Target job role or position title (e.g. "Senior Backend Engineer").
    pub role: String,
    /// Target company or organization name (e.g. "Acme Corp").
    pub company: String,
    /// Date of application (e.g. "August 29, 2026").
    pub date: String,

    /// Resolved accomplishment bullet points for the selected preset.
    pub bullets: Vec<Bullet>,
    /// Pre-rendered opening hook paragraph with interpolated role and company.
    pub opening_hook: String,
    /// Pre-rendered closing hook paragraph with interpolated role and company.
    pub closing_hook: String,

    /// Master candidate profile containing contact info, education, and experience.
    pub profile: Profile,
}

impl CoverLetterTemplateContext {
    /// Constructs and pre-renders a new `CoverLetterTemplateContext` from the given parameters.
    ///
    /// The preset's `opening_hook` and `closing_hook` template strings are evaluated
    /// and pre-rendered using the provided `role`, `company`, `date`, `profile`, and `bullets`.
    ///
    /// # Errors
    ///
    /// Returns a [`TemplateError`] if pre-rendering either hook string fails.
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
            .collect::<Vec<_>>();

        let opening_hook = Self::pre_render(
            &preset.opening_hook,
            &role,
            &company,
            &date,
            &profile,
            &bullets,
        )?;

        let closing_hook = Self::pre_render(
            &preset.closing_hook,
            &role,
            &company,
            &date,
            &profile,
            &bullets,
        )?;

        Ok(Self {
            role,
            company,
            date,
            bullets,
            profile,
            opening_hook,
            closing_hook,
        })
    }

    /// Evaluates a template snippet with the current context variables.
    fn pre_render(
        item: &str,
        role: &str,
        company: &str,
        date: &str,
        profile: &Profile,
        bullets: &[Bullet],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_letter_context_new_and_prerender() {
        let profile = Profile::default();
        let config = UserConfig::default();
        let preset = config.presets.get("backend").expect("preset should exist");

        let context = CoverLetterTemplateContext::new(
            "Principal Engineer".into(),
            "Acme Corp".into(),
            "2026-08-29".into(),
            profile.clone(),
            preset,
            &config,
        )
        .expect("context creation should succeed");

        assert_eq!(context.role, "Principal Engineer");
        assert_eq!(context.company, "Acme Corp");
        assert_eq!(context.date, "2026-08-29");
        assert_eq!(context.profile.name, profile.name);
        assert!(!context.bullets.is_empty());

        // Verify pre-rendered hooks replaced variables
        assert!(context.opening_hook.contains("Principal Engineer"));
        assert!(context.opening_hook.contains("Acme Corp"));
        assert!(!context.opening_hook.contains("{{ role }}"));
        assert!(!context.opening_hook.contains("{{ company }}"));

        assert!(context.closing_hook.contains("Acme Corp"));
        assert!(!context.closing_hook.contains("{{ company }}"));
    }
}
