use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Operation {
  operation: String,
  result: f64,
}

/// Returns a list of past operations
///
/// Arguments:
///
/// * `conn`: &Connection - This is the connection to the database.
///
/// Returns:
///
/// The list of operations.
pub fn get_operation_history(conn: &Connection) -> Result<Vec<Operation>> {
  let mut stmt = conn.prepare("SELECT operation, result FROM operations ORDER BY id ASC;")?;
  let operation_iter = stmt.query_map(params![], |row| {
    Ok(Operation {
      operation: row.get(0)?,
      result: row.get(1)?,
    })
  })?;
  let mut operations = Vec::new();
  for operation in operation_iter {
    operations.push(operation?);
  }
  Ok(operations)
}

/// Stores an operation and its result in the database.
///
/// Arguments:
///
/// * `conn`: &Connection - this is the connection to the database.
/// * `operation`: The operation to store.
/// * `result`: The result of the operation
///
/// Returns:
///
/// An Operation struct containing the operation and result.
pub fn store_operation(conn: &Connection, operation: &str, result: f64) -> Result<Operation> {
  conn.execute(
    "INSERT INTO operations (operation, result) VALUES (?1, ?2);",
    params![operation, result],
  )?;
  Ok(Operation {
    operation: operation.to_string(),
    result: result,
  })
}
