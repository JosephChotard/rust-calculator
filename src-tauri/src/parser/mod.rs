mod context;
mod expr;
mod extra_math;
pub mod parsers;
pub mod shunting_yard;
pub mod tokenizer;

use context::FuncEvalError;
pub use context::{builtin, Context, ContextProvider};
pub use expr::eval_str_with_context;
pub use parsers::{Operation, Token};
use shunting_yard::RPNError;
use std::fmt;
pub use tokenizer::{tokenize, ParserError};

/// An error produced during parsing or evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
  UnknownVariable(String),
  Function(String, FuncEvalError),
  /// An error returned by the parser.
  ParseError(ParserError),
  /// The shunting-yard algorithm returned an error.
  RPNError(RPNError),
  // A catch all for all other errors during evaluation
  EvalError(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::UnknownVariable(ref name) => {
        write!(f, "Evaluation error: unknown variable `{}`.", name)
      }
      Error::Function(ref name, ref e) => {
        write!(f, "Evaluation error: function `{}`: {}", name, e)
      }
      Error::ParseError(ref e) => {
        write!(f, "Parse error: ").expect("Could not write to formatter.");
        e.fmt(f)
      }
      Error::RPNError(ref e) => {
        write!(f, "RPN error: ").expect("Could not write to formatter.");
        e.fmt(f)
      }
      Error::EvalError(ref e) => {
        write!(f, "Eval error: ").expect("Could not write to formatter.");
        e.fmt(f)
      }
    }
  }
}

impl From<ParserError> for Error {
  fn from(err: ParserError) -> Error {
    Error::ParseError(err)
  }
}

impl From<RPNError> for Error {
  fn from(err: RPNError) -> Error {
    Error::RPNError(err)
  }
}

impl std::error::Error for Error {
  fn cause(&self) -> Option<&dyn std::error::Error> {
    match *self {
      Error::ParseError(ref e) => Some(e),
      Error::RPNError(ref e) => Some(e),
      Error::Function(_, ref e) => Some(e),
      _ => None,
    }
  }
}
