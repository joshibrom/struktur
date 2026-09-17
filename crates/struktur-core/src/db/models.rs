use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, ToSqlOutput},
};
use strum::{Display as StrumDisplay, EnumString};
use time::OffsetDateTime;

use std::str::FromStr;

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

impl Job {
    pub fn new(company: impl Into<String>, role: impl Into<String>) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: uuid::Uuid::now_v7().into(),
            company: company.into(),
            role: role.into(),
            status: JobStatus::default(),
            location: None,
            date_applied: None,
            salary_range: None,
            job_url: None,
            contact_name: None,
            contact_email: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_apply_now(self) -> Self {
        self.with_application_date(OffsetDateTime::now_utc())
            .with_job_status(JobStatus::Applied)
    }

    pub fn with_application_date(mut self, date_applied: OffsetDateTime) -> Self {
        self.date_applied = Some(date_applied);
        self
    }

    pub fn with_contact(
        mut self,
        name: Option<impl Into<String>>,
        email: Option<impl Into<String>>,
    ) -> Self {
        self.contact_name = name.map(|s| s.into());
        self.contact_email = email.map(|s| s.into());
        self
    }

    pub fn with_job_status(mut self, status: JobStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_job_url(mut self, url: impl Into<String>) -> Self {
        self.job_url = Some(url.into());
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn with_salary_range(mut self, salary_range: impl Into<String>) -> Self {
        self.salary_range = Some(salary_range.into());
        self
    }

    fn update_time(&mut self) {
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn transition_status(
        &mut self,
        new_status: JobStatus,
        description: impl Into<String>,
    ) -> JobEvent {
        let old_status = self.status;
        self.status = new_status;
        self.update_time();

        JobEvent::new_status_change(&self.id, old_status, new_status, description)
    }
}

#[derive(StrumDisplay, EnumString, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum JobStatus {
    #[default]
    Saved,
    Applied,
    Interviewing,
    Offer,
    Rejected,
    Withdrawn,
}

impl ToSql for JobStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for JobStatus {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let text = value.as_str()?;
        Self::from_str(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
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

impl JobEvent {
    pub fn new(
        job_id: impl Into<String>,
        event_type: JobEventType,
        description: impl Into<String>,
    ) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: uuid::Uuid::now_v7().into(),
            job_id: job_id.into(),
            event_type,
            title: None,
            from_status: None,
            to_status: None,
            contact_name: None,
            contact_email: None,
            description: description.into(),
            event_date: now,
            created_at: now,
        }
    }

    pub fn new_status_change(
        job_id: impl Into<String>,
        from: JobStatus,
        to: JobStatus,
        description: impl Into<String>,
    ) -> Self {
        Self::new(job_id, JobEventType::StatusChange, description).with_transition(from, to)
    }

    pub fn with_contact(
        mut self,
        name: Option<impl Into<String>>,
        email: Option<impl Into<String>>,
    ) -> Self {
        self.contact_name = name.map(|s| s.into());
        self.contact_email = email.map(|s| s.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_transition(mut self, from: JobStatus, to: JobStatus) -> Self {
        self.from_status = Some(from);
        self.to_status = Some(to);
        self
    }
}

#[derive(StrumDisplay, EnumString, Debug, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum JobEventType {
    StatusChange,
    Note,
    Interview,
    FollowUp,
    Offer,
}

impl ToSql for JobEventType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromSql for JobEventType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let text = value.as_str()?;
        Self::from_str(text).map_err(|err| FromSqlError::Other(Box::new(err)))
    }
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
