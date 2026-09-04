//! Formatted candidate profile rendering and inspection.

use struktur_core::{
    profile::Profile,
    template::{
        RenderableTemplate, TemplateError,
        cv::{CvTemplateContext, plaintext::PlaintextCvTemplate},
    },
};

/// Renders the provided profile into a human-readable CV / resume string using the plaintext template.
///
/// # Errors
///
/// Returns a [`TemplateError`] if template loading or rendering fails.
pub fn show_profile(profile: Profile) -> Result<String, TemplateError> {
    let ctx = CvTemplateContext::new(profile);
    PlaintextCvTemplate::render(&ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_profile_renders_successfully() {
        let profile = Profile::default();
        let rendered = show_profile(profile).expect("default profile should render");
        assert!(rendered.contains("Jane Doe"));
        assert!(rendered.contains("--- Education ---"));
        assert!(rendered.contains("--- Employment ---"));
    }
}
