use std::path::PathBuf;

use rusqlite::Connection;

use crate::storage::get_project_dirs;

mod migrations;
pub mod models;

const DATABASE_FILE_NAME: &str = "jobs.db";

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("RusqliteError: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IOError: {0}")]
    Io(#[from] std::io::Error),
}

pub fn open() -> Result<Connection, DatabaseError> {
    let path = get_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(DatabaseError::Io)?;
    }
    let mut conn = rusqlite::Connection::open(path)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    migrations::run(&mut conn).map_err(DatabaseError::Db)?;

    Ok(conn)
}

pub fn get_path() -> Result<PathBuf, DatabaseError> {
    let base = get_project_dirs()
        .ok_or(DatabaseError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unable to find a HOME directory",
        )))?
        .data_dir()
        .to_owned();
    Ok(base.join(DATABASE_FILE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrations_are_idempotent() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        // First run
        migrations::run(&mut conn).unwrap();
        let version: u32 = conn
            .query_row("PRAGMA user_version;", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, 1);

        // Second run should be a safe no-op
        migrations::run(&mut conn).unwrap();
        let version: u32 = conn
            .query_row("PRAGMA user_version;", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, 1);
    }
}
