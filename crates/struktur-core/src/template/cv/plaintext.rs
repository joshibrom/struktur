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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        profile::{Education, Employment, Profile, ProfileLink},
        template::cv::CvTemplateContext,
    };

    #[test]
    fn test_plaintext_cv_template_render_full() {
        let profile = Profile::default();
        let context = CvTemplateContext::from(profile.clone());

        let rendered =
            PlaintextCvTemplate::render(&context).expect("cv template rendering should succeed");

        assert!(rendered.contains(&profile.name));
        assert!(rendered.contains(&profile.email));
        assert!(rendered.contains(&profile.phone));
        assert!(rendered.contains(&profile.location));
        assert!(rendered.contains("Website:"));
        assert!(rendered.contains("GitHub:"));

        assert!(rendered.contains("--- Education ---"));
        assert!(rendered.contains("--- Employment ---"));
        assert!(rendered.contains("--- Projects ---"));
        assert!(rendered.contains("--- Professional Service ---"));

        for edu in &profile.education {
            assert!(rendered.contains(&edu.degree));
            assert!(rendered.contains(&edu.school));
        }

        for emp in &profile.employment {
            assert!(rendered.contains(&emp.title));
            assert!(rendered.contains(&emp.employer));
        }
    }

    #[test]
    fn test_plaintext_cv_template_render_minimal() {
        let minimal_profile = Profile {
            name: "Alex Smith".into(),
            email: "alex@example.com".into(),
            phone: "123-456-7890".into(),
            location: "Remote".into(),
            website: None,
            github: Some(ProfileLink::new("alex", "https://github.com/alex")),
            education: vec![Education {
                degree: "B.S. Math".into(),
                school: "State College".into(),
                start_date: "2020-09".into(),
                end_date: None,
                gpa: None,
                coursework: Vec::new(),
            }],
            employment: vec![Employment {
                title: "Software Engineer".into(),
                employer: "Tech Co".into(),
                location: "Remote".into(),
                start_date: "2022-01".into(),
                end_date: None,
                bullets: vec!["Shipped features.".into()],
            }],
            professional_service: Vec::new(),
            projects: Vec::new(),
        };

        let context = CvTemplateContext::from(minimal_profile);
        let rendered = PlaintextCvTemplate::render(&context)
            .expect("minimal cv template rendering should succeed");

        assert!(rendered.contains("Alex Smith"));
        assert!(rendered.contains("GitHub: https://github.com/alex"));
        assert!(!rendered.contains("Website:"));
        assert!(rendered.contains("Current"));
        assert!(!rendered.contains("GPA:"));
        assert!(!rendered.contains("Coursework:"));
    }

    #[test]
    fn test_plaintext_cv_default_template_fallback() {
        let fallback = PlaintextCvTemplate::get_default_template();
        assert!(fallback.contains("{{ profile.name }}"));
        assert!(fallback.contains("--- Education ---"));
        assert!(fallback.contains("--- Employment ---"));
        assert!(fallback.contains("--- Projects ---"));
        assert!(fallback.contains("--- Professional Service ---"));
    }
}
