use strum::{Display as StrumDisplay, EnumString};
use time::OffsetDateTime;

use crate::template::TemplateArchetype;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: String,
    pub company: String,
    pub role: String,
    pub status: JobStatus,
    pub location: Option<String>,
    pub date_applied: Option<OffsetDateTime>,
    pub salary_range: Option<String>,
    pub job_url: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub notes: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(StrumDisplay, EnumString, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    #[default]
    #[strum(serialize = "saved")]
    Saved,
    #[strum(serialize = "applied")]
    Applied,
    #[strum(serialize = "interviewing")]
    Interviewing,
    #[strum(serialize = "offer")]
    Offer,
    #[strum(serialize = "rejected")]
    Rejected,
    #[strum(serialize = "withdrawn")]
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JobEvent {
    pub id: String,
    pub job_id: String,
    pub event_type: JobEventType,
    pub title: Option<String>,
    pub from_status: Option<JobStatus>,
    pub to_status: Option<JobStatus>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub description: String,

    pub event_date: OffsetDateTime,
    pub created_at: OffsetDateTime,
}

#[derive(StrumDisplay, EnumString, Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobEventType {
    #[strum(serialize = "status_change")]
    StatusChange,
    #[strum(serialize = "note")]
    Note,
    #[strum(serialize = "interview")]
    Interview,
    #[strum(serialize = "follow_up")]
    FollowUp,
    #[strum(serialize = "offer")]
    Offer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rendering {
    pub id: String,
    pub job_id: String,
    pub preset_name: Option<String>,
    pub output_type: TemplateArchetype,
    pub format: String,
    pub rendered_text: String,
    pub context: String,
    pub created_at: OffsetDateTime,
}
