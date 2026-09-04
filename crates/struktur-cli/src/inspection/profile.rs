use struktur_core::{
    config::{Preset, UserConfig},
    profile::Profile,
    template::{RenderableTemplate, TemplateContext, cv::plaintext::PlaintextCvTemplate},
};

pub fn show_profile(profile: Profile, config: &UserConfig) -> String {
    let preset = Preset {
        id: String::new(),
        title: String::new(),
        description: String::new(),
        default_tone: String::new(),
        opening_hook: String::new(),
        closing_hook: String::new(),
        default_bullets: Vec::new(),
        additional_archetypes: Vec::new(),
    };
    let ctx = TemplateContext::new(
        String::new(),
        String::new(),
        String::new(),
        profile,
        &preset,
        config,
    )
    .expect("empty strings should pre-render");

    PlaintextCvTemplate::render(&ctx).expect("template should render")
}
