//! CV and Resume template rendering models and definitions.

use serde::{Deserialize, Serialize};

use crate::profile::Profile;

pub mod plaintext;

/// Context data passed into template engines for rendering CV / Resume documents.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
