use super::parsers::*;
use nom::error::ErrorKind;
use nom::Err::{Error, Failure, Incomplete};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
  /// A token that is not allowed at the given location (contains the location of the offending
  /// character in the source string).
  UnexpectedToken(usize),
  /// Missing right parentheses at the end of the source string (contains the number of missing
  /// parens).
  MissingRParen(i32),
  /// Missing operator or function argument at the end of the expression.
  MissingArgument,
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ParserError::UnexpectedToken(i) => write!(f, "Unexpected token at char {}.", i + 1),
      ParserError::MissingRParen(i) => write!(
        f,
        "Missing {} right parenthes{}.",
        i,
        if i == 1 { "is" } else { "es" }
      ),
      ParserError::MissingArgument => write!(f, "Missing argument at the end of expression."),
    }
  }
}

impl std::error::Error for ParserError {}

#[derive(Debug, Clone, Copy)]
enum TokenizerState {
  // accept any token that is an expression from the left: var, num, (, negpos
  LExpr,
  // accept any token that needs an expression on the left: fact, binop, ), comma
  AfterRExpr,
}

#[derive(Debug, Clone, Copy)]
enum ParentState {
  Subexpr,
  Func,
}

pub fn tokenize<S: AsRef<str>>(input: S) -> Result<Vec<Token>, ParserError> {
  use self::TokenizerState::*;
  let mut state = LExpr;
  // number of function arguments left
  let mut paren_stack = vec![];

  let mut res = vec![];

  let input = input.as_ref();
  let mut s = input;

  while !s.is_empty() {
    let r = match (state, paren_stack.last()) {
      (LExpr, _) => lexpr(s),
      (AfterRExpr, None) => after_rexpr_no_paren(s),
      (AfterRExpr, Some(&ParentState::Subexpr)) => after_rexpr(s),
      (AfterRExpr, Some(&ParentState::Func)) => after_rexpr_comma(s),
    };

    match r {
      Ok((rest, t)) => {
        match t {
          Token::LParen => {
            paren_stack.push(ParentState::Subexpr);
          }
          Token::Func(..) => {
            paren_stack.push(ParentState::Func);
          }
          Token::RParen => {
            paren_stack.pop().expect("The paren_stack is empty!");
          }
          Token::Var(_) | Token::Number(_) => {
            state = AfterRExpr;
          }
          Token::Binary(_) | Token::Comma => {
            state = LExpr;
          }
          _ => {}
        }
        res.push(t);
        s = rest;
      }
      Err(err) => match err {
        Error(_) => {
          return Err(ParserError::UnexpectedToken(input.len() - s.len()));
        }
        Incomplete(n) => {
          panic!("Incomplete tokenizer error: {:?} on input: {}", n, input);
        }
        Failure(e) => {
          println!("Failure: {:?}", e.code);
          match e.code {
            ErrorKind::Float => {
              return Err(ParserError::MissingArgument);
            }
            _ => {
              return Err(ParserError::UnexpectedToken(input.len() - s.len()));
            }
          }
        }
      },
    }
  }

  match state {
    LExpr => Err(ParserError::MissingArgument),
    _ if !paren_stack.is_empty() => Err(ParserError::MissingRParen(paren_stack.len() as i32)),
    _ => Ok(res),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenize() {
    use super::Operation::*;
    use super::Token::*;

    assert_eq!(tokenize("a"), Ok(vec![Var("a".into())]));

    assert_eq!(
      tokenize("2+4-6"),
      Ok(vec![
        Number(2f64),
        Binary(Plus),
        Number(4f64),
        Binary(Minus),
        Number(6f64)
      ])
    );

    assert_eq!(
      tokenize("-(5+4)"),
      Ok(vec![
        Unary(Minus),
        LParen,
        Number(5f64),
        Binary(Plus),
        Number(4f64),
        RParen
      ])
    );

    assert_eq!(
      tokenize("-4+(1-+3)"),
      Ok(vec![
        Number(-4f64),
        Binary(Plus),
        LParen,
        Number(1f64),
        Binary(Minus),
        Number(3f64),
        RParen
      ])
    );

    assert_eq!(
      tokenize("3*4^9/ 8!"),
      Ok(vec![
        Number(3f64),
        Binary(Times),
        Number(4f64),
        Binary(Pow),
        Number(9f64),
        Binary(Div),
        Number(8f64),
        Unary(Fact)
      ])
    );

    assert_eq!(
      tokenize("-2^ ab0 *12 - C_0"),
      Ok(vec![
        Number(-2f64),
        Binary(Pow),
        Var("ab0".into()),
        Binary(Times),
        Number(12f64),
        Binary(Minus),
        Var("C_0".into()),
      ])
    );

    assert_eq!(
      tokenize("-sin(pi * 3)^ cos(2) / Func2(x, f(y), z) * _buildIN(y)"),
      Ok(vec![
        Unary(Minus),
        Func("sin".into(), None),
        Var("pi".into()),
        Binary(Times),
        Number(3f64),
        RParen,
        Binary(Pow),
        Func("cos".into(), None),
        Number(2f64),
        RParen,
        Binary(Div),
        Func("Func2".into(), None),
        Var("x".into()),
        Comma,
        Func("f".into(), None),
        Var("y".into()),
        RParen,
        Comma,
        Var("z".into()),
        RParen,
        Binary(Times),
        Func("_buildIN".into(), None),
        Var("y".into()),
        RParen,
      ])
    );

    assert_eq!(
      tokenize("2 %   3"),
      Ok(vec![Number(2f64), Binary(Mod), Number(3f64)])
    );

    assert_eq!(tokenize("!2"), Err(ParserError::UnexpectedToken(0)));
    assert_eq!(tokenize("()"), Err(ParserError::UnexpectedToken(1)));
    assert_eq!(tokenize("2)"), Err(ParserError::UnexpectedToken(1)));

    assert_eq!(tokenize(""), Err(ParserError::MissingArgument));
    assert_eq!(tokenize("(5+4*(6"), Err(ParserError::MissingRParen(2)));
    assert_eq!(tokenize("(5+4*(6+("), Err(ParserError::MissingArgument));

    assert_eq!(tokenize("f(2,)"), Err(ParserError::UnexpectedToken(4)));
    assert_eq!(tokenize("f(,2)"), Err(ParserError::UnexpectedToken(2)));
  }
}
