use serde::{Deserialize, Serialize};

use crate::template::RenderableTemplate;

#[derive(Serialize, Deserialize)]
pub struct PlaintextTemplate {}

impl RenderableTemplate for PlaintextTemplate {
    fn file_name() -> &'static str {
        "plaintext.tera"
    }

    fn get_default_template() -> &'static str {
        DEFAULT_TEMPLATE
    }
}

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
