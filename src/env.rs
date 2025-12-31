use super::*;

/// Environment interface.
///
/// Stores the builtins available to expressions.
pub trait Env {
	/// Lookup a native function.
	fn function(&self, name: &str) -> Result<Function, ErrorKind>;
	/// Gets a variable’s value.
	fn value(&self, name: &str) -> Result<Value, ErrorKind>;
	/// Sets a variable’s value.
	fn set_value(&mut self, name: &str, value: Value) -> Result<(), ErrorKind>;
}

//----------------------------------------------------------------

/// Underlying type used for arithmetic.
pub type Value = f64;

/// Signature for native functions.
pub type Function = fn(env: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind>;

/// Looks up a native function by name.
pub fn function(name: &str) -> Option<Function> {
	let function = match name {
		"" => native::id,

		"add" => native::add,
		"sub" => native::sub,
		"mul" => native::mul,
		"div" => native::div,
		"rem" => native::rem,
		"pow" => native::pow,

		"round" => native::round,
		"floor" => native::floor,
		"ceil" => native::ceil,
		"trunc" => native::trunc,
		"fract" => native::fract,

		"abs" => native::abs,
		"sign" => native::sign,
		"sqr" => native::sqr,
		"sqrt" => native::sqrt,
		"cube" => native::cube,
		"cbrt" => native::cbrt,
		"isinf" => native::isinf,
		"isnan" => native::isnan,

		"min" => native::min,
		"max" => native::max,
		"clamp" => native::clamp,

		"step" => native::step,
		"smoothstep" => native::smoothstep,
		"smootherstep" => native::smootherstep,

		"eq" => native::eq,
		"ne" => native::ne,
		"gt" => native::gt,
		"ge" => native::ge,
		"lt" => native::lt,
		"le" => native::le,

		"all" => native::all,
		"any" => native::any,
		"not" => native::not,
		"select" => native::select,

		"exp" => native::exp,
		"exp2" => native::exp2,
		"expm1" => native::expm1,
		"log" => native::log,
		"log10" => native::log10,
		"log2" => native::log2,
		"ln" => native::ln,
		"ln1p" => native::ln1p,

		"mean" => native::mean,
		"median" => native::median,
		"range" => native::range,
		"var" => native::var,
		"stdev" => native::stdev,

		"deg" => native::deg,
		"rad" => native::rad,
		"sin" => native::sin,
		"cos" => native::cos,
		"tan" => native::tan,
		"asin" => native::asin,
		"acos" => native::acos,
		"atan" => native::atan,
		"atan2" => native::atan2,

		"sinh" => native::sinh,
		"cosh" => native::cosh,
		"tanh" => native::tanh,
		"asinh" => native::asinh,
		"acosh" => native::acosh,
		"atanh" => native::atanh,

		_ => return None,
	};
	Some(function)
}

/// Basic environment.
///
/// Supports just the default builtins and saves the last answer.
#[derive(Clone, Default)]
pub struct BasicEnv {
	pub ans: Value,
}

impl Env for BasicEnv {
	fn function(&self, name: &str) -> Result<Function, ErrorKind> {
		function(name).ok_or(ErrorKind::NameNotFound)
	}
	fn value(&self, name: &str) -> Result<Value, ErrorKind> {
		let value = match name {
			"ans" => self.ans,
			"e" => f64::consts::E,
			"pi" => f64::consts::PI,
			"tau" => f64::consts::TAU,
			_ => return Err(ErrorKind::NameNotFound),
		};
		Ok(value)
	}
	fn set_value(&mut self, name: &str, value: Value) -> Result<(), ErrorKind> {
		match name {
			"ans" => self.ans = value,
			_ => return Err(ErrorKind::NameNotFound),
		}
		Ok(())
	}
}

//----------------------------------------------------------------

#[test]
fn var() {
	let mut env = BasicEnv::default();
	env.set_value("ans", 12.4).unwrap();
	assert_eq!(env.value("ans"), Ok(12.4));
	assert_eq!(env.value("pi"), Ok(f64::consts::PI));
	assert_eq!(env.value("unknown"), Err(ErrorKind::NameNotFound));
	assert_eq!(env.value("mean"), Err(ErrorKind::NameNotFound));
}
