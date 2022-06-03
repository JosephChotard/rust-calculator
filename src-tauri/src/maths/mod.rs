use super::parser::{eval_str, Error};

pub fn get_result<S: AsRef<str>>(input: S) -> Result<f64, Error> {
  eval_str(input)
}
