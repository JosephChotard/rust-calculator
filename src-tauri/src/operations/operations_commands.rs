use super::super::parser::Context;
use super::operations_service::{
  calculate_result, check_if_command, clear_operation_history, get_operation_history, run_command,
  store_operation, Operation,
};
use rusqlite::Connection;
use std::result::Result;
use std::sync::Mutex;
use tauri::{State, Window};

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
  parser_context: State<Mutex<Context>>,
  window: Window,
  input: &str,
) -> Result<(), String> {
  match check_if_command(&input) {
    true => {
      run_command(
        &input,
        &conn.lock().unwrap(),
        &mut parser_context.lock().unwrap(),
        &window,
      );
      return Ok(());
    }
    false => {}
  };
  match store_operation(
    input,
    &mut conn.lock().unwrap(),
    &mut parser_context.lock().unwrap(),
    window,
  ) {
    Ok(_) => Ok(()),
    Err(err) => {
      return Err(err.to_string());
    }
  }
}

#[tauri::command]
pub fn get_result_command(
  input: &str,
  parser_context: State<Mutex<Context>>,
) -> Result<f64, String> {
  match check_if_command(&input) {
    true => return Err("command".to_string()),
    false => {}
  };
  match calculate_result(input, &parser_context.lock().unwrap()) {
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
pub fn clear_operation_history_command(
  conn: State<Mutex<Connection>>,
  parser_context: State<Mutex<Context>>,
) -> Result<(), String> {
  match clear_operation_history(&conn.lock().unwrap(), &mut parser_context.lock().unwrap()) {
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
