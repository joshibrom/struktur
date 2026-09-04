//! Plaintext / Markdown cover letter template implementation.

use serde::{Deserialize, Serialize};

use crate::template::{RenderableTemplate, TemplateArchetype};

/// Plaintext cover letter template suitable for terminal printing and clipboard export.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlaintextTemplate;

impl RenderableTemplate for PlaintextTemplate {
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
