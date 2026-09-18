//! Table formatting for job application database models.

use struktur_core::db::models::{Job, JobStatus};
use tabled::Tabled;

use super::to_table;

#[derive(Tabled)]
struct JobTableRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Company")]
    company: String,
    #[tabled(rename = "Role")]
    role: String,
    #[tabled(rename = "Status")]
    status: JobStatus,
    #[tabled(rename = "Location")]
    location: String,
    #[tabled(rename = "Application Date")]
    date_applied: String,
    #[tabled(rename = "Notes")]
    notes: String,
}

impl From<Job> for JobTableRow {
    fn from(value: Job) -> Self {
        Self {
            id: value.id,
            company: value.company,
            role: value.role,
            status: value.status,
            location: value.location.unwrap_or_default(),
            date_applied: value
                .date_applied
                .map(|dt| dt.date().to_string())
                .unwrap_or_default(),
            notes: value.notes.unwrap_or_default(),
        }
    }
}

/// Formats a list of tracked jobs as a terminal table.
pub fn list_jobs_as_table(jobs: &[Job]) -> String {
    let rows = jobs
        .iter()
        .map(|job| job.clone().into())
        .collect::<Vec<JobTableRow>>();

    to_table(rows, 7)
}
