//! Builtins.
//!
//! Builtin functions are programmed in Rust and can be made accessible while evaluating arithmetic expressions.

use std::f64::consts;
use crate::*;

pub fn builtin_id(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_add(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(0f64, |acc, x| acc + x)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sub(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	match vals.len() {
		1 => Ok(-vals[0]),
		2 => Ok(vals[0] - vals[1]),
		_ => Err(Error::BadArgument),
	}
}
pub fn builtin_mul(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 1 { Ok(vals.iter().fold(1f64, |acc, x| acc * x)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_div(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0] / vals[1]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_rem(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0] % vals[1]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_pow(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].powf(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_floor(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].floor()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ceil(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ceil()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_round(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].round()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_abs(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].abs()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sqr(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cube(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * vals[0] * vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sqrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sqrt()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cbrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cbrt()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_min(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(vals[0], |acc, &x| acc.min(x))) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_max(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(vals[0], |acc, &x| acc.max(x))) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_exp(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_exp2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp2()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_expm1(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp_m1()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ln(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ln()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].log(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].log2()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log10(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].log10()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ln1p(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ln_1p()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_e(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::E) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_mean(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(builtin_add(env, vals)? / vals.len() as Value)
}
pub fn builtin_median(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 {
		vals.sort_by(f64::total_cmp);
		// Pick the median value
		let len = vals.len();
		if len & 1 == 0 {
			Ok((vals[(len >> 1) - 1] + vals[len >> 1]) * 0.5)
		}
		else {
			Ok(vals[len >> 1])
		}
	}
	else {
		Err(Error::BadArgument)
	}
}
pub fn builtin_range(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 {
		let (mut min, mut max) = (vals[0], vals[0]);
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
	else {
		Err(Error::BadArgument)
	}
}
pub fn builtin_var(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	let mean = builtin_mean(env, vals)?;
	Ok(vals.iter().fold(0f64, |acc, &x| acc + (x - mean) * (x - mean)) / vals.len() as Value)
}
pub fn builtin_stdev(env: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(builtin_var(env, vals)?.sqrt())
}
pub fn builtin_deg(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * (180f64 / consts::PI)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_rad(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * (consts::PI / 180f64)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_pi(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::PI) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tau(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::PI + consts::PI) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sin()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cos()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].tan()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_asin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].asin()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_acos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].acos()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].atan()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atan2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].atan2(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sinh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cosh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].tanh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_asinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].asinh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_acosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].acosh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].atanh()) }
	else { Err(Error::BadArgument) }
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
