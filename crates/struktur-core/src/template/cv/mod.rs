//! CV and Resume template rendering models and definitions.

use serde::Serialize;

use crate::profile::Profile;

pub mod plaintext;

/// Context data passed into template engines for rendering CV / Resume documents.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CvTemplateContext {
    /// Master candidate profile containing contact info, education, and experience.
    pub profile: Profile,
}

impl CvTemplateContext {
    /// Constructs a new `CvTemplateContext` wrapping the provided candidate profile.
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

impl From<Profile> for CvTemplateContext {
    fn from(profile: Profile) -> Self {
        Self::new(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cv_template_context_new() {
        let profile = Profile::default();
        let context = CvTemplateContext::new(profile.clone());
        assert_eq!(context.profile, profile);

        let from_context: CvTemplateContext = profile.clone().into();
        assert_eq!(from_context.profile, profile);
    }
}
