//! Builtins.
//!
//! Builtin functions are programmed in Rust and can be made accessible while evaluating arithmetic expressions.

use ::std::f64::consts;
use super::env::{Value, Error, Env};

// How do I use macros to tidy this up?
// It’s pretty much guaranteed I messed up somewhere...

pub fn builtin_id(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_add(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(0f64, |acc, x| acc + x)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sub(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	match vals.len() {
		1 => Ok(-vals[0]),
		2 => Ok(vals[0] - vals[1]),
		_ => Err(Error::BadArgument),
	}
}
pub fn builtin_mul(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 1 { Ok(vals.iter().fold(1f64, |acc, x| acc * x)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_div(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0] / vals[1]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_rem(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0] % vals[1]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_pow(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].powf(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_floor(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].floor()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ceil(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ceil()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_round(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].round()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_abs(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].abs()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sqr(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cube(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * vals[0] * vals[0]) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sqrt(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sqrt()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cbrt(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cbrt()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_min(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(vals[0], |acc, &x| acc.min(x))) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_max(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 { Ok(vals.iter().fold(vals[0], |acc, &x| acc.max(x))) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_exp(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_exp2(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp2()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_expm1(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].exp_m1()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ln(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ln()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].log(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log2(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].log2()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_log10(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].log10()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_ln1p(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].ln_1p()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_e(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::E) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_mean(env: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(try!(builtin_add(env, vals)) / vals.len() as Value)
}
pub fn builtin_median(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() > 0 {
		// Sorting floats, my favourite pasttime
		vals.sort_by(|&a, &b| {
			if a < b { ::std::cmp::Ordering::Less }
			else if a > b { ::std::cmp::Ordering::Greater }
			else { ::std::cmp::Ordering::Equal }
		});
		// Pick the median value
		let len = vals.len();
		if (len & 1) == 0 {
			Ok((vals[(len >> 1) - 1] + vals[len >> 1]) / 2.0)
		}
		else {
			Ok(vals[len >> 1])
		}
	}
	else {
		Err(Error::BadArgument)
	}
}
pub fn builtin_range(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
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
pub fn builtin_var(env: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	let mean = try!(builtin_mean(env, vals));
	Ok(vals.iter().fold(0f64, |acc, &x| acc + (x - mean) * (x - mean)) / vals.len() as Value)
}
pub fn builtin_stdev(env: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	Ok(try!(builtin_var(env, vals)).sqrt())
}
pub fn builtin_deg(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * (180f64 / consts::PI)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_rad(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0] * (consts::PI / 180f64)) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_pi(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::PI) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tau(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 0 { Ok(consts::PI + consts::PI) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sin(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sin()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cos(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cos()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tan(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].tan()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_asin(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].asin()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_acos(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].acos()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atan(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].atan()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atan2(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 2 { Ok(vals[0].atan2(vals[1])) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_sinh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].sinh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_cosh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].cosh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_tanh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].tanh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_asinh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].asinh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_acosh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].acosh()) }
	else { Err(Error::BadArgument) }
}
pub fn builtin_atanh(_: &Env, vals: &mut [Value]) -> Result<Value, Error> {
	if vals.len() == 1 { Ok(vals[0].atanh()) }
	else { Err(Error::BadArgument) }
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::env::Env;

	#[test]
	fn stats() {
		let env = Env::new();
		assert_eq!(builtin_mean(&env, &mut [1.0, 2.0, 4.0, -1.0]), Ok(1.5));
		assert_eq!(builtin_median(&env, &mut [2.0, 1.0, 4.0]), Ok(2.0));
		assert_eq!(builtin_median(&env, &mut [8.0, 4.0]), Ok(6.0));
		assert_eq!(builtin_range(&env, &mut [1.0, 7.0, 4.5]), Ok(6.0));
		assert_eq!(builtin_var(&env, &mut [3.0, 4.0, 7.0, 10.0]), Ok(7.5));
		assert_eq!(builtin_stdev(&env, &mut [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]), Ok(2.0));
	}
}
