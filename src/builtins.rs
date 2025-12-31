//! Builtins.
//!
//! Builtin functions are programmed in Rust and can be made accessible while evaluating arithmetic expressions.

use std::f64::consts;
use crate::*;

pub fn builtin_id(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value)
}
pub fn builtin_signum(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.signum())
}
pub fn builtin_add(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(vals.iter().fold(0f64, |acc, x| acc + x))
}
pub fn builtin_sub(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	match vals {
		&mut [value] => Ok(-value),
		&mut [lhs, rhs] => Ok(lhs - rhs),
		_ => Err(Error::BadArgument),
	}
}
pub fn builtin_mul(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(vals.iter().fold(1f64, |acc, x| acc * x))
}
pub fn builtin_div(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(lhs / rhs)
}
pub fn builtin_rem(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(lhs % rhs)
}
pub fn builtin_pow(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [base, exp] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(base.powf(exp))
}
pub fn builtin_floor(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.floor())
}
pub fn builtin_ceil(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.ceil())
}
pub fn builtin_round(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.round())
}
pub fn builtin_abs(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.abs())
}
pub fn builtin_sqr(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value * value)
}
pub fn builtin_cube(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value * value * value)
}
pub fn builtin_sqrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.sqrt())
}
pub fn builtin_cbrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.cbrt())
}
pub fn builtin_min(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(vals.iter().fold(Value::INFINITY, |acc, &x| acc.min(x)))
}
pub fn builtin_max(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(vals.iter().fold(Value::NEG_INFINITY, |acc, &x| acc.max(x)))
}
pub fn builtin_clamp(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value, min, max] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.max(min).min(max))
}
pub fn builtin_eq(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	match vals {
		&mut [lhs, rhs] => if lhs == rhs { Ok(1f64) } else { Ok(0f64) },
		&mut [lhs, rhs, tolerance] => if (lhs - rhs).abs() <= tolerance.abs() { Ok(1f64) } else { Ok(0f64) },
		_ => Err(Error::BadArgument),
	}
}
pub fn builtin_ne(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	match vals {
		&mut [lhs, rhs] => if lhs != rhs { Ok(1f64) } else { Ok(0f64) },
		&mut [lhs, rhs, tolerance] => if (lhs - rhs).abs() > tolerance.abs() { Ok(1f64) } else { Ok(0f64) },
		_ => Err(Error::BadArgument),
	}
}
pub fn builtin_lt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	if lhs < rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn builtin_le(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	if lhs <= rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn builtin_gt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	if lhs > rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn builtin_ge(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [lhs, rhs] = vals else {
		return Err(Error::BadArgument);
	};
	if lhs >= rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn builtin_all(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	for &val in vals.iter() {
		if val == 0f64 {
			return Ok(0f64);
		}
	}
	Ok(1f64)
}
pub fn builtin_any(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	for &val in vals.iter() {
		if val != 0f64 {
			return Ok(1f64);
		}
	}
	Ok(0f64)
}
pub fn builtin_not(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(1.0 - value.signum().abs())
}
pub fn builtin_select(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() < 2 {
		return Err(Error::BadArgument);
	}
	let index = vals[0].floor() as i32 as usize;
	let choices = &vals[1..];
	choices.get(index).cloned().ok_or(Error::BadArgument)
}
pub fn builtin_step(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	if value > 0f64 { Ok(1f64) } else { Ok(0f64) }
}
pub fn builtin_smoothstep(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [x] = vals else {
		return Err(Error::BadArgument);
	};
	if x <= 0f64 {
		Ok(0f64)
	}
	else if x >= 1f64 {
		Ok(1f64)
	}
	else {
		Ok(x * x * (3f64 - 2f64 * x))
	}
}
pub fn builtin_smootherstep(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [x] = vals else {
		return Err(Error::BadArgument);
	};
	if x <= 0f64 {
		Ok(0f64)
	}
	else if x >= 1f64 {
		Ok(1f64)
	}
	else {
		Ok(x * x * x * (x * (x * 6f64 - 15f64) + 10f64))
	}
}
pub fn builtin_exp(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.exp())
}
pub fn builtin_exp2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.exp2())
}
pub fn builtin_expm1(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.exp_m1())
}
pub fn builtin_ln(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.ln())
}
pub fn builtin_log(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value, base] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.log(base))
}
pub fn builtin_log2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.log2())
}
pub fn builtin_log10(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.log10())
}
pub fn builtin_ln1p(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.ln_1p())
}
pub fn builtin_e(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if !vals.is_empty() {
		return Err(Error::BadArgument);
	}
	Ok(consts::E)
}
pub fn builtin_mean(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.is_empty() {
		return Err(Error::BadArgument);
	}
	Ok(builtin_add(env, vals)? / vals.len() as Value)
}
pub fn builtin_median(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.is_empty() {
		return Err(Error::BadArgument);
	}
	vals.sort_unstable_by(f64::total_cmp);
	// Pick the median value
	let len = vals.len();
	if len & 1 == 0 {
		Ok((vals[(len >> 1) - 1] + vals[len >> 1]) * 0.5)
	}
	else {
		Ok(vals[len >> 1])
	}
}
pub fn builtin_range(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let (mut min, mut max) = (Value::INFINITY, Value::NEG_INFINITY);
	for &val in vals.iter() {
		if !(val >= min) {
			min = val;
		}
		else if !(val <= max) {
			max = val;
		}
	}
	Ok(max - min)
}
pub fn builtin_var(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let mean = builtin_mean(env, vals)?;
	Ok(vals.iter().fold(0f64, |acc, &x| acc + (x - mean) * (x - mean)) / vals.len() as Value)
}
pub fn builtin_stdev(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(builtin_var(env, vals)?.sqrt())
}
pub fn builtin_deg(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [radians] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(radians.to_degrees())
}
pub fn builtin_rad(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [degrees] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(degrees.to_radians())
}
pub fn builtin_pi(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if !vals.is_empty() {
		return Err(Error::BadArgument);
	}
	Ok(consts::PI)
}
pub fn builtin_tau(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if !vals.is_empty() {
		return Err(Error::BadArgument);
	}
	Ok(consts::TAU)
}
pub fn builtin_sin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [radians] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(radians.sin())
}
pub fn builtin_cos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [radians] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(radians.cos())
}
pub fn builtin_tan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [radians] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(radians.tan())
}
pub fn builtin_asin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.asin())
}
pub fn builtin_acos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.acos())
}
pub fn builtin_atan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.atan())
}
pub fn builtin_atan2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [this, other] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(this.atan2(other))
}
pub fn builtin_sinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.sinh())
}
pub fn builtin_cosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.cosh())
}
pub fn builtin_tanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.tanh())
}
pub fn builtin_asinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.asinh())
}
pub fn builtin_acosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.acosh())
}
pub fn builtin_atanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let &mut [value] = vals else {
		return Err(Error::BadArgument);
	};
	Ok(value.atanh())
}

#[test]
fn stats() {
	let env = crate::BasicEnv::default();
	assert_eq!(builtin_mean(&env, &mut [1.0, 2.0, 4.0, -1.0]), Ok(1.5));
	assert_eq!(builtin_median(&env, &mut [2.0, 1.0, 4.0]), Ok(2.0));
	assert_eq!(builtin_median(&env, &mut [8.0, 4.0]), Ok(6.0));
	assert_eq!(builtin_range(&env, &mut [1.0, 7.0, 4.5]), Ok(6.0));
	assert_eq!(builtin_var(&env, &mut [3.0, 4.0, 7.0, 10.0]), Ok(7.5));
	assert_eq!(builtin_stdev(&env, &mut [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]), Ok(2.0));
}
