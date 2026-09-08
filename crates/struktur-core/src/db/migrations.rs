use rusqlite::{Connection, Error};

const MIGRATIONS: &[&str] = &[include_str!("./migrations/0001_initial_schema.sql")];

pub fn run(conn: &mut Connection) -> Result<(), Error> {
    let current_version: usize = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;

    for (i, sql) in MIGRATIONS.iter().enumerate().skip(current_version) {
        let target_version = (i + 1) as u32;
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", target_version)?;
        tx.commit()?;
    }

    Ok(())
}
