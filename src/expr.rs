use super::*;

// Consider this a finite state automaton of some kind.
// At any point while parsing an expression, it is either expecting a value or operator-like thing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
	Value,
	Operator,
}

struct FnVal {
	pfn: Function,
	pre: Order,
	nargs: u8,
}

/// Expression context.
pub struct Expr<'a> {
	env: &'a dyn Env,
	fns: Vec<FnVal>,
	vals: Vec<Value>,
	next: State,
	position: usize,
}

impl<'a> Expr<'a> {
	/// Creates a new expression and binds it to the environment.
	#[inline]
	pub const fn new(env: &'a dyn Env) -> Expr<'a> {
		Expr {
			env,
			fns: Vec::new(),
			vals: Vec::new(),
			next: State::Value,
			position: 0,
		}
	}
	#[inline]
	fn with_capacity(env: &'a dyn Env, capacity: usize) -> Expr<'a> {
		Expr {
			env,
			fns: Vec::with_capacity(capacity),
			vals: Vec::with_capacity(capacity),
			next: State::Value,
			position: 0,
		}
	}
	/// Parses a token.
	pub fn parse(&mut self, tok: &Token) -> Result<(), Error> {
		self.position = tok.position;
		let result = match self.next {
			State::Operator => self.parse_op(&tok.kind),
			State::Value => self.parse_val(&tok.kind),
		};
		result.map_err(|kind| Error { kind, position: tok.position })
	}
	/// Feeds new input to be parsed and evaluated.
	pub fn feed(&mut self, input: &str) -> Result<(), Error> {
		// Tokenize and parse the input
		for tok in tokenize(input) {
			// Dispatch based on a simple state machine:
			//  expect either an operator or value like token.
			self.parse(&tok)?;
		}
		Ok(())
	}
	/// Finalizes the expression and calculates the final result.
	pub fn result(mut self) -> Result<Value, Error> {
		let position = self.position;
		let wrap_err = |kind| Error { kind, position };
		// Must end at a value like token
		if self.next == State::Value {
			return Err(wrap_err(ErrorKind::UnfinishedExpression));
		}
		// Evaluate all pending operators
		self.eval_gt(Order::FnBarrier).map_err(wrap_err)?;
		// Expect exactly one result
		if self.vals.len() != 1 || self.fns.len() != 0 {
			return Err(wrap_err(ErrorKind::UnbalancedParens));
		}
		// Return the result
		Ok(self.vals[0])
	}
}

//----------------------------------------------------------------

// Implementation details go here.
impl<'a> Expr<'a> {
	fn parse_val(&mut self, tok: &TokenKind) -> Result<(), ErrorKind> {
		match tok {
			TokenKind::Unk(_) => {
				Err(ErrorKind::InvalidToken)
			},
			TokenKind::Lit(val) => {
				// Push on the value stack
				self.vals.push(*val);
				// Followed by an operator
				self.next = State::Operator;
				Ok(())
			},
			TokenKind::Op(op) => {
				// Unary operators have high precedence
				let desc = op.desc();
				if desc.unary {
					self.fns.push(FnVal {
						pfn: desc.pfn,
						pre: Order::Unary,
						nargs: 1,
					});
					// Followed by a value
					self.next = State::Value;
					Ok(())
				}
				else {
					Err(ErrorKind::DisallowedUnary)
				}
			},
			TokenKind::Var(name) => {
				// Lookup the symbol variable
				let result = self.env.value(name)?;
				// And push the resulting value
				self.vals.push(result);
				// Followed by an operator
				self.next = State::Operator;
				Ok(())
			},
			TokenKind::Open(name) => {
				let pfn = self.env.function(name)?;
				let pre = Order::FnBarrier; // Very low precedence acts as a barrier
				let nargs = 1;
				self.fns.push(FnVal { pfn, pre, nargs });
				// Followed by its arguments
				self.next = State::Value;
				Ok(())
			},
			TokenKind::Comma => {
				Err(ErrorKind::NaExpression)
			},
			TokenKind::Close => {
				// This should catch function calls with empty argument list...
				// Eg. `add()` or `pi()`. For constants just leave the parens out.
				if self.fns.last().map(|f| f.pre == Order::FnBarrier && f.nargs == 1).unwrap_or(false) {
					Err(ErrorKind::BadArgument)
				}
				else {
					Err(ErrorKind::NaExpression)
				}
			},
		}
	}
	fn parse_op(&mut self, tok: &TokenKind) -> Result<(), ErrorKind> {
		match tok {
			TokenKind::Unk(_) => {
				Err(ErrorKind::InvalidToken)
			},
			TokenKind::Lit(_) => {
				Err(ErrorKind::ExpectOperator)
			},
			TokenKind::Op(op) => {
				// Get relevant operator descriptor
				let desc = op.desc();
				// Evaluate all lower precedence fns
				match desc.assoc {
					Assoc::Left => self.eval_ge(desc.pre)?,
					Assoc::Right => self.eval_gt(desc.pre)?,
					// Assoc::None => return Err(Error::InternalError),
				};
				// Push operator as fn, always takes two arguments
				self.fns.push(FnVal {
					pfn: desc.pfn,
					pre: desc.pre,
					nargs: 2,
				});
				// Followed by a value
				self.next = State::Value;
				Ok(())
			},
			TokenKind::Var(_) => {
				// Insert implicit multiplication token
				self.parse_op(&TokenKind::Op(Operator::IMul))?;
				// Retry inserting this token
				self.parse_val(tok)
			},
			TokenKind::Open(_) => {
				// Insert implicit multiplication token
				self.parse_op(&TokenKind::Op(Operator::IMul))?;
				// Retry inserting this token
				self.parse_val(tok)
			},
			TokenKind::Comma => {
				// Eval until an fn barier
				self.eval_gt(Order::FnBarrier)?;
				// Increment nargs for that fn
				self.fns.last_mut().ok_or(ErrorKind::MisplacedComma)?.nargs += 1;
				// Followed by a value
				self.next = State::Value;
				Ok(())
			},
			TokenKind::Close => {
				// Eval everything until the fn barrier and push past it
				self.eval_gt(Order::FnBarrier)?;
				self.eval_apply()?;
				// Followed by an operator
				self.next = State::Operator;
				Ok(())
			},
		}
	}
	// Eval all fns with higher or equal precedence.
	fn eval_ge(&mut self, pre: Order) -> Result<(), ErrorKind> {
		while self.fns.last().map(|f| f.pre >= pre).unwrap_or(false) {
			self.eval_apply()?;
		}
		Ok(())
	}
	// Eval all fns with strictly higher precedence.
	fn eval_gt(&mut self, pre: Order) -> Result<(), ErrorKind> {
		while self.fns.last().map(|f| f.pre > pre).unwrap_or(false) {
			self.eval_apply()?;
		}
		Ok(())
	}
	// Pop and eval a single fn.
	fn eval_apply(&mut self) -> Result<(), ErrorKind> {
		if let Some(f) = self.fns.pop() {
			// Find its arguments
			if f.nargs as usize > self.vals.len() {
				// This should never happen... Panic instead?
				// Indicates a logic error when manipulating the nargs.
				return Err(ErrorKind::InternalError);
			}
			let args = self.vals.len() - f.nargs as usize..;
			// Apply the fn
			let result = {
				let vals = &mut self.vals[args.clone()];
				(f.pfn)(self.env, vals)?
			};
			// Pop vals and push result
			let _ = self.vals.drain(args.clone());
			self.vals.push(result);
			Ok(())
		}
		else {
			// You tried to apply an fn when there are no more fns to apply
			Err(ErrorKind::UnbalancedParens)
		}
	}
}

/// Evaluates and calculates the result in one step.
///
/// ```
/// let env = pupil::BasicEnv::default();
/// let result = pupil::eval(&env, "2 + 3");
/// assert_eq!(result, Ok(5.0));
/// ```
pub fn eval(env: &dyn Env, input: &str) -> Result<Value, Error> {
	let mut expr = Expr::new(env);
	expr.feed(input)?;
	expr.result()
}

/// Evaluates a list of tokens and calculates the result.
///
/// This is useful if you want to tokenize separately first.
///
/// ```
/// let env = pupil::BasicEnv::default();
/// let tokens: Vec<pupil::Token> = pupil::tokenize("2 + 3").collect();
/// let result = pupil::eval_tokens(&env, &tokens);
/// assert_eq!(result, Ok(5.0));
/// ```
pub fn eval_tokens(env: &dyn Env, tokens: &[Token]) -> Result<Value, Error> {
	let mut expr = Expr::with_capacity(env, tokens.len() / 2 + 1);
	for tok in tokens {
		expr.parse(tok)?;
	}
	expr.result()
}

#[test]
fn basics() {
	let env = BasicEnv::default();
	assert_eq!(eval(&env, "2 + 3"), Ok(5.0));
	assert_eq!(eval(&env, "2-3*4"), Ok(-10.0));
	assert_eq!(eval(&env, "2*3+4"), Ok(10.0));
	assert_eq!(eval(&env, "3^2-2"), Ok(7.0));
	assert_eq!(eval(&env, "2+---2"), Ok(0.0));
	assert_eq!(eval(&env, "-1"), Ok(-1.0));
	assert_eq!(eval(&env, "-2^2 + 3*4 + sin(pi / 2)"), Ok(9.0));
}
#[test]
fn funcs() {
	let env = BasicEnv::default();
	assert_eq!(eval(&env, "2*(3+4)"), Ok(14.0));
	assert_eq!(eval(&env, "mul(2,add(3,4))"), Ok(14.0));
}
#[test]
fn errors() {
	let env = BasicEnv::default();
	let err_kind = |input: &str| eval(&env, input).map_err(|e| e.kind);
	assert_eq!(err_kind(""), Err(ErrorKind::UnfinishedExpression));
	assert_eq!(err_kind("12 5"), Err(ErrorKind::ExpectOperator));
	assert_eq!(err_kind(","), Err(ErrorKind::NaExpression));
	assert_eq!(err_kind(")"), Err(ErrorKind::NaExpression));
	assert_eq!(err_kind("*2"), Err(ErrorKind::DisallowedUnary));
	assert_eq!(err_kind("2 +"), Err(ErrorKind::UnfinishedExpression));
	assert_eq!(err_kind("(2"), Err(ErrorKind::UnbalancedParens));
	assert_eq!(err_kind("(3))"), Err(ErrorKind::UnbalancedParens));
	assert_eq!(err_kind("2,"), Err(ErrorKind::MisplacedComma));
	assert_eq!(err_kind("pi()"), Err(ErrorKind::NameNotFound));
	assert_eq!(err_kind("mean"), Err(ErrorKind::NameNotFound));
	assert_eq!(err_kind("hello(5)"), Err(ErrorKind::NameNotFound));
	assert_eq!(err_kind("hi"), Err(ErrorKind::NameNotFound));
}
