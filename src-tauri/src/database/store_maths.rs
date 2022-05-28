use rusqlite::{params, Connection, Result};

pub fn store_operation(conn: &Connection, operation: &str, result: f64) -> Result<()> {
  conn.execute(
    "INSERT INTO operations (operation, result) VALUES (?1, ?2);",
    params![operation, result],
  )?;
  Ok(())
}
