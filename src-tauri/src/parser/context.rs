use fnv::FnvHashMap;
use std::f64::consts;
use std::fmt;
use std::sync::Arc;

/// A trait of a source of variables (and constants) and functions for substitution into an
/// evaluated expression.
///
/// A simplest way to create a custom context provider is to use Context.
///
/// ```rust
/// use parser::{ContextProvider, Context};
///
/// let mut ctx = Context::new(); // built-ins
/// ctx.var("x", 2.); // insert a new variable
/// assert_eq!(ctx.get_var("pi"), Some(std::f64::consts::PI));
pub trait ContextProvider {
  fn get_var(&self, _: &str) -> Option<f64> {
    None
  }
  fn eval_func(&self, _: &str, _: &[f64]) -> Result<f64, FuncEvalError> {
    Err(FuncEvalError::UnknownFunction)
  }

  fn var<S: Into<String>>(&mut self, var: S, value: f64) -> &mut Self {
    self
  }
}

/// Function evaluation error.
#[derive(Debug, Clone, PartialEq)]
pub enum FuncEvalError {
  TooFewArguments,
  TooManyArguments,
  NumberArgs(usize),
  UnknownFunction,
}

impl fmt::Display for FuncEvalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FuncEvalError::UnknownFunction => write!(f, "Unknown function"),
      FuncEvalError::NumberArgs(i) => write!(f, "Expected {} arguments", i),
      FuncEvalError::TooFewArguments => write!(f, "Too few arguments"),
      FuncEvalError::TooManyArguments => write!(f, "Too many arguments"),
    }
  }
}

impl std::error::Error for FuncEvalError {
  fn description(&self) -> &str {
    match *self {
      FuncEvalError::UnknownFunction => "unknown function",
      FuncEvalError::NumberArgs(_) => "wrong number of function arguments",
      FuncEvalError::TooFewArguments => "too few function arguments",
      FuncEvalError::TooManyArguments => "too many function arguments",
    }
  }
}

#[doc(hidden)]
pub fn max_array(xs: &[f64]) -> f64 {
  xs.iter().fold(::std::f64::NEG_INFINITY, |m, &x| m.max(x))
}

#[doc(hidden)]
pub fn min_array(xs: &[f64]) -> f64 {
  xs.iter().fold(::std::f64::INFINITY, |m, &x| m.min(x))
}

#[doc(hidden)]
pub fn avg_array(xs: &[f64]) -> f64 {
  xs.iter().fold(0., |m, &x| m + x) / xs.len() as f64
}

#[doc(hidden)]
pub fn builtin<'a>() -> Context<'a> {
  // TODO: cache this (lazy_static)
  Context::new()
}

type ContextHashMap<K, V> = FnvHashMap<K, V>;

/// A structure for storing variables/constants and functions to be used in an expression.
///
/// # Example
///
/// ```rust
/// use parser::{eval_str_with_context, Context};
///
/// let mut ctx = Context::new(); // builtins
/// ctx.var("x", 3.)
///    .func("f", |x| 2. * x)
///    .funcn("sum", |xs| xs.iter().sum(), ..);
///
/// assert_eq!(eval_str_with_context("pi + sum(1., 2.) + f(x)", &ctx),
///            Ok(std::f64::consts::PI + 1. + 2. + 2. * 3.));
/// ```
#[derive(Clone)]
pub struct Context<'a> {
  vars: ContextHashMap<String, f64>,
  funcs: ContextHashMap<String, GuardedFunc<'a>>,
}

impl<'a> Context<'a> {
  /// Creates a context with built-in constants and functions.
  pub fn new() -> Context<'a> {
    thread_local!(static DEFAULT_CONTEXT: Context<'static> = {
        let mut ctx = Context::empty();
        ctx.var("pi", consts::PI);
        ctx.var("e", consts::E);

        ctx.func("sqrt", f64::sqrt);
        ctx.func("exp", f64::exp);
        ctx.func("ln", f64::ln);
        ctx.func("log10", f64::log10);
        ctx.func("abs", f64::abs);
        ctx.func("sin", f64::sin);
        ctx.func("cos", f64::cos);
        ctx.func("tan", f64::tan);
        ctx.func("asin", f64::asin);
        ctx.func("acos", f64::acos);
        ctx.func("atan", f64::atan);
        ctx.func("sinh", f64::sinh);
        ctx.func("cosh", f64::cosh);
        ctx.func("tanh", f64::tanh);
        ctx.func("asinh", f64::asinh);
        ctx.func("acosh", f64::acosh);
        ctx.func("atanh", f64::atanh);
        ctx.func("floor", f64::floor);
        ctx.func("ceil", f64::ceil);
        ctx.func("round", f64::round);
        ctx.func("signum", f64::signum);
        ctx.func2("atan2", f64::atan2);
        ctx.func2("logn", |base, num| f64::log(num, base));
        ctx.funcn("max", max_array, 1..);
        ctx.funcn("min", min_array, 1..);
        ctx.funcn("avg", avg_array, 1..);
        ctx
    });

    DEFAULT_CONTEXT.with(|ctx| ctx.clone())
  }

  /// Creates an empty contexts.
  pub fn empty() -> Context<'a> {
    Context {
      vars: ContextHashMap::default(),
      funcs: ContextHashMap::default(),
    }
  }

  pub fn clear(&mut self) {
    self.vars.clear();
  }

  /// Adds a new variable/constant.
  pub fn var<S: Into<String>>(&mut self, var: S, value: f64) -> &mut Self {
    self.vars.insert(var.into(), value);
    self
  }

  /// Adds a new function of one argument.
  pub fn func<S, F>(&mut self, name: S, func: F) -> &mut Self
  where
    S: Into<String>,
    F: Fn(f64) -> f64 + 'a + Send + Sync,
  {
    self.funcs.insert(
      name.into(),
      Arc::new(move |args: &[f64]| {
        if args.len() == 1 {
          Ok(func(args[0]))
        } else {
          Err(FuncEvalError::NumberArgs(1))
        }
      }),
    );
    self
  }

  /// Adds a new function of two arguments.
  pub fn func2<S, F>(&mut self, name: S, func: F) -> &mut Self
  where
    S: Into<String>,
    F: Fn(f64, f64) -> f64 + 'a + Send + Sync,
  {
    self.funcs.insert(
      name.into(),
      Arc::new(move |args: &[f64]| {
        if args.len() == 2 {
          Ok(func(args[0], args[1]))
        } else {
          Err(FuncEvalError::NumberArgs(2))
        }
      }),
    );
    self
  }

  /// Adds a new function of a variable number of arguments.
  ///
  /// `n_args` specifies the allowed number of variables by giving an exact number `n` or a range
  /// `n..m`, `..`, `n..`, `..m`. The range is half-open, exclusive on the right, as is common in
  /// Rust standard library.
  ///
  /// # Example
  ///
  /// ```rust
  /// let mut ctx = meval::Context::empty();
  ///
  /// // require exactly 2 arguments
  /// ctx.funcn("sum_two", |xs| xs[0] + xs[1], 2);
  ///
  /// // allow an arbitrary number of arguments
  /// ctx.funcn("sum", |xs| xs.iter().sum(), ..);
  /// ```
  pub fn funcn<S, F, N>(&mut self, name: S, func: F, n_args: N) -> &mut Self
  where
    S: Into<String>,
    F: Fn(&[f64]) -> f64 + 'a + Send + Sync,
    N: ArgGuard,
  {
    self.funcs.insert(name.into(), n_args.to_arg_guard(func));
    self
  }
}

impl<'a> Default for Context<'a> {
  fn default() -> Self {
    Context::new()
  }
}

impl<'a> ContextProvider for Context<'a> {
  fn get_var(&self, name: &str) -> Option<f64> {
    self.vars.get(name).cloned()
  }
  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    self
      .funcs
      .get(name)
      .map_or(Err(FuncEvalError::UnknownFunction), |f| f(args))
  }
}

impl<'a, T: ContextProvider> ContextProvider for &'a T {
  fn get_var(&self, name: &str) -> Option<f64> {
    (&**self).get_var(name)
  }

  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    (&**self).eval_func(name, args)
  }
}

impl<'a, T: ContextProvider> ContextProvider for &'a mut T {
  fn get_var(&self, name: &str) -> Option<f64> {
    (&**self).get_var(name)
  }

  fn eval_func(&self, name: &str, args: &[f64]) -> Result<f64, FuncEvalError> {
    (&**self).eval_func(name, args)
  }
}

type GuardedFunc<'a> = Arc<dyn Fn(&[f64]) -> Result<f64, FuncEvalError> + 'a + Send + Sync>;

/// Trait for types that can specify the number of required arguments for a function with a
/// variable number of arguments.
///
/// # Example
///
/// ```rust
/// let mut ctx = meval::Context::empty();
///
/// // require exactly 2 arguments
/// ctx.funcn("sum_two", |xs| xs[0] + xs[1], 2);
///
/// // allow an arbitrary number of arguments
/// ctx.funcn("sum", |xs| xs.iter().sum(), ..);
/// ```
pub trait ArgGuard {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a>;
}

impl ArgGuard for usize {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a> {
    Arc::new(move |args: &[f64]| {
      if args.len() == self {
        Ok(func(args))
      } else {
        Err(FuncEvalError::NumberArgs(1))
      }
    })
  }
}

impl ArgGuard for std::ops::RangeFrom<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a> {
    Arc::new(move |args: &[f64]| {
      if args.len() >= self.start {
        Ok(func(args))
      } else {
        Err(FuncEvalError::TooFewArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::RangeTo<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a> {
    Arc::new(move |args: &[f64]| {
      if args.len() < self.end {
        Ok(func(args))
      } else {
        Err(FuncEvalError::TooManyArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::Range<usize> {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a> {
    Arc::new(move |args: &[f64]| {
      if args.len() >= self.start && args.len() < self.end {
        Ok(func(args))
      } else if args.len() < self.start {
        Err(FuncEvalError::TooFewArguments)
      } else {
        Err(FuncEvalError::TooManyArguments)
      }
    })
  }
}

impl ArgGuard for std::ops::RangeFull {
  fn to_arg_guard<'a, F: Fn(&[f64]) -> f64 + 'a + Send + Sync>(self, func: F) -> GuardedFunc<'a> {
    Arc::new(move |args: &[f64]| Ok(func(args)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_context() {
    let y = 0.;

    let z = 3.;

    let mut ctx = Context::new();
    ctx.var("x", 1.).func("f", |x| x + y).func("g", |x| x + z);
    ctx.func2("g", |x, y| x + y);
    assert_eq!(ctx.get_var("x"), Some(1.));
    assert_eq!(ctx.get_var("y"), None);
    assert_eq!(ctx.eval_func("f", &[1.0]), Ok(1.));
    assert_eq!(ctx.eval_func("g", &[1.0, 2.0]), Ok(3.));
  }

  #[test]
  fn test_default_functions() {
    let ctx = Context::new();
    assert_eq!(ctx.eval_func("sqrt", &[4.]), Ok(2.));
    assert_eq!(ctx.eval_func("logn", &[10., 100.]), Ok(2.));
    assert_eq!(ctx.eval_func("logn", &[100., 10.]), Ok(0.5));
  }
}
