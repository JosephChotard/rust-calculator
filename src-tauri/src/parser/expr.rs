use super::context::ContextProvider;
use super::extra_math::factorial;
use super::parsers::{starts_with_assignment, Token};
use super::shunting_yard::to_rpn;
use super::tokenize;
use super::Error;
use std::str::FromStr;

/// Representation of a parsed expression.
///
/// The expression is internally stored in the [reverse Polish notation (RPN)][RPN] as a sequence
/// of `Token`s.
///
/// Methods `bind`, `bind_with_context`, `bind2`, ... can be used to create  closures from
/// the expression that then can be passed around and used as any other `Fn` closures.
///
/// ```rust
/// let func = "x^2".parse::<meval::Expr>().unwrap().bind("x").unwrap();
/// let r = Some(2.).map(func);
/// assert_eq!(r, Some(4.));
/// ```
///
/// [RPN]: https://en.wikipedia.org/wiki/Reverse_Polish_notation
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    rpn: Vec<Token>,
    assign_to: Option<String>,
}

impl Expr {
    // Evaluates the expression with the given context.
    pub fn eval_with_context<C: ContextProvider>(&self, ctx: C) -> Result<f64, Error> {
        use super::parsers::Operation::*;
        use super::parsers::Token::*;

        let mut stack = Vec::with_capacity(16);

        for token in &self.rpn {
            match *token {
                Var(ref n) => {
                    if let Some(v) = ctx.get_var(n) {
                        stack.push(v);
                    } else {
                        return Err(Error::UnknownVariable(n.clone()));
                    }
                }
                Number(f) => stack.push(f),
                Binary(op) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let r = match op {
                        Plus => left + right,
                        Minus => left - right,
                        Times => left * right,
                        Div => left / right,
                        Mod => left % right,
                        Pow => left.powf(right),
                        _ => {
                            return Err(Error::EvalError(format!(
                                "Unimplemented binary operation: {:?}",
                                op
                            )));
                        }
                    };
                    stack.push(r);
                }
                Unary(op) => {
                    let x = stack.pop().unwrap();
                    let r = match op {
                        Plus => x,
                        Minus => -x,
                        Fact => {
                            // Check to make sure x has no fractional component (can be converted to int without loss)
                            match factorial(x) {
                                Ok(res) => res,
                                Err(e) => return Err(Error::EvalError(String::from(e))),
                            }
                        }
                        _ => {
                            return Err(Error::EvalError(format!(
                                "Unimplemented unary operation: {:?}",
                                op
                            )));
                        }
                    };
                    stack.push(r);
                }
                Func(ref n, Some(i)) => {
                    if stack.len() < i {
                        return Err(Error::EvalError(format!(
                            "eval: stack does not have enough arguments for function token \
                             {:?}",
                            token
                        )));
                    }
                    match ctx.eval_func(n, &stack[stack.len() - i..]) {
                        Ok(r) => {
                            let nl = stack.len() - i;
                            stack.truncate(nl);
                            stack.push(r);
                        }
                        Err(e) => return Err(Error::Function(n.to_owned(), e)),
                    }
                }
                _ => return Err(Error::EvalError(format!("Unrecognized token: {:?}", token))),
            }
        }

        let r = stack.pop().expect("Stack is empty, this is impossible.");
        if !stack.is_empty() {
            return Err(Error::EvalError(format!(
                "There are still {} items on the stack.",
                stack.len()
            )));
        }
        Ok(r)
    }
}

/// Evaluates a string with the given context.
///
/// No built-ins are defined in this case.
pub fn eval_str_with_context<S: AsRef<str>, C: ContextProvider>(
    expr: S,
    ctx: C,
) -> Result<(Option<String>, f64), Error> {
    let expr = Expr::from_str(expr.as_ref())?;

    let res = expr.eval_with_context(&ctx);

    match res {
        Ok(r) => Ok((expr.assign_to, r)),
        Err(e) => Err(e),
    }
}

impl FromStr for Expr {
    type Err = Error;
    /// Constructs an expression by parsing a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (expr, var) = match starts_with_assignment(s) {
            Ok((expr, var)) => match var {
                Token::Var(name) => (expr, Some(name)),
                _ => (s, None),
            },
            Err(_) => (s, None),
        };
        let tokens = tokenize(expr)?;

        let rpn = to_rpn(&tokens)?;

        Ok(Expr {
            rpn: rpn,
            assign_to: var,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::context::builtin;
    use super::*;

    #[test]
    fn test_eval_str() {
        let context = builtin();
        assert_eq!(eval_str_with_context("2 + 3", &context), Ok((None, 5.)));
        assert_eq!(
            eval_str_with_context("2 + (3 + 4)", &context),
            Ok((None, 9.))
        );
        assert_eq!(
            eval_str_with_context("-2^(4 - 3) * (3 + 4)", &context),
            Ok((None, -14.))
        );
        assert_eq!(
            eval_str_with_context("-2*3! + 1", &context),
            Ok((None, -11.))
        );
        assert_eq!(
            eval_str_with_context("-171!", &context),
            Ok((None, std::f64::NEG_INFINITY))
        );
        assert_eq!(
            eval_str_with_context("150!/148!", &context),
            Ok((None, 22350.))
        );
        assert_eq!(
            eval_str_with_context("a + 3", &context),
            Err(Error::UnknownVariable("a".into()))
        );
        assert_eq!(
            eval_str_with_context("round(sin (pi) * cos(0))", &context),
            Ok((None, 0.))
        );
        assert_eq!(
            eval_str_with_context("round( sqrt(3^2 + 4^2)) ", &context),
            Ok((None, 5.))
        );
        assert_eq!(eval_str_with_context("max(1.)", &context), Ok((None, 1.)));
        assert_eq!(
            eval_str_with_context("max(1., 2., -1)", &context),
            Ok((None, 2.))
        );
        assert_eq!(
            eval_str_with_context("min(1., 2., -1)", &context),
            Ok((None, -1.))
        );
        assert_eq!(
            eval_str_with_context("sin(1.) + cos(2.)", &context),
            Ok((None, (1f64).sin() + (2f64).cos()))
        );
        assert_eq!(
            eval_str_with_context("10 % 9", &context),
            Ok((None, 10f64 % 9f64))
        );

        assert!(matches!(
            eval_str_with_context("0.5!", &context),
            Err(Error::EvalError { .. })
        ));
    }

    #[test]
    fn test_builtins() {
        let context = builtin();

        assert_eq!(
            eval_str_with_context("atan2(1.,2.)", &context),
            Ok((None, (1f64).atan2(2.)))
        );
        assert_eq!(
            eval_str_with_context("sqrt(8)", &context),
            Ok((None, 8f64.sqrt()))
        );
    }

    #[test]
    fn test_eval_func_ctx() {
        use super::super::context::Context;
        let y = 5.;
        assert_eq!(
            eval_str_with_context("phi(2.)", Context::new().func("phi", |x| x + y + 3.)),
            Ok((None, 2. + y + 3.))
        );
        assert_eq!(
            eval_str_with_context(
                "phi(2., 3.)",
                Context::new().func2("phi", |x, y| x + y + 3.)
            ),
            Ok((None, 2. + 3. + 3.))
        );
        assert_eq!(
            eval_str_with_context(
                "phi(2., 3.)",
                Context::new().funcn("phi", |xs: &[f64]| xs[0] + xs[1], 2)
            ),
            Ok((None, 2. + 3.))
        );

        assert_eq!(
            eval_str_with_context(
                "alpha = 54+34",
                Context::new().funcn("phi", |xs: &[f64]| xs[0] + xs[1], 2)
            ),
            Ok((Some("alpha".into()), 54. + 34.))
        );
    }

    #[test]
    fn test_variable_assignment() {
        assert_eq!(Expr::from_str("a = 2").unwrap().assign_to, Some("a".into()),);

        assert_eq!(
            Expr::from_str("variable = a+b+2").unwrap().assign_to,
            Some("variable".into()),
        );

        assert_eq!(Expr::from_str("a+b+2").unwrap().assign_to, None,);
    }
}
