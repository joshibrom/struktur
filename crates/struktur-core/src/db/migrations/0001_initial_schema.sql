CREATE TABLE IF NOT EXISTS jobs (
    id              INTEGER PRIMARY KEY,
    company         TEXT NOT NULL,
    role            TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'saved',  -- saved, applied, interviewing, offer, rejected, withdrawn
    location        TEXT,                           -- e.g. 'Remote', 'New York, NY', 'Hybrid'
    date_applied    TEXT,                           -- ISO 8601 date string
    salary_range    TEXT,
    job_url         TEXT,
    contact_name    TEXT,                           -- Primary recruiter, hiring manager, or referral
    contact_email   TEXT,                           -- Primary contact email
    notes           TEXT,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS job_events (
    id            INTEGER PRIMARY KEY,
    job_id        INTEGER NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    event_type    TEXT NOT NULL,        -- status_change, note, interview, follow_up, offer
    title         TEXT,                 -- Optional summary (e.g. 'Recruiter Screen', 'Technical Round 1')
    from_status   TEXT,                 -- Optional for status changes
    to_status     TEXT,                 -- Optional for status changes
    contact_name  TEXT,                 -- Specific interviewer or contact for this event
    contact_email TEXT,                 -- Event contact email
    description   TEXT NOT NULL,        -- Detailed notes or rationale
    event_date    TEXT NOT NULL,        -- ISO 8601 date of the event
    created_at    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS renderings (
    id              INTEGER PRIMARY KEY,
    job_id          INTEGER NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    preset_name     TEXT,
    output_type     TEXT NOT NULL,                  -- cover_letter, cv
    format          TEXT NOT NULL DEFAULT 'plaintext', -- plaintext, typst (Phase 5)
    rendered_text   TEXT NOT NULL,
    context         TEXT NOT NULL,                  -- JSON serialized template context
    created_at      TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_company ON jobs(company);
CREATE INDEX IF NOT EXISTS idx_jobs_updated_at ON jobs(updated_at);
CREATE INDEX IF NOT EXISTS idx_renderings_job_id ON renderings(job_id);
CREATE INDEX IF NOT EXISTS idx_job_events_job_id_date ON job_events(job_id, event_date);

