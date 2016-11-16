//! Environment.

use ::std::{fmt, error};
use super::builtins::*;

//----------------------------------------------------------------

/// Things that can go wrong while evaluating.
#[derive(Clone, Debug, Eq, PartialEq)]
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
	InternalCorruption,
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
	EnvError(EnvError),
}
impl From<EnvError> for Error {
	fn from(env_err: EnvError) -> Error {
		Error::EnvError(env_err)
	}
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
			Error::EnvError(_) => "env error",
		}
	}
	fn cause(&self) -> Option<&error::Error> {
		match *self {
			Error::EnvError(ref err) => Some(err),
			_ => None,
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
	("sin", builtin_sin),
	("sinh", builtin_sinh),
	("sqr", builtin_sqr),
	("sqrt", builtin_sqrt),
	("stdev", builtin_stdev),
	("sub", builtin_sub),
	("tan", builtin_tan),
	("tanh", builtin_tanh),
	("tau", builtin_tau),
	("var", builtin_var),
];

/// Environment lookup errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvError {
	/// The symbol wasn't found.
	NotFound,
	/// The symbol is a builtin function, but a value was expected.
	Builtin,
}
impl fmt::Display for EnvError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		error::Error::description(self).fmt(f)
	}
}
impl error::Error for EnvError {
	fn description(&self) -> &str {
		match *self {
			EnvError::NotFound => "not found",
			EnvError::Builtin => "builtin",
		}
	}
}

//----------------------------------------------------------------

use ::std::borrow::Cow;

/// The environment.
///
/// Stores the builtins available to expressions and the last answer.
pub trait Env {
	/// Lookup a builtin function by id.
	fn builtin(&self, id: &str) -> Result<BuiltinFn, EnvError>;
	/// Lookup a value by id.
	fn value(&self, id: &str) -> Result<Value, EnvError>;
	/// Set a variable by id.
	fn set(&mut self, id: Cow<'static, str>, value: Value) -> Result<(), EnvError>;
}

/// Basic environment.
///
/// Supports just the default builtins and saves the last variable.
#[derive(Clone, Debug, PartialEq)]
pub struct BasicEnv {
	pub ans: Value,
}
impl Default for BasicEnv {
	fn default() -> BasicEnv {
		BasicEnv {
			ans: 0f64,
		}
	}
}
impl Env for BasicEnv {
	fn builtin(&self, id: &str) -> Result<BuiltinFn, EnvError> {
		match DEFAULT_BUILTINS.binary_search_by_key(&id, |it| it.0) {
			Ok(index) => Ok(DEFAULT_BUILTINS[index].1),
			Err(_) => Err(EnvError::NotFound),
		}
	}
	fn value(&self, id: &str) -> Result<Value, EnvError> {
		if id == "ans" {
			Ok(self.ans)
		}
		else {
			self.builtin(id).and_then(|f| f(self, &mut []).map_err(|_| EnvError::Builtin))
		}
	}
	fn set(&mut self, id: Cow<'static, str>, value: Value) -> Result<(), EnvError> {
		if id.as_ref() == "ans" {
			self.ans = value;
			Ok(())
		}
		else {
			Err(EnvError::NotFound)
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn var() {
		let mut env = BasicEnv::default();
		env.set("ans".into(), 12.4).unwrap();
		assert_eq!(env.value("ans"), Ok(12.4));
		assert_eq!(env.value("pi"), Ok(::std::f64::consts::PI));
		assert_eq!(env.value("unknown"), Err(EnvError::NotFound));
		assert_eq!(env.value("mean"), Err(EnvError::Builtin));
		
		// Assert the default builtins are sorted
		let mut copy = super::DEFAULT_BUILTINS;
		copy.sort_by_key(|builtin| builtin.0);
		for (left, right) in Iterator::zip(copy[..].iter(), super::DEFAULT_BUILTINS[..].iter()) {
			assert_eq!(left.0, right.0);
		}
	}
}
