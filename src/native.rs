use super::*;

pub fn id(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value)
}
pub fn sign(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.signum())
}
pub fn add(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	Ok(vals.iter().fold(0f64, |acc, x| acc + x))
}
pub fn sub(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	match vals {
		&mut [value] => Ok(-value),
		&mut [lhs, rhs] => Ok(lhs - rhs),
		_ => Err(ErrorKind::BadArgument),
	}
}
pub fn mul(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	Ok(vals.iter().fold(1f64, |acc, x| acc * x))
}
pub fn div(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(lhs / rhs)
}
pub fn rem(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(lhs % rhs)
}
pub fn pow(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [base, exp] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(base.powf(exp))
}
pub fn fract(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.fract())
}
pub fn floor(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.floor())
}
pub fn ceil(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.ceil())
}
pub fn trunc(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.trunc())
}
pub fn round(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.round())
}
pub fn abs(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.abs())
}
pub fn sqr(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value * value)
}
pub fn cube(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value * value * value)
}
pub fn sqrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.sqrt())
}
pub fn cbrt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.cbrt())
}
pub fn isinf(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if value.is_infinite() { Ok(1f64) } else { Ok(0f64) }
}
pub fn isnan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if value.is_nan() { Ok(1f64) } else { Ok(0f64) }
}
pub fn min(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	Ok(vals.iter().fold(Value::INFINITY, |acc, &x| acc.min(x)))
}
pub fn max(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	Ok(vals.iter().fold(Value::NEG_INFINITY, |acc, &x| acc.max(x)))
}
pub fn clamp(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value, min, max] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.max(min).min(max))
}
pub fn eq(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	match vals {
		&mut [lhs, rhs] => if lhs == rhs { Ok(1f64) } else { Ok(0f64) },
		&mut [lhs, rhs, tolerance] => if (lhs - rhs).abs() <= tolerance.abs() { Ok(1f64) } else { Ok(0f64) },
		_ => Err(ErrorKind::BadArgument),
	}
}
pub fn ne(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	match vals {
		&mut [lhs, rhs] => if lhs != rhs { Ok(1f64) } else { Ok(0f64) },
		&mut [lhs, rhs, tolerance] => if (lhs - rhs).abs() > tolerance.abs() { Ok(1f64) } else { Ok(0f64) },
		_ => Err(ErrorKind::BadArgument),
	}
}
pub fn lt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if lhs < rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn le(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if lhs <= rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn gt(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if lhs > rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn ge(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [lhs, rhs] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if lhs >= rhs { Ok(1f64) } else { Ok(0f64) }
}
pub fn all(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	for &val in vals.iter() {
		if val == 0f64 {
			return Ok(0f64);
		}
	}
	Ok(1f64)
}
pub fn any(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	for &val in vals.iter() {
		if val != 0f64 {
			return Ok(1f64);
		}
	}
	Ok(0f64)
}
pub fn not(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(1.0 - value.signum().abs())
}
pub fn select(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	if vals.len() < 2 {
		return Err(ErrorKind::BadArgument);
	}
	let index = vals[0].floor() as i32 as usize;
	let choices = &vals[1..];
	choices.get(index).cloned().ok_or(ErrorKind::BadArgument)
}
pub fn step(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [edge, value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	if value < edge { Ok(0f64) } else { Ok(1f64) }
}
pub fn smoothstep(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [x] = vals else {
		return Err(ErrorKind::BadArgument);
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
pub fn smootherstep(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [x] = vals else {
		return Err(ErrorKind::BadArgument);
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
pub fn exp(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.exp())
}
pub fn exp2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.exp2())
}
pub fn expm1(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.exp_m1())
}
pub fn ln(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.ln())
}
pub fn log(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value, base] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.log(base))
}
pub fn log2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.log2())
}
pub fn log10(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.log10())
}
pub fn ln1p(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.ln_1p())
}
pub fn mean(env: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	if vals.is_empty() {
		return Err(ErrorKind::BadArgument);
	}
	Ok(add(env, vals)? / vals.len() as Value)
}
pub fn median(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	if vals.is_empty() {
		return Err(ErrorKind::BadArgument);
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
pub fn range(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
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
pub fn var(env: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let mean = mean(env, vals)?;
	Ok(vals.iter().fold(0f64, |acc, &x| acc + (x - mean) * (x - mean)) / vals.len() as Value)
}
pub fn stdev(env: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	Ok(var(env, vals)?.sqrt())
}
pub fn deg(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [radians] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(radians.to_degrees())
}
pub fn rad(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [degrees] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(degrees.to_radians())
}
pub fn sin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [radians] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(radians.sin())
}
pub fn cos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [radians] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(radians.cos())
}
pub fn tan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [radians] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(radians.tan())
}
pub fn asin(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.asin())
}
pub fn acos(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.acos())
}
pub fn atan(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.atan())
}
pub fn atan2(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [this, other] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(this.atan2(other))
}
pub fn sinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.sinh())
}
pub fn cosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.cosh())
}
pub fn tanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.tanh())
}
pub fn asinh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.asinh())
}
pub fn acosh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.acosh())
}
pub fn atanh(_: &dyn Env, vals: &mut [Value]) -> Result<Value, ErrorKind> {
	let &mut [value] = vals else {
		return Err(ErrorKind::BadArgument);
	};
	Ok(value.atanh())
}

#[test]
fn stats() {
	let env = BasicEnv::default();
	assert_eq!(mean(&env, &mut [1.0, 2.0, 4.0, -1.0]), Ok(1.5));
	assert_eq!(median(&env, &mut [2.0, 1.0, 4.0]), Ok(2.0));
	assert_eq!(median(&env, &mut [8.0, 4.0]), Ok(6.0));
	assert_eq!(range(&env, &mut [1.0, 7.0, 4.5]), Ok(6.0));
	assert_eq!(var(&env, &mut [3.0, 4.0, 7.0, 10.0]), Ok(7.5));
	assert_eq!(stdev(&env, &mut [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]), Ok(2.0));
}
