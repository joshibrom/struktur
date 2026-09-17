use struktur_core::db::models::{Job, JobStatus};
use tabled::Tabled;

use super::to_table;

#[derive(Tabled)]
struct JobTableRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "Company")]
    pub company: String,
    #[tabled(rename = "Role")]
    pub role: String,
    #[tabled(rename = "Status")]
    pub status: JobStatus,
    #[tabled(rename = "Location")]
    pub location: String,
    #[tabled(rename = "Application Date")]
    pub date_applied: String,
    #[tabled(rename = "Notes")]
    pub notes: String,
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
                .map(|dt| dt.to_string())
                .unwrap_or_default(),
            notes: value.notes.unwrap_or_default(),
        }
    }
}

pub fn list_jobs_as_table(jobs: &[Job]) -> String {
    let rows = jobs
        .iter()
        .map(|job| job.clone().into())
        .collect::<Vec<JobTableRow>>();

    to_table(rows, 7)
}
