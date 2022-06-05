use super::super::database::{
  clear_operation_history, get_operation_history, store_operation, Operation,
};
use super::super::maths::get_result;
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
  input: &str,
) -> Result<Operation, String> {
  match get_result(input) {
    Ok(result) => match store_operation(&conn.lock().unwrap(), input, result) {
      Ok(operation) => Ok(operation),
      Err(e) => Err(format!("{}", e).into()),
    },
    Err(err) => {
      return Err(err.to_string());
    }
  }
}

#[tauri::command]
pub fn get_result_command(input: &str) -> Result<f64, String> {
  match get_result(input) {
    Ok(result) => Ok(result),
    Err(err) => Err(err.to_string()),
  }
}

/// Clears the operation history
///
/// Arguments:
///
/// * `conn`: State<Mutex<Connection>> - this is the connection to the database.
///
/// Returns:
///
/// A Result<(), String>
#[tauri::command]
pub fn clear_operation_history_command(conn: State<Mutex<Connection>>) -> Result<(), String> {
  match clear_operation_history(&conn.lock().unwrap()) {
    Ok(_) => Ok(()),
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
