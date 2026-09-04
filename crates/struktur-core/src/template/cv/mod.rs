use serde::Serialize;

use crate::profile::Profile;

pub mod plaintext;

/// Context data passed into template engines for document rendering.
#[derive(Serialize, Debug, Clone)]
pub struct CvTemplateContext {
    /// Master candidate profile containing contact info, education, and experience.
    pub profile: Profile,
}

impl CvTemplateContext {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}
