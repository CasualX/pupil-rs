use std::{error, fmt};

//----------------------------------------------------------------

/// Things that can go wrong.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
	/// Expected an operator-like thing.
	///
	/// Eg. `12 5`. Expected an operator instead of `5`.
	ExpectOperator,
	/// Expected a value-like thing.
	NaExpression,
	/// Disallowed unary operator.
	///
	/// Only `+` and `-` are allowed as unary operators.
	DisallowedUnary,
	/// Something went wrong unexpectedly.
	///
	/// This is a bug.
	InternalError,
	/// Expression isn’t finished, cannot end with an operator.
	///
	/// Eg. `2 +`.
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
	EnvErrorNotFound,
	/// Expected a variable name, found a builtin symbol instead.
	EnvErrorBuiltinFn,
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let desc = match self {
			Error::ExpectOperator => "expected an operator",
			Error::NaExpression => "not an expression",
			Error::DisallowedUnary => "not an unary operator",
			Error::InternalError => "internal corruption",
			Error::UnfinishedExpression => "unfinished expression",
			Error::InvalidToken => "invalid token",
			Error::UnbalancedParens => "unbalanced parens",
			Error::MisplacedComma => "misplaced comma",
			Error::BadArgument => "bad argument",
			Error::EnvErrorNotFound => "env error not found",
			Error::EnvErrorBuiltinFn => "env error builtin",
		};
		desc.fmt(f)
	}
}
impl error::Error for Error {}

//----------------------------------------------------------------

/// Underlying type used for arithmetic.
pub type Value = f64;

/// Signature for builtins.
pub type BuiltinFn = fn(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error>;

static DEFAULT_BUILTINS: [(&str, BuiltinFn); 53] = {
use crate::builtins::*;
[
	("", builtin_id),
	("abs", builtin_abs),
	("acos", builtin_cos),
	("acosh", builtin_acosh),
	("add", builtin_add),
	("asin", builtin_sin),
	("asinh", builtin_asinh),
	("atan", builtin_tan),
	("atan2", builtin_atan2),
	("atanh", builtin_atanh),
	("cbrt", builtin_cbrt),
	("ceil", builtin_ceil),
	("clamp", builtin_clamp),
	("cos", builtin_cos),
	("cosh", builtin_cosh),
	("cube", builtin_cube),
	("deg", builtin_deg),
	("div", builtin_div),
	("e", builtin_e),
	("exp", builtin_exp),
	("exp2", builtin_exp2),
	("expm1", builtin_expm1),
	("floor", builtin_floor),
	("ln", builtin_ln),
	("ln1p", builtin_ln1p),
	("log", builtin_log),
	("log10", builtin_log10),
	("log2", builtin_log2),
	("max", builtin_max),
	("mean", builtin_mean),
	("median", builtin_median),
	("min", builtin_min),
	("mul", builtin_mul),
	("pi", builtin_pi),
	("pow", builtin_pow),
	("rad", builtin_rad),
	("range", builtin_range),
	("rem", builtin_rem),
	("round", builtin_round),
	("signum", builtin_signum),
	("sin", builtin_sin),
	("sinh", builtin_sinh),
	("smootherstep", builtin_smootherstep),
	("smoothstep", builtin_smoothstep),
	("sqr", builtin_sqr),
	("sqrt", builtin_sqrt),
	("stdev", builtin_stdev),
	("step", builtin_step),
	("sub", builtin_sub),
	("tan", builtin_tan),
	("tanh", builtin_tanh),
	("tau", builtin_tau),
	("var", builtin_var),
]
};

/// The environment.
///
/// Stores the builtin functions and variables available to expressions.
pub trait Env {
	/// Lookup a builtin function.
	fn builtin(&self, name: &str) -> Result<BuiltinFn, Error>;
	/// Gets a variable’s value.
	fn get_value(&self, name: &str) -> Result<Value, Error>;
	/// Sets a variable’s value.
	fn set_value(&mut self, name: &str, value: Value) -> Result<(), Error>;
}

/// Basic environment.
///
/// Supports just the default builtins and saves the last answer.
#[derive(Clone)]
pub struct BasicEnv<'a> {
	pub ans: Value,
	pub builtins: &'a [(&'a str, BuiltinFn)],
}
impl<'a> Default for BasicEnv<'a> {
	fn default() -> BasicEnv<'a> {
		BasicEnv {
			ans: 0.0f64,
			builtins: &DEFAULT_BUILTINS,
		}
	}
}
impl<'a> Env for BasicEnv<'a> {
	fn builtin(&self, name: &str) -> Result<BuiltinFn, Error> {
		match self.builtins.binary_search_by_key(&name, |it| it.0) {
			Ok(index) => Ok(self.builtins[index].1),
			Err(_) => Err(Error::EnvErrorNotFound),
		}
	}
	fn get_value(&self, name: &str) -> Result<Value, Error> {
		match name {
			"ans" => Ok(self.ans),
			// Builtins which take zero arguments are treated as constants
			_ => self.builtin(name)?(self, &mut []).map_err(|_| Error::EnvErrorBuiltinFn),
		}
	}
	fn set_value(&mut self, name: &str, value: Value) -> Result<(), Error> {
		match name {
			"ans" => self.ans = value,
			_ => return Err(Error::EnvErrorNotFound),
		}
		Ok(())
	}
}

//----------------------------------------------------------------

#[test]
fn var() {
	let mut env = BasicEnv::default();
	env.set_value("ans", 12.4).unwrap();
	assert_eq!(env.get_value("ans"), Ok(12.4));
	assert_eq!(env.get_value("pi"), Ok(std::f64::consts::PI));
	assert_eq!(env.get_value("unknown"), Err(Error::EnvErrorNotFound));
	assert_eq!(env.get_value("mean"), Err(Error::EnvErrorBuiltinFn));
	
	// Assert the default builtins are sorted
	let mut copy = DEFAULT_BUILTINS;
	copy.sort_by_key(|builtin| builtin.0);
	for ((left, _), (right, _)) in Iterator::zip(copy.iter(), DEFAULT_BUILTINS.iter()) {
		assert_eq!(left, right);
	}
}
