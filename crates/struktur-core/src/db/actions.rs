use rusqlite::{OptionalExtension, params};

use crate::db::{
    DatabaseError,
    models::{Job, JobEvent, JobStatus},
};

pub fn within_transaction<Data, FuncRet>(
    conn: &mut rusqlite::Connection,
    func: impl FnOnce(&rusqlite::Transaction, &Data) -> Result<FuncRet, DatabaseError>,
    data: &Data,
) -> Result<FuncRet, DatabaseError> {
    let mut tx = conn.transaction()?;
    let val = func(&mut tx, data)?;
    tx.commit()?;
    Ok(val)
}

pub fn get_job(tx: &rusqlite::Connection, job_id: &str) -> Result<Option<Job>, DatabaseError> {
    Ok(tx
        .query_row("SELECT * FROM jobs WHERE id = ?1", params![job_id], |row| {
            row.try_into()
        })
        .optional()?)
}

pub fn insert_job(tx: &rusqlite::Connection, job: &Job) -> Result<(), DatabaseError> {
    tx.execute(
        "INSERT INTO jobs
        (id, company, role, status, location, date_applied, salary_range, job_url, contact_name, contact_email, notes, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)"
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
            job.created_at,
            job.updated_at,
        ])?;
    Ok(())
}

pub fn update_job(tx: &rusqlite::Connection, job: &Job) -> Result<(), DatabaseError> {
    tx.execute(
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

pub fn insert_job_event(
    tx: &rusqlite::Connection,
    job_event: &JobEvent,
) -> Result<(), DatabaseError> {
    tx.execute(
        "INSERT INTO job_events
        (id, job_id, event_type, title, from_status, to_status, contact_name, contact_email, description, event_date, created_at
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"
        , params![
            job_event.id,
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
    Ok(())
}

pub fn transition_job_status(
    conn: &mut rusqlite::Connection,
    job_id: &str,
    new_status: JobStatus,
    description: impl Into<String>,
) -> Result<(Job, JobEvent), DatabaseError> {
    let tx = conn.transaction()?;

    let mut job = get_job(&tx, job_id)?.ok_or(rusqlite::Error::QueryReturnedNoRows)?;
    let event = job.transition_status(new_status, description);

    update_job(&tx, &job)?;
    insert_job_event(&tx, &event)?;

    tx.commit()?;
    Ok((job, event))
}
