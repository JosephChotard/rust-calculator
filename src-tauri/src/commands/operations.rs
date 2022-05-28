use super::super::database::{get_operation_history, store_operation, Operation};
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
/// * `Operation` - This is the operation that was stored.
#[tauri::command]
pub fn store_operation_command(
  conn: State<Mutex<Connection>>,
  operation: &str,
  result: f64,
) -> Result<Operation, String> {
  match store_operation(&conn.lock().unwrap(), operation, result) {
    Ok(operation) => Ok(operation),
    Err(e) => Err(format!("{}", e).into()),
  }
}

/// Returns all the operations stored in the database
///
/// Arguments:
///
/// * `conn`: State<Mutex<Connection>> - this is the connection to the database.
///
/// Returns:
///
/// A vector of operations.
#[tauri::command]
pub fn get_operation_history_command(conn: State<Mutex<Connection>>) -> Vec<Operation> {
  match get_operation_history(&conn.lock().unwrap()) {
    Ok(operations) => operations,
    Err(e) => {
      println!("Error: {}", e);
      Vec::new()
    }
  }
}
