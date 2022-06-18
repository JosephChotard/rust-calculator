use super::parser::{eval_str_with_context, Context, Error};

pub fn evaluate<S: AsRef<str>>(input: S, context: &Context) -> Result<f64, Error> {
  match eval_str_with_context(input, context) {
    Ok((_, result)) => Ok(result),
    Err(e) => Err(e),
  }
}

/// It takes an expression, evaluates it, stores the variable if present and returns the result.
///
/// Arguments:
///
/// * `input`: The string to evaluate.
/// * `context`: The context to use for the evaluation.
///
/// Returns:
///
/// The result
pub fn get_result<S: AsRef<str>>(input: S, context: &mut Context) -> Result<f64, Error> {
  match eval_str_with_context(input, &context) {
    Ok((variable, result)) => {
      match variable {
        Some(name) => {
          context.var(name, result);
          // store_variable(name, result);
        }
        None => {}
      };
      Ok(result)
    }
    Err(e) => Err(e),
  }
}
