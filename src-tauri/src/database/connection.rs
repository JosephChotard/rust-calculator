use directories::ProjectDirs;
use rusqlite::{params, Connection, Result};
use std::fs;

pub fn get_connection() -> Result<Connection> {
  let project_dirs = ProjectDirs::from("com", "josephchotard", "calculator").unwrap();

  // Create project directory if it doesn't exist
  fs::create_dir_all(project_dirs.data_dir()).expect("Could not create project directory");

  let db_path = project_dirs.data_dir().join("calculator.db");

  let conn = Connection::open(db_path.to_str().unwrap())?;

  create_initial_tables(&conn)?;
  Ok(conn)
}

fn create_initial_tables(conn: &Connection) -> Result<()> {
  conn.execute(
    "CREATE TABLE IF NOT EXISTS operations (id integer NOT NULL, operation text NOT NULL, result num NOT NULL, created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (id));",
    params![]
  )?;
  conn.execute(
    "CREATE TABLE IF NOT EXISTS variables (name text NOT NULL,value num DEFAULT NULL, PRIMARY KEY (name));",
    params![],
  )?;
  Ok(())
}
