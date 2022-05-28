use super::super::database::store_operation;
use rusqlite::Connection;
use std::result::Result;
use std::sync::Mutex;
use tauri::State;

/// Stores an operation and its result in the database
///
/// Arguments:
///
/// * `conn`: This is the global sqlite connection (tauri passses it to the function for us).
/// * `operation`: String - This is the operation that will be stored.
/// * `result`: f64 - This is the result of the operation.
///
/// Returns:
///
/// Nothing.
#[tauri::command]
pub fn store_operation_command(conn: State<Mutex<Connection>>, operation: &str, result: f64) {
  match store_operation(&conn.lock().unwrap(), operation, result) {
    Ok(_) => {}
    Err(e) => {
      println!("Error: {}", e);
    }
  }
}
