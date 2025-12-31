use std::io::{self, Write};
use std::env;

fn main() {
	let con = unsafe { libc::isatty(0) != 0 };
	let args = env::args();

	if con {
		println!("Welcome to pupil, the arithmetic expression evaluator.");
		if args.len() <= 1 {
			print!("
Enter an expression, eg. 2 + 3, and press enter.
Press ctrl-C to exit.

Built-in functions:
  +-*/^   : Operators with correct precedence.
  (expr)  : Group expression with parentheses.
  ans     : Use answer from previous expression.
  pi, tau : Trigonometric constants.
  e       : Euler’s number.
  add, sub, mul, div, rem, pow,
  floor, ceil, trunc, round, fract,
  abs, sign, sqr, cube, sqrt, cbrt, isinf, isnan,
  min, max, clamp, step, smoothstep, smootherstep,
  eq, ne, lt, le, gt, ge, all, any, not, select,
  exp, expm1, ln, log, log2, log10, ln1p,
  mean, median, range, var, stdev,
  deg, rad, sin, cos, tan, asin, acos, atan, atan2,
  sinh, cosh, tanh, asinh, acosh, atanh
          : Use parens to provide arguments.

");
		}
	}

	// Initialize the environment
	let mut env = pupil::BasicEnv::default();

	// Eval the command line args
	if args.len() > 1 {
		let mut expr = pupil::Expr::new(&env);
		let result = (|| {
			for s in args.skip(1) {
				if let Err(e) = expr.feed(&s) {
					return Err(e);
				}
			}
			expr.result()
		})();
		// Print the result
		match result {
			Ok(val) => {
				println!("Ok: {}", val);
			},
			Err(e) => {
				eprintln!("Err: {}!", e);
			},
		}
	}
	// Eval from stdin
	else {
		loop {
			// If user is at a console, print a nice REPL
			if con {
				print!(">>> ");
				io::stdout().flush().ok();
			}
			// Read input from stdin
			let mut line = String::new();
			if io::stdin().read_line(&mut line).is_err() {
				break;
			}
			// Not sure how to handle ctrl-c events, Rust’s read_line is a bit weird in this regard
			// I basically get an empty string as opposed to a newline when you just press enter.
			if line.len() == 0 {
				break;
			}
			// If you press enter without any input, just retry without evaluating.
			let line = line.trim();
			if line.len() > 0 {
				// Evaluate the expression
				match pupil::eval(&env, &line) {
					Ok(val) => {
						println!("{}", val);
						env.ans = val;
					},
					Err(e) => {
						eprint!("    {}", e.compact_diagnostic(&line));
					},
				}
			}
		}
	}
}
