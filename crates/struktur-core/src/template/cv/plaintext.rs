//! Plaintext / Markdown CV / Resume template implementation.

use serde::{Deserialize, Serialize};

use crate::template::{RenderableTemplate, TemplateArchetype};

/// Plaintext CV template suitable for terminal printing and clipboard export.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlaintextCvTemplate;

impl RenderableTemplate for PlaintextCvTemplate {
    type BaseContext = super::CvTemplateContext;

    fn file_name() -> &'static str {
        "plaintext.tera"
    }

    fn get_archetype() -> TemplateArchetype {
        TemplateArchetype::Cv
    }

    fn get_default_template() -> &'static str {
        DEFAULT_TEMPLATE.trim()
    }
}

/// The embedded default Tera template for plaintext CVs and resumes.
const DEFAULT_TEMPLATE: &str = r#"
{{ profile.name }}
{{ profile.email }} | {{ profile.phone }} | {{ profile.location }}
{% if profile.website %}Website: {{ profile.website.href }}{% endif %}{% if profile.website and profile.github %} | {% endif %}{% if profile.github %}GitHub: {{ profile.github.href }}{% endif %}

--- Education ------------------------------------------------------------------
{% for edu in profile.education %}
{{ edu.degree }}{% if edu.gpa %} (GPA: {{ edu.gpa }}){% endif %}
{{ edu.school }} | {{ edu.start_date }} -- {% if edu.end_date %}{{ edu.end_date }}{% else %}Current{% endif %}
{%- if edu.coursework %}
    Coursework:
{%- for cw in edu.coursework %}
        - {{ cw }}
{%- endfor %}
{%- endif %}
{% endfor %}

--- Employment -----------------------------------------------------------------
{% for emp in profile.employment %}
{{ emp.title }} @ {{ emp.employer }} ({{ emp.location }})
{{ emp.start_date }} -- {% if emp.end_date %}{{ emp.end_date }}{% else %}Current{% endif %}
{%- for bul in emp.bullets %}
    - {{ bul }}
{%- endfor %}
{% endfor %}

--- Projects -------------------------------------------------------------------
{% for proj in profile.projects %}
{{ proj.title }} [{{ proj.category }}] ({{ proj.stack | join(sep=", ") }})
{%- if proj.date %}
{{ proj.date }}
{%- endif %}
{%- for bul in proj.bullets %}
    - {{ bul }}
{%- endfor %}
{% endfor %}

--- Professional Service -------------------------------------------------------
{% for serv in profile.professional_service %}
{{ serv.title }} @ {{ serv.organization }}
{{ serv.start_date }} -- {% if serv.end_date %}{{ serv.end_date }}{% else %}Current{% endif %}
{%- for bul in serv.bullets %}
    - {{ bul }}
{%- endfor %}
{% endfor %}
"#;
