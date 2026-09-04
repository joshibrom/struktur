//! Plaintext / Markdown cover letter template implementation.

use serde::{Deserialize, Serialize};

use crate::template::{RenderableTemplate, TemplateArchetype};

/// Plaintext cover letter template suitable for terminal printing and clipboard export.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlaintextTemplate;

impl RenderableTemplate for PlaintextTemplate {
    type BaseContext = super::CoverLetterTemplateContext;

    fn file_name() -> &'static str {
        "plaintext.tera"
    }

    fn get_archetype() -> TemplateArchetype {
        TemplateArchetype::CoverLetter
    }

    fn get_default_template() -> &'static str {
        DEFAULT_TEMPLATE.trim()
    }
}

/// The embedded default Tera template for plaintext cover letters.
const DEFAULT_TEMPLATE: &str = r#"
{{ profile.name }}
{{ profile.email }} | {{ profile.phone }} | {{ profile.location }}
{{ date }}

{{ company }}
Regarding: {{ role }} Position

Dear Hiring Team,

{{ opening_hook }}

Key Highlights:
{% for bullet in bullets -%}
* {{ bullet.text }}
{% endfor %}

{{ closing_hook }}

Sincerely,
{{ profile.name }}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::UserConfig,
        profile::Profile,
        template::cover_letter::CoverLetterTemplateContext,
    };

    #[test]
    fn test_plaintext_template_render() {
        let profile = Profile::default();
        let config = UserConfig::default();
        let preset = config.presets.get("backend").expect("preset should exist");

        let context = CoverLetterTemplateContext::new(
            "Staff Backend Engineer".into(),
            "Stripe".into(),
            "August 29, 2026".into(),
            profile.clone(),
            preset,
            &config,
        )
        .expect("context creation should succeed");

        let rendered = PlaintextTemplate::render(&context).expect("rendering should succeed");

        assert!(rendered.contains(&profile.name));
        assert!(rendered.contains(&profile.email));
        assert!(rendered.contains("August 29, 2026"));
        assert!(rendered.contains("Regarding: Staff Backend Engineer Position"));
        assert!(rendered.contains("Stripe"));
        assert!(rendered.contains("Key Highlights:"));

        for bullet in &context.bullets {
            assert!(rendered.contains(&bullet.text));
        }
    }

    #[test]
    fn test_default_template_fallback() {
        let fallback = PlaintextTemplate::get_default_template();
        assert!(fallback.contains("{{ profile.name }}"));
        assert!(fallback.contains("{{ opening_hook }}"));
        assert!(fallback.contains("{{ closing_hook }}"));
    }
}
