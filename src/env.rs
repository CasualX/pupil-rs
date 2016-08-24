//! Environment.

use ::std::collections::HashMap;
use ::std::borrow::Cow;
use ::std::{fmt, error};
use super::builtins::*;

//----------------------------------------------------------------

/// Things that can go wrong while evaluating.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
	/// Expected an operator-like thing.
	ExpectOperator,
	/// Expected a value-like thing.
	NaExpression,
	/// Disallowed unary operator.
	DisallowedUnary,
	/// Something went wrong unexpectedly.
	InternalCorruption,
	/// Expression isn’t finished, cannot end with an operator.
	UnfinishedExpression,
	/// Tokenization failed to lex a token.
	InvalidToken,
	/// Different number of `(` and `)`.
	UnbalancedParens,
	/// Misplaced a comma token outside of a function application.
	MisplacedComma,
	/// Bad number of arguments.
	BadArgument,
	/// A variable or function symbol wasn’t found.
	UnknownSymbol,
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		error::Error::description(self).fmt(f)
	}
}
impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::ExpectOperator => "expected an operator",
			Error::NaExpression => "not an expression",
			Error::DisallowedUnary => "not an unary operator",
			Error::InternalCorruption => "internal corruption",
			Error::UnfinishedExpression => "unfinished expression",
			Error::InvalidToken => "invalid token",
			Error::UnbalancedParens => "unbalanced parens",
			Error::MisplacedComma => "misplaced comma",
			Error::BadArgument => "bad argument",
			Error::UnknownSymbol => "unknown symbol",
		}
	}
}

//----------------------------------------------------------------

/// Underlying type used for arithmetic.
pub type Value = f64;

/// Signature for builtins.
pub type BuiltinFn = fn(env: &Env, vals: &mut [Value]) -> Result<Value, Error>;

static DEFAULT_BUILTINS: [(&'static str, BuiltinFn); 48] = [
	("", builtin_id as BuiltinFn),
	("add", builtin_add),
	("sub", builtin_sub),
	("mul", builtin_mul),
	("div", builtin_div),
	("rem", builtin_rem),
	("pow", builtin_pow),
	("floor", builtin_floor),
	("ceil", builtin_ceil),
	("round", builtin_round),
	("abs", builtin_abs),
	("sqr", builtin_sqr),
	("cube", builtin_cube),
	("sqrt", builtin_sqrt),
	("cbrt", builtin_cbrt),
	("min", builtin_min),
	("max", builtin_max),
	("exp", builtin_exp),
	("exp2", builtin_exp2),
	("expm1", builtin_expm1),
	("ln", builtin_ln),
	("log", builtin_log),
	("log2", builtin_log2),
	("log10", builtin_log10),
	("ln1p", builtin_ln1p),
	("e", builtin_e),
	("mean", builtin_mean),
	("median", builtin_median),
	("range", builtin_range),
	("var", builtin_var),
	("stdev", builtin_stdev),
	("deg", builtin_deg),
	("rad", builtin_rad),
	("pi", builtin_pi),
	("tau", builtin_tau),
	("sin", builtin_sin),
	("cos", builtin_cos),
	("tan", builtin_tan),
	("asin", builtin_sin),
	("acos", builtin_cos),
	("atan", builtin_tan),
	("atan2", builtin_atan2),
	("sinh", builtin_sinh),
	("cosh", builtin_cosh),
	("tanh", builtin_tanh),
	("asinh", builtin_asinh),
	("acosh", builtin_acosh),
	("atanh", builtin_atanh),
];

/// The environment.
///
/// Stores the builtins available to expressions and the last answer.
pub struct Env {
	builtins: HashMap<&'static str, BuiltinFn>,
	vars: HashMap<Cow<'static, str>, Value>,
}

impl Env {
	/// Create a new environment.
	pub fn new() -> Env {
		Env {
			builtins: HashMap::new(),
			vars: HashMap::new(),
		}
	}
	pub fn init(&mut self) {
		for builtin in &DEFAULT_BUILTINS[..] {
			self.builtins.insert(builtin.0, builtin.1);
		}
	}
	/// Find a builtin.
	pub fn find(&self, id: &str) -> Option<BuiltinFn> {
		self.builtins.get(id).map(|&x| x)
	}
	/// Find a variable.
	pub fn var(&self, id: &str) -> Option<Value> {
		// For now constants `pi`, `e` and `tau` are builtins that take no arguments...
		// If this fails, look up in the `self.vars` hashmap.
		self.builtins.get(id)
			.and_then(|&x| x(self, &mut []).ok())
			.or_else(|| self.vars.get(id).map(|&x| x))
	}
	/// Set a variable.
	pub fn set_var<I: Into<Cow<'static, str>>>(&mut self, id: I, val: Value) {
		// If this isn't inlined it should help with monomorphization.
		fn internal_set_var(env: &mut Env, id: Cow<'static, str>, val: Value) {
			*env.vars.entry(id).or_insert(val) = val;
		}
		internal_set_var(self, id.into(), val);
	}
}
impl Default for Env {
	fn default() -> Env {
		let mut env = Env::new();
		env.init();
		env
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn var() {
		let mut env = Env::default();
		// Regular variables.
		env.set_var("ans", 12.4);
		assert_eq!(env.var("ans"), Some(12.4));
		// Constants can be looked up.
		assert_eq!(env.var("pi"), Some(::std::f64::consts::PI));
	}
}
