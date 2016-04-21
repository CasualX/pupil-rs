//! Environment.

use ::std::collections::HashMap;
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

/// The environment.
///
/// Stores the builtins available to expressions and the last answer.
pub struct Env {
	map: HashMap<&'static str, BuiltinFn>,
	pub ans: Value,
}

impl Env {
	/// Create a new environment.
	pub fn new() -> Env {
		Env {
			map: HashMap::new(),
			ans: 0.0,
		}
	}
	pub fn init(&mut self) {
		self.map.insert("", builtin_id);
		self.map.insert("ans", builtin_ans);
		self.map.insert("add", builtin_add);
		self.map.insert("sub", builtin_sub);
		self.map.insert("mul", builtin_mul);
		self.map.insert("div", builtin_div);
		self.map.insert("rem", builtin_rem);
		self.map.insert("pow", builtin_pow);
		self.map.insert("floor", builtin_floor);
		self.map.insert("ceil", builtin_ceil);
		self.map.insert("round", builtin_round);
		self.map.insert("abs", builtin_abs);
		self.map.insert("sqr", builtin_sqr);
		self.map.insert("cube", builtin_cube);
		self.map.insert("sqrt", builtin_sqrt);
		self.map.insert("cbrt", builtin_cbrt);
		self.map.insert("min", builtin_min);
		self.map.insert("max", builtin_max);
		self.map.insert("exp", builtin_exp);
		self.map.insert("exp2", builtin_exp2);
		self.map.insert("expm1", builtin_expm1);
		self.map.insert("ln", builtin_ln);
		self.map.insert("log", builtin_log);
		self.map.insert("log2", builtin_log2);
		self.map.insert("log10", builtin_log10);
		self.map.insert("ln1p", builtin_ln1p);
		self.map.insert("e", builtin_e);
		self.map.insert("mean", builtin_mean);
		self.map.insert("median", builtin_median);
		self.map.insert("range", builtin_range);
		self.map.insert("var", builtin_var);
		self.map.insert("stdev", builtin_stdev);
		self.map.insert("deg", builtin_deg);
		self.map.insert("rad", builtin_rad);
		self.map.insert("pi", builtin_pi);
		self.map.insert("tau", builtin_tau);
		self.map.insert("sin", builtin_sin);
		self.map.insert("cos", builtin_cos);
		self.map.insert("tan", builtin_tan);
		self.map.insert("asin", builtin_sin);
		self.map.insert("acos", builtin_cos);
		self.map.insert("atan", builtin_tan);
		self.map.insert("atan2", builtin_atan2);
		self.map.insert("sinh", builtin_sin);
		self.map.insert("cosh", builtin_cos);
		self.map.insert("tanh", builtin_tan);
		self.map.insert("asinh", builtin_sin);
		self.map.insert("acosh", builtin_cos);
		self.map.insert("atanh", builtin_tan);
	}
	/// Find a builtin.
	pub fn find(&self, id: &str) -> Option<BuiltinFn> {
		self.map.get(id).map(|&x| x)
	}
}
impl Default for Env {
	fn default() -> Env {
		let mut env = Env::new();
		env.init();
		env
	}
}
