use rusqlite::{OptionalExtension, params};

use crate::db::{
    DatabaseError,
    models::{Job, JobEvent, JobStatus},
};

pub type ActionResult<T> = Result<T, DatabaseError>;

pub fn within_transaction<T, Func>(conn: &mut rusqlite::Connection, func: Func) -> ActionResult<T>
where
    Func: FnOnce(&rusqlite::Transaction) -> ActionResult<T>,
{
    let mut conn = conn.transaction()?;
    let val = func(&mut conn)?;
    conn.commit()?;
    Ok(val)
}

pub fn get_jobs(
    conn: &rusqlite::Connection,
    status_filter: Option<JobStatus>,
) -> ActionResult<Vec<Job>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM jobs
        WHERE (?1 IS NULL OR status = ?1)
        ORDER BY updated_at DESC",
    )?;
    let jobs = stmt
        .query_map(params![status_filter], |row| row.try_into())?
        .collect::<Result<Vec<Job>, _>>()?;
    Ok(jobs)
}

pub fn get_job(conn: &rusqlite::Connection, job_id: i64) -> ActionResult<Option<Job>> {
    Ok(conn
        .query_row("SELECT * FROM jobs WHERE id = ?1", params![job_id], |row| {
            row.try_into()
        })
        .optional()?)
}

pub fn insert_job(conn: &rusqlite::Connection, job: &Job) -> ActionResult<i64> {
    conn.execute(
        "INSERT INTO jobs
        (company, role, status, location, date_applied, salary_range, job_url, contact_name, contact_email, notes, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"
        , params![
            job.company,
            job.role,
            job.status,
            job.location,
            job.date_applied,
            job.salary_range,
            job.job_url,
            job.contact_name,
            job.contact_email,
            job.notes,
            job.created_at,
            job.updated_at,
        ])?;
    Ok(conn.last_insert_rowid())
}

pub fn update_job(conn: &rusqlite::Connection, job: &Job) -> ActionResult<()> {
    conn.execute(
        "UPDATE jobs SET
        company = ?2, role = ?3, status = ?4, location = ?5, date_applied = ?6, salary_range = ?7, job_url = ?8, contact_name = ?9, contact_email = ?10, notes = ?11, updated_at = ?12
        WHERE id = ?1"
        , params![
            job.id,
            job.company,
            job.role,
            job.status,
            job.location,
            job.date_applied,
            job.salary_range,
            job.job_url,
            job.contact_name,
            job.contact_email,
            job.notes,
            job.updated_at,
        ])?;
    Ok(())
}

pub fn get_job_events(conn: &rusqlite::Connection, job_id: i64) -> ActionResult<Vec<JobEvent>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM job_events
        WHERE job_id = ?1
        ORDER BY event_date ASC",
    )?;
    let events = stmt
        .query_map(params![job_id], |row| row.try_into())?
        .collect::<Result<Vec<JobEvent>, _>>()?;

    Ok(events)
}

pub fn insert_job_event(conn: &rusqlite::Connection, job_event: &JobEvent) -> ActionResult<i64> {
    conn.execute(
        "INSERT INTO job_events
        (job_id, event_type, title, from_status, to_status, contact_name, contact_email, description, event_date, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"
        , params![
            job_event.job_id,
            job_event.event_type,
            job_event.title,
            job_event.from_status,
            job_event.to_status,
            job_event.contact_name,
            job_event.contact_email,
            job_event.description,
            job_event.event_date,
            job_event.created_at,
        ])?;
    Ok(conn.last_insert_rowid())
}

pub fn transition_job_status(
    conn: &mut rusqlite::Connection,
    job_id: i64,
    new_status: JobStatus,
    description: impl Into<String>,
) -> ActionResult<(Job, JobEvent)> {
    let conn = conn.transaction()?;

    let mut job = get_job(&conn, job_id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    let mut event = job.transition_status(new_status, description);

    update_job(&conn, &job)?;
    event.id = Some(insert_job_event(&conn, &event)?);

    conn.commit()?;
    Ok((job, event))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::JobEventType;
    use time::OffsetDateTime;

    fn setup_test_db() -> rusqlite::Connection {
        let mut conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();
        crate::db::migrations::run(&mut conn).unwrap();
        conn
    }

    #[test]
    fn test_insert_and_get_job() {
        let conn = setup_test_db();
        let mut job = Job::new("Stripe", "Staff Software Engineer")
            .with_location("Remote")
            .with_salary_range("$180k - $220k")
            .with_job_url("https://stripe.com/jobs/123")
            .with_contact(Some("Sarah Recruiter"), Some("sarah@stripe.com"))
            .with_notes("Referred by Alex");

        job.id = Some(insert_job(&conn, &job).unwrap());

        let fetched = get_job(&conn, job.id.unwrap_or(-1))
            .unwrap()
            .expect("job should exist");
        assert_eq!(fetched.id, job.id);
        assert_eq!(fetched.company, "Stripe");
        assert_eq!(fetched.role, "Staff Software Engineer");
        assert_eq!(fetched.status, JobStatus::Saved);
        assert_eq!(fetched.location.as_deref(), Some("Remote"));
        assert_eq!(fetched.salary_range.as_deref(), Some("$180k - $220k"));
        assert_eq!(
            fetched.job_url.as_deref(),
            Some("https://stripe.com/jobs/123")
        );
        assert_eq!(fetched.contact_name.as_deref(), Some("Sarah Recruiter"));
        assert_eq!(fetched.contact_email.as_deref(), Some("sarah@stripe.com"));
        assert_eq!(fetched.notes.as_deref(), Some("Referred by Alex"));

        // Fetching non-existent job returns Ok(None)
        let non_existent = get_job(&conn, -1).unwrap();
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_update_job() {
        let conn = setup_test_db();
        let mut job = Job::new("Acme Corp", "Backend Engineer");
        job.id = Some(insert_job(&conn, &job).unwrap());

        job.location = Some("Austin, conn".to_string());
        job.salary_range = Some("$150k - $175k".to_string());
        job.notes = Some("Updated notes".to_string());
        job.updated_at = OffsetDateTime::now_utc();

        update_job(&conn, &job).unwrap();

        let updated = get_job(&conn, job.id.expect("ID should exist"))
            .unwrap()
            .expect("job should exist");
        assert_eq!(updated.location.as_deref(), Some("Austin, conn"));
        assert_eq!(updated.salary_range.as_deref(), Some("$150k - $175k"));
        assert_eq!(updated.notes.as_deref(), Some("Updated notes"));
    }

    #[test]
    fn test_insert_job_event() {
        let conn = setup_test_db();
        let job = Job::new("Github", "Platform Engineer");
        let id = insert_job(&conn, &job).unwrap();

        let event = JobEvent::new(
            id,
            JobEventType::Interview,
            "Technical screen with hiring manager",
        )
        .with_title("Hiring Manager Screen")
        .with_contact(Some("Jane Manager"), Some("jane@github.com"));

        insert_job_event(&conn, &event).unwrap();
    }

    #[test]
    fn test_transition_job_status() {
        let mut conn = setup_test_db();
        let mut job = Job::new("Shopify", "Senior Developer");
        job.id = Some(insert_job(&conn, &job).unwrap());
        assert_eq!(job.status, JobStatus::Saved);

        let (updated_job, event) = transition_job_status(
            &mut conn,
            job.id.expect("ID should exist"),
            JobStatus::Applied,
            "Submitted application via careers portal",
        )
        .unwrap();

        assert_eq!(updated_job.status, JobStatus::Applied);
        assert_eq!(event.job_id, job.id.unwrap());
        assert_eq!(event.event_type, JobEventType::StatusChange);
        assert_eq!(event.from_status, Some(JobStatus::Saved));
        assert_eq!(event.to_status, Some(JobStatus::Applied));
        assert_eq!(
            event.description,
            "Submitted application via careers portal"
        );

        // Verify state in database was updated
        let db_job = get_job(&conn, job.id.unwrap())
            .unwrap()
            .expect("job should exist");
        assert_eq!(db_job.status, JobStatus::Applied);
    }

    #[test]
    fn test_within_transaction() {
        let mut conn = setup_test_db();
        let job = Job::new("Datadog", "Software Engineer");

        // Successful transaction commits
        let id = within_transaction(&mut conn, |conn| insert_job(conn, &job)).unwrap();

        assert!(get_job(&conn, id).unwrap().is_some());

        // Error rolls back
        let job2 = Job::new("Elastic", "Systems Engineer");
        let result: Result<i64, _> = within_transaction(&mut conn, |conn| {
            insert_job(conn, &job2)?;
            Err(DatabaseError::Io(std::io::Error::other(
                "simulated failure",
            )))
        });

        assert!(result.is_err());
        assert!(job2.id.is_none());
        assert!(get_job(&conn, job2.id.unwrap_or(-1)).unwrap().is_none());
    }
}
