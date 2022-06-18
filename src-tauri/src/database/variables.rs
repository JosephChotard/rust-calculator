use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Variable {
  name: String,
  value: f64,
}

pub fn get_variables(conn: &Connection) -> Result<Vec<Variable>> {
  let mut stmt = conn.prepare("SELECT name, value FROM variables;")?;
  let variable_iter = stmt.query_map(params![], |row| {
    Ok(Variable {
      name: row.get(0)?,
      value: row.get(1)?,
    })
  })?;
  let mut variables = Vec::new();
  for variable in variable_iter {
    variables.push(variable?);
  }
  Ok(variables)
}

pub fn clear_variables(conn: &Connection) -> Result<()> {
  conn.execute("DELETE FROM variables;", params![])?;
  Ok(())
}

pub fn store_variable(conn: &Connection, name: &str, value: f64) -> Result<Variable> {
  conn.execute(
    "INSERT INTO variables (name, value) VALUES (?1, ?2);",
    params![name, value],
  )?;
  Ok(Variable {
    name: name.to_string(),
    value: value,
  })
}
