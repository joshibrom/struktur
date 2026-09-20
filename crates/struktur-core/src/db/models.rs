use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, ToSqlOutput},
};
use serde::Serialize;
use strum::{Display as StrumDisplay, EnumString};
use time::OffsetDateTime;

use std::str::FromStr;

use crate::template::TemplateArchetype;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: Option<i64>,
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
            id: None,
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

    /// Updates the `updated_at` timestamp to the current UTC time.
    pub fn update_time(&mut self) {
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

        JobEvent::new_status_change(
            self.id
                .expect("Job should exist with ID before status transition"),
            old_status,
            new_status,
            description,
        )
    }

    pub fn get_contact_reference(&self) -> Option<String> {
        match &self.contact_name {
            Some(name) => match &self.contact_email {
                Some(email) => Some(format!("{name} <{email}>")),
                None => Some(name.to_owned()),
            },
            None => self.contact_email.as_ref().map(|email| email.to_owned()),
        }
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Job {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(Job {
            id: row.get("id")?,
            company: row.get("company")?,
            role: row.get("role")?,
            status: row.get("status")?,
            location: row.get("location")?,
            date_applied: row.get("date_applied")?,
            salary_range: row.get("salary_range")?,
            job_url: row.get("job_url")?,
            contact_name: row.get("contact_name")?,
            contact_email: row.get("contact_email")?,
            notes: row.get("notes")?,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
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
    pub id: Option<i64>,
    pub job_id: i64,
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
    pub fn new(job_id: i64, event_type: JobEventType, description: impl Into<String>) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: None,
            job_id,
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
        job_id: i64,
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

impl TryFrom<&rusqlite::Row<'_>> for JobEvent {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(JobEvent {
            id: row.get("id")?,
            job_id: row.get("job_id")?,
            event_type: row.get("event_type")?,
            title: row.get("title")?,
            from_status: row.get("from_status")?,
            to_status: row.get("to_status")?,
            description: row.get("description")?,
            contact_name: row.get("contact_name")?,
            contact_email: row.get("contact_email")?,
            event_date: row.get("event_date")?,
            created_at: row.get("created_at")?,
        })
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
    pub id: Option<i64>,
    pub job_id: i64,
    pub preset_name: Option<String>,
    pub output_type: TemplateArchetype,
    pub format: String,
    pub rendered_text: String,
    pub context: String,
    pub created_at: OffsetDateTime,
}

impl Rendering {
    pub fn new(
        job_id: i64,
        output_type: TemplateArchetype,
        format: impl Into<String>,
        rendered_text: impl Into<String>,
        context: impl Serialize,
    ) -> Self {
        let now = OffsetDateTime::now_utc();

        Self {
            id: None,
            job_id,
            preset_name: None,
            output_type,
            format: format.into(),
            rendered_text: rendered_text.into(),
            context: serde_json::to_string(&context).unwrap_or_default(),
            created_at: now,
        }
    }

    pub fn with_preset(mut self, preset_name: impl Into<String>) -> Self {
        self.preset_name = Some(preset_name.into());
        self
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Rendering {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id")?,
            job_id: row.get("job_id")?,
            preset_name: row.get("preset_name")?,
            output_type: row.get("output_type")?,
            format: row.get("format")?,
            rendered_text: row.get("rendered_text")?,
            context: row.get("context")?,
            created_at: row.get("created_at")?,
        })
    }
}
