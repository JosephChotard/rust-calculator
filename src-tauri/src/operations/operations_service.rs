use super::super::parser::{builtin, eval_str_with_context, Context, Error as ParserError};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use tauri::Window;

pub fn create_context_from_db<'a>(conn: &Connection) -> Context<'a> {
  let mut context = builtin();

  let mut stmt = conn
    .prepare("SELECT name, value FROM variables")
    .expect("Could not prepare statement");

  let mut rows = stmt
    .query_map(params![], |row| {
      let name: String = row.get(0).expect("Could not get name");
      let value: f64 = row.get(1).expect("Could not get value");
      Ok((name, value))
    })
    .expect("Could not query variables");
  while let Some(Ok((name, value))) = rows.next() {
    context.var(name, value);
  }

  context
}

pub fn check_if_command(input: &str) -> bool {
  let commands = vec!["clear", "exit"];
  commands.contains(&input)
}

pub fn run_command(input: &str, conn: &Connection, context: &mut Context, window: &Window) {
  match input {
    "clear" => {
      match clear_operation_history(conn, context) {
        Ok(_) => {
          window
            .emit("history_cleared", {})
            .expect("Could not emit event");
        }
        Err(e) => println!("Error: {}", e),
      };
    }
    "exit" => {}
    _ => {}
  };
}

// pub fn run_command(input: &str) -> bool {

// }
/// It takes a string and a context, and returns a result
///
/// Arguments:
///
/// * `input`: The string to parse.
/// * `context`: Contains the variables and functions that are available to the parser.
///
/// Returns:
///
/// The operation result or an error
pub fn calculate_result<S: AsRef<str>>(input: S, context: &Context) -> Result<f64, ParserError> {
  match eval_str_with_context(input, context) {
    Ok((_, result)) => Ok(result),
    Err(e) => Err(e),
  }
}

/// It takes a string, evaluates it, and if it's a variable assignment, saves the variable in the
/// context
///
/// Arguments:
///
/// * `input`: The input string to parse.
/// * `context`: The context to use for the evaluation.
///
/// Returns:
///
/// The operation result or an error
pub fn save_variable<S: AsRef<str>>(
  input: S,
  context: &mut Context,
  conn: &Connection,
) -> Result<f64, ParserError> {
  match eval_str_with_context(input, &context) {
    Ok((variable, result)) => {
      match variable {
        Some(name) => {
          context.var(&name, result);
          conn
            .execute(
              "INSERT OR REPLACE INTO variables (name, value) VALUES (?1, ?2)",
              params![name, result],
            )
            .expect("Could not insert variable");
        }
        None => {}
      };
      conn
        .execute(
          "INSERT OR REPLACE INTO variables (name, value) VALUES (?1, ?2)",
          params!["ans", result],
        )
        .expect("Could not save ans");
      context.var("ans", result);
      Ok(result)
    }
    Err(e) => Err(e),
  }
}

/// Delete all operations from the database
///
/// Arguments:
///
/// * `conn`: &Connection - this is the connection to the database.
///
/// Returns:
///
/// Nothing.
pub fn clear_operation_history(conn: &Connection, context: &mut Context) -> Result<()> {
  conn.execute("DELETE FROM operations;", params![])?;
  conn.execute("DELETE FROM variables;", params![])?;
  context.clear();
  Ok(())
}

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
/// * `operation`: The operation to store.
/// * `conn`: this is the connection to the database.
/// * `context`: The math context
///
/// Returns:
///
/// An Operation struct containing the operation and result.
pub fn store_operation(
  operation: &str,
  conn: &mut Connection,
  context: &mut Context,
  window: Window,
) -> Result<Operation, ParserError> {
  match save_variable(operation, context, conn) {
    Ok(result) => {
      conn
        .execute(
          "INSERT INTO operations (operation, result) VALUES (?1, ?2);",
          params![operation, result],
        )
        .expect("Could not store in database");
      let op = Operation {
        operation: operation.to_string(),
        result: result,
      };
      window
        .emit("add_to_history", &op)
        .expect("Could not emit add_to_history");
      Ok(op)
    }
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rusqlite::{params, Connection};

  fn create_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Could not create in-memory database");
    conn.execute(
      "CREATE TABLE IF NOT EXISTS operations (id integer NOT NULL, operation text NOT NULL, result num NOT NULL, PRIMARY KEY (id));",
      params![]
    ).expect("Could not create table operations");

    conn.execute(
      "CREATE TABLE IF NOT EXISTS variables (name text NOT NULL,value num DEFAULT NULL, PRIMARY KEY (name));",
      params![],
    ).expect("Could not create table variables");

    // Iniitialize the db with some data
    conn
      .execute(
        "INSERT INTO operations (operation, result) VALUES ('1', 1);",
        params![],
      )
      .expect("Could not insert into operations");
    conn
      .execute(
        "INSERT INTO operations (operation, result) VALUES ('2', 2);",
        params![],
      )
      .expect("Could not insert into operations");
    conn
      .execute(
        "INSERT INTO variables (name, value) VALUES ('a', 1);",
        params![],
      )
      .expect("Could not insert into variables");
    conn
      .execute(
        "INSERT INTO variables (name, value) VALUES ('var1', 2);",
        params![],
      )
      .expect("Could not insert into variables");
    conn
      .execute(
        "INSERT INTO variables (name, value) VALUES ('c', 24.65);",
        params![],
      )
      .expect("Could not insert into variables");
    conn
  }

  #[test]
  fn test_clear_history() {
    use super::super::super::parser::ContextProvider;
    let conn = create_db();

    let mut context = create_context_from_db(&conn);

    // Clear the history
    clear_operation_history(&conn, &mut context).expect("Could not clear history");

    // Assert that the context and db is empty
    let operation_count: usize = conn
      .query_row("SELECT COUNT(*) FROM operations;", [], |r| r.get(0))
      .expect("Could not get operation count");
    assert_eq!(operation_count, 0);
    let variable_count: usize = conn
      .query_row("SELECT COUNT(*) FROM variables;", [], |r| r.get(0))
      .expect("Could not get variable count");
    assert_eq!(variable_count, 0);

    assert_eq!(context.get_var("a"), None);
    assert_eq!(context.get_var("var1"), None);
    assert_eq!(context.get_var("c"), None);
  }

  #[test]
  fn test_create_context_from_db() {
    use super::super::super::parser::ContextProvider;

    let conn = create_db();

    let context = create_context_from_db(&conn);

    // Assert that the context and db is not empty
    let operation_count: usize = conn
      .query_row("SELECT COUNT(*) FROM operations;", [], |r| r.get(0))
      .expect("Could not get operation count");
    assert_eq!(operation_count, 2);
    let variable_count: usize = conn
      .query_row("SELECT COUNT(*) FROM variables;", [], |r| r.get(0))
      .expect("Could not get variable count");
    assert_eq!(variable_count, 3);

    assert_eq!(context.get_var("a"), Some(1.0));
    assert_eq!(context.get_var("var1"), Some(2.0));
    assert_eq!(context.get_var("c"), Some(24.65));
  }
}
