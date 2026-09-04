use struktur_core::{
    profile::Profile,
    template::{
        RenderableTemplate, TemplateError,
        cv::{CvTemplateContext, plaintext::PlaintextCvTemplate},
    },
};

pub fn show_profile(profile: Profile) -> Result<String, TemplateError> {
    let ctx = CvTemplateContext::new(profile);
    PlaintextCvTemplate::render(&ctx)
}
