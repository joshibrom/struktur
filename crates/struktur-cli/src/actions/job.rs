use struktur_core::db::{
    self,
    models::{Job, JobStatus},
};

use crate::{cmd::JobAddArgs, inspection};

use super::ActionResult;

/// Adds a new job application record to the local SQLite database.
///
/// # Errors
///
/// Returns an error if the database cannot be opened or if the database insertion transaction fails.
pub fn add(args: JobAddArgs) -> ActionResult {
    let mut job = Job::new(args.company, args.role);

    if let Some(status) = args.status {
        if status == JobStatus::Applied {
            job = job.with_apply_now();
        } else {
            job.status = status;
        }
    }

    job.location = args.location;
    job.salary_range = args.salary;
    job.job_url = args.url;
    job.notes = args.notes;
    job.contact_name = args.contact_name;
    job.contact_email = args.contact_email;

    let mut conn = db::open()?;
    let id =
        db::actions::within_transaction(&mut conn, |conn| db::actions::insert_job(conn, &job))?;

    println!("Added {} at {} (ID: {})", job.role, job.company, id);

    Ok(())
}

/// Lists tracked job applications formatted as a terminal table, optionally filtered by status.
///
/// # Errors
///
/// Returns an error if the database cannot be opened or if querying jobs fails.
pub fn list_all(status_filter: Option<JobStatus>) -> ActionResult {
    let conn = db::open()?;
    let jobs = db::actions::get_jobs(&conn, status_filter)?;

    let table = inspection::listing::db_models::list_jobs_as_table(jobs.as_slice());
    println!("{table}");

    Ok(())
}

/// Transitions the status of a tracked job application and records a status change event.
///
/// # Errors
///
/// Returns an error if the database cannot be opened, the job is not found, or the status transition fails.
pub fn update_status(job_id: i64, status: JobStatus, description: Option<String>) -> ActionResult {
    let mut conn = db::open()?;
    let (job, event) = db::actions::transition_job_status(
        &mut conn,
        job_id,
        status,
        description.as_deref().unwrap_or_default(),
    )?;

    let from = event
        .from_status
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let to = event
        .to_status
        .map(|s| s.to_string())
        .unwrap_or_else(|| status.to_string());

    println!(
        "Updated [{}] {} at {}: {from} -> {to}",
        job.id.unwrap_or(job_id),
        job.role,
        job.company,
    );

    let event_id = event.id.map(|id| format!("#{id}")).unwrap_or_default();

    match description {
        Some(desc) if !desc.trim().is_empty() => {
            println!("  Event {event_id} logged: \"{desc}\"");
        }
        _ => {
            println!("  Event {event_id} logged");
        }
    }

    Ok(())
}
