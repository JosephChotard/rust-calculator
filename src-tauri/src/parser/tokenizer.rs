use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::{map, recognize};
use nom::error::Error;
use nom::multi::many0_count;
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use std;
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
      ParserError::UnexpectedToken(i) => write!(f, "Unexpected token at byte {}.", i),
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

impl std::error::Error for ParserError {
  fn description(&self) -> &str {
    match *self {
      ParserError::UnexpectedToken(_) => "unexpected token",
      ParserError::MissingRParen(_) => "missing right parenthesis",
      ParserError::MissingArgument => "missing argument",
    }
  }
}

/// Mathematical operations.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operation {
  Plus,
  Minus,
  Times,
  Div,
  Mod,
  Pow,
  Fact,
}

/// Expression tokens.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  /// Binary operation.
  Binary(Operation),
  /// Unary operation.
  Unary(Operation),

  /// Left parenthesis.
  LParen,
  /// Right parenthesis.
  RParen,
  /// Comma: function argument separator
  Comma,

  /// A number.
  Number(f64),
  /// A variable.
  Var(String),
  /// A function with name and number of arguments.
  Func(String, Option<usize>),
}

#[derive(Debug, Clone, Copy)]
enum TokenizerState {
  // accept any token that is an expression from the left: var, num, (, negpos
  LExpr,
  // accept any token that needs an expression on the left: fact, binop, ), comma
  AfterRExpr,
}

pub fn tokenize<S: AsRef<str>>(input: S) -> Result<Vec<Token>, ParserError> {
  Ok(vec![])
}

fn number(input: &str) -> IResult<&str, Token> {
  match double::<&str, Error<&str>>(input) {
    Ok((rest, n)) => IResult::Ok((rest, Token::Number(n))),
    Err(e) => IResult::Err(e),
  }
}

// Parse func( returns func
fn func(input: &str) -> IResult<&str, Token> {
  match terminated(ident, preceded(multispace0, tag("(")))(input) {
    Ok((rest, name)) => IResult::Ok((rest, Token::Func(name.to_string(), None))),
    Err(e) => IResult::Err(e),
  }
}

fn var(input: &str) -> IResult<&str, Token> {
  match ident(input) {
    Ok((rest, name)) => IResult::Ok((rest, Token::Var(name.to_string()))),
    Err(e) => IResult::Err(e),
  }
}

fn ident(input: &str) -> IResult<&str, &str> {
  match recognize(pair(
    alt((alpha1, tag("_"))),
    many0_count(alt((alphanumeric1, tag("_")))),
  ))(input)
  {
    Ok((rest, parsed)) => IResult::Ok((rest, parsed)),
    Err(e) => IResult::Err(e),
  }
}

fn negpos(input: &str) -> IResult<&str, Token> {
  alt((
    map(tag("-"), |_| Token::Unary(Operation::Minus)),
    map(tag("+"), |_| Token::Unary(Operation::Plus)),
  ))(input)
}

fn binop(input: &str) -> IResult<&str, Token> {
  alt((
    map(tag("+"), |_| Token::Binary(Operation::Plus)),
    map(tag("-"), |_| Token::Binary(Operation::Minus)),
    map(tag("*"), |_| Token::Binary(Operation::Times)),
    map(tag("/"), |_| Token::Binary(Operation::Div)),
    map(tag("%"), |_| Token::Binary(Operation::Mod)),
    map(tag("^"), |_| Token::Binary(Operation::Pow)),
  ))(input)
}

fn lparen(input: &str) -> IResult<&str, Token> {
  map(tag("("), |_| Token::LParen)(input)
}

fn rparen(input: &str) -> IResult<&str, Token> {
  map(tag(")"), |_| Token::RParen)(input)
}

fn comma(input: &str) -> IResult<&str, Token> {
  map(tag(","), |_| Token::Comma)(input)
}

fn fact(input: &str) -> IResult<&str, Token> {
  map(tag("!"), |_| Token::Unary(Operation::Fact))(input)
}

fn lexpr(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((var, number, lparen, negpos)), multispace0)(input)
}

fn after_rexpr_no_paren(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((fact, binop)), multispace0)(input)
}

fn after_rexpr(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((fact, binop, rparen)), multispace0)(input)
}

fn after_rexpr_comma(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((fact, binop, rparen, comma)), multispace0)(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number() {
    assert_eq!(number(&"32143"), IResult::Ok(("", Token::Number(32143f64))));
    assert_eq!(number(&"2."), IResult::Ok(("", Token::Number(2.0f64))));
    assert_eq!(
      number(&"32143.25"),
      IResult::Ok(("", Token::Number(32143.25f64)))
    );
    assert_eq!(
      number(&"0.125e9"),
      IResult::Ok(("", Token::Number(0.125e9f64)))
    );
    assert_eq!(
      number(&"20.5E-3"),
      IResult::Ok(("", Token::Number(20.5E-3f64)))
    );
    assert_eq!(
      number(&"123423e+50"),
      IResult::Ok(("", Token::Number(123423e+50f64)))
    );
    assert_eq!(
      number(&"123423e-50something"),
      IResult::Ok(("something", Token::Number(123423e-50f64)))
    );
    assert_eq!(
      number(&"123text"),
      IResult::Ok(("text", Token::Number(123f64)))
    );
  }

  #[test]
  fn test_func_parse() {
    assert_eq!(
      func("func(1,2,3)"),
      IResult::Ok(("1,2,3)", Token::Func("func".to_string(), None)))
    );
    assert_eq!(
      func("func ("),
      IResult::Ok(("", Token::Func("func".to_string(), None)))
    );

    for &s in ["abc(", "u0(", "_034 (", "A_be45EA  ("].iter() {
      assert_eq!(
        func(s),
        IResult::Ok(("", Token::Func((&s[0..s.len() - 1]).trim().into(), None)))
      );
    }

    assert!(matches!(func("1,2 (3)"), IResult::Err { .. }));
  }

  #[test]
  fn test_var() {
    assert_eq!(var("abc"), IResult::Ok(("", Token::Var("abc".to_string()))));
    assert_eq!(var("_"), IResult::Ok(("", Token::Var("_".to_string()))));
    assert_eq!(
      var("_abc"),
      IResult::Ok(("", Token::Var("_abc".to_string())))
    );
    assert_eq!(
      var("_abc_"),
      IResult::Ok(("", Token::Var("_abc_".to_string())))
    );
    assert_eq!(
      var("_abc_123"),
      IResult::Ok(("", Token::Var("_abc_123".to_string())))
    );
    assert_eq!(
      var("_abc123"),
      IResult::Ok(("", Token::Var("_abc123".to_string())))
    );
    assert_eq!(
      var("_abc_123!"),
      IResult::Ok(("!", Token::Var("_abc_123".to_string())))
    );

    assert!(matches!(var("1"), IResult::Err { .. }));
    assert!(matches!(var("1.2"), IResult::Err { .. }));
    assert!(matches!(var("1a"), IResult::Err { .. }));
  }

  #[test]
  fn test_negpos() {
    assert_eq!(
      negpos("+"),
      IResult::Ok(("", Token::Unary(Operation::Plus)))
    );
    assert_eq!(
      negpos("-"),
      IResult::Ok(("", Token::Unary(Operation::Minus)))
    );
    assert_eq!(
      negpos("+-"),
      IResult::Ok(("-", Token::Unary(Operation::Plus)))
    );
    assert_eq!(
      negpos("-1233 + 5"),
      IResult::Ok(("1233 + 5", Token::Unary(Operation::Minus)))
    );
    assert!(matches!(negpos("1233 - 5"), IResult::Err { .. }));
  }

  #[test]
  fn test_simple() {
    assert_eq!(lparen("("), IResult::Ok(("", Token::LParen)));
    assert_eq!(lparen("(1"), IResult::Ok(("1", Token::LParen)));
    assert!(matches!(lparen("1"), IResult::Err { .. }));

    assert_eq!(rparen(")"), IResult::Ok(("", Token::RParen)));
    assert_eq!(rparen(")1"), IResult::Ok(("1", Token::RParen)));
    assert!(matches!(rparen("1"), IResult::Err { .. }));

    assert_eq!(fact("!"), IResult::Ok(("", Token::Unary(Operation::Fact))));
    assert_eq!(
      fact("!1"),
      IResult::Ok(("1", Token::Unary(Operation::Fact)))
    );
    assert!(matches!(fact("1"), IResult::Err { .. }));

    assert_eq!(comma(","), IResult::Ok(("", Token::Comma)));
    assert_eq!(comma(",1"), IResult::Ok(("1", Token::Comma)));
    assert!(matches!(comma("1"), IResult::Err { .. }));
  }

  #[test]
  fn test_binop() {
    assert_eq!(
      binop("+"),
      IResult::Ok(("", Token::Binary(Operation::Plus)))
    );
    assert_eq!(
      binop("-"),
      IResult::Ok(("", Token::Binary(Operation::Minus)))
    );
    assert_eq!(
      binop("*"),
      IResult::Ok(("", Token::Binary(Operation::Times)))
    );
    assert_eq!(binop("/"), IResult::Ok(("", Token::Binary(Operation::Div))));
    assert_eq!(binop("^"), IResult::Ok(("", Token::Binary(Operation::Pow))));
    assert_eq!(binop("%"), IResult::Ok(("", Token::Binary(Operation::Mod))));
  }

  #[ignore]
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
      tokenize("-4+(1-+3)"),
      Ok(vec![
        Unary(Minus),
        Number(4f64),
        Binary(Plus),
        LParen,
        Number(1f64),
        Binary(Minus),
        Unary(Plus),
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
        Unary(Minus),
        Number(2f64),
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
    assert_eq!(tokenize("(5+4*(6+("), Err(ParserError::MissingRParen(3)));

    assert_eq!(tokenize("f(2,)"), Err(ParserError::UnexpectedToken(4)));
    assert_eq!(tokenize("f(,2)"), Err(ParserError::UnexpectedToken(2)));
  }
}
