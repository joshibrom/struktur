use struktur_core::db::{
    self,
    models::{Job, JobEventType, JobStatus},
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

pub fn show(job_id: i64) -> ActionResult {
    let conn = db::open()?;
    let job = db::actions::get_job(&conn, job_id)?
        .ok_or(anyhow::anyhow!("Job with ID {job_id} not found"))?;
    let events = db::actions::get_job_events(&conn, job_id)?;
    let renders = db::actions::get_renderings_for_job(&conn, job_id)?;

    let full_sep = "=".repeat(80);
    let half_sep = "-".repeat(80);

    println!(
        "[#{}] {} at {}",
        job.id.unwrap_or_default(),
        job.role,
        job.company
    );
    println!("{full_sep}");

    let job_fields = [
        ("Status", job.status.to_string()),
        (
            "Applied",
            job.date_applied
                .map(|dt| dt.date().to_string())
                .unwrap_or_default(),
        ),
        ("Location", job.location.clone().unwrap_or_default()),
        ("Salary", job.salary_range.clone().unwrap_or_default()),
        ("Job URL", job.job_url.clone().unwrap_or_default()),
        ("Contact", job.get_contact_reference().unwrap_or_default()),
        ("Notes", job.notes.unwrap_or_default()),
    ];

    for (col, val) in job_fields {
        if !val.is_empty() {
            println!("{:<13} {}", format!("{col}:"), val);
        }
    }

    println!("\nTIMELINE ({} events)", events.len());
    println!("{half_sep}");

    if !events.is_empty() {
        for event in &events {
            let tagline = match &event.event_type {
                &JobEventType::StatusChange => format!(
                    "{} -> {}",
                    event
                        .from_status
                        .map(|status| status.to_string())
                        .unwrap_or_default(),
                    event
                        .to_status
                        .map(|status| status.to_string())
                        .unwrap_or_default()
                ),
                _ => event.title.clone().unwrap_or_default(),
            };
            const DATE_WIDTH: usize = 11;
            const TYPE_WIDTH: usize = 14;
            println!(
                "- {:<DATE_WIDTH$} {:<TYPE_WIDTH$} {}",
                event.event_date.date().to_string(),
                event.event_type.to_string(),
                tagline
            );
            if !event.description.trim().is_empty() {
                println!(
                    "  {:<prefix_width$} {}\n",
                    ' ',
                    event.description,
                    prefix_width = DATE_WIDTH + TYPE_WIDTH + 1
                );
            }
        }
    } else {
        println!("  (No events found for this job)")
    }

    println!("\nDOCUMENTS ({} renderings)", renders.len());
    println!("{half_sep}");

    if !renders.is_empty() {
        for render in &renders {
            let render_id = render.id.map(|id| format!("#{id}")).unwrap_or_default();
            let preset = render
                .preset_name
                .as_deref()
                .map(|p| format!("[{p}]"))
                .unwrap_or_default();
            println!(
                "  {:<4} {:<14} {:<12} ({})  {}",
                render_id,
                render.output_type.to_string(),
                preset,
                render.format,
                render.created_at.date(),
            );
        }
    } else {
        println!(
            "  (No documents generated yet. Run 'struktur generate ... --job {job_id}' to link one)"
        );
    }

    println!("{full_sep}");

    Ok(())
}
