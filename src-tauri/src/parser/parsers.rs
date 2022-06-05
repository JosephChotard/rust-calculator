use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::{map, recognize};
use nom::multi::many0_count;
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

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

fn number(input: &str) -> IResult<&str, Token> {
  map(double, |n| Token::Number(n))(input)
}

// Parse func( returns func
fn func(input: &str) -> IResult<&str, Token> {
  map(terminated(ident, preceded(multispace0, tag("("))), |name| {
    Token::Func(name.to_string(), None)
  })(input)
}

fn var(input: &str) -> IResult<&str, Token> {
  map(ident, |s| Token::Var(s.to_string()))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
  recognize(pair(
    alt((alpha1, tag("_"))),
    many0_count(alt((alphanumeric1, tag("_")))),
  ))(input)
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
    map(alt((tag("^"), tag("**"))), |_| {
      Token::Binary(Operation::Pow)
    }),
    map(tag("*"), |_| Token::Binary(Operation::Times)),
    map(tag("/"), |_| Token::Binary(Operation::Div)),
    map(tag("%"), |_| Token::Binary(Operation::Mod)),
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

pub fn lexpr(input: &str) -> IResult<&str, Token> {
  delimited(
    multispace0,
    alt((number, func, var, negpos, lparen)),
    multispace0,
  )(input)
}

pub fn after_rexpr_no_paren(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((fact, binop)), multispace0)(input)
}

pub fn after_rexpr(input: &str) -> IResult<&str, Token> {
  delimited(multispace0, alt((fact, binop, rparen)), multispace0)(input)
}

pub fn after_rexpr_comma(input: &str) -> IResult<&str, Token> {
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
    assert_eq!(
      binop("**"),
      IResult::Ok(("", Token::Binary(Operation::Pow)))
    );
    assert_eq!(binop("%"), IResult::Ok(("", Token::Binary(Operation::Mod))));
  }
}
