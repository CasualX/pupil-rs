use crate::*;

// Consider this a finite state automaton of some kind.
// At any point while parsing an expression, it is either expecting a value or operator-like thing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State { Val, Op }

struct FnVal {
	pfn: BuiltinFn,
	pre: Order,
	nargs: u8,
}

/// The expression context.
pub struct Expr<'a> {
	env: &'a dyn Env,
	fns: Vec<FnVal>,
	vals: Vec<Value>,
	next: State,
}

impl<'a> Expr<'a> {
	/// Creates a new expression and binds it to the environment.
	pub fn new(env: &'a dyn Env) -> Expr<'a> {
		Expr {
			env,
			fns: Vec::new(),
			vals: Vec::new(),
			next: State::Val,
		}
	}
	/// Parses a token.
	pub fn parse(&mut self, tok: Token) -> Result<(), Error> {
		match self.next {
			State::Op => self.parse_op(tok),
			State::Val => self.parse_val(tok),
		}
	}
	/// Feeds new input to be parsed and evaluated.
	pub fn feed(&mut self, input: &str) -> Result<(), Error> {
		// Tokenize and parse the input
		for tok in tokenize(input) {
			// Dispatch based on a simple state machine:
			//  expect either an operator or value like token.
			self.parse(tok)?;
		}
		Ok(())
	}
	/// Finalizes the expression and calculates the final result.
	pub fn result(mut self) -> Result<Value, Error> {
		// Must end at a value like token
		if self.next == State::Val {
			return Err(Error::UnfinishedExpression);
		}
		// Evaluate all pending operators
		self.eval_gt(Order::FnBarrier)?;
		// Expect exactly one result
		if self.vals.len() != 1 || self.fns.len() != 0 {
			return Err(Error::UnbalancedParens);
		}
		// Return the result
		Ok(self.vals[0])
	}
}

//----------------------------------------------------------------

// Implementation details go here.
impl<'a> Expr<'a> {
	fn parse_val(&mut self, tok: Token) -> Result<(), Error> {
		match tok {
			Token::Unk(_) => {
				Err(Error::InvalidToken)
			},
			Token::Lit(val) => {
				// Push on the value stack
				self.vals.push(val);
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
			Token::Op(op) => {
				// Unary operators have high precedence
				let desc = op.desc();
				if desc.unary {
					self.fns.push(FnVal {
						pfn: desc.pfn,
						pre: Order::Unary,
						nargs: 1,
					});
					// Followed by a value
					self.next = State::Val;
					Ok(())
				}
				else {
					Err(Error::DisallowedUnary)
				}
			},
			Token::Var(name) => {
				// Lookup the symbol variable
				let result = self.env.get_value(name)?;
				// And push the resulting value
				self.vals.push(result);
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
			Token::Open(name) => {
				let pfn = self.env.builtin(name)?;
				let pre = Order::FnBarrier; // Very low precedence acts as a barrier
				let nargs = 1;
				self.fns.push(FnVal { pfn, pre, nargs });
				// Followed by its arguments
				self.next = State::Val;
				Ok(())
			},
			Token::Comma => {
				Err(Error::NaExpression)
			},
			Token::Close => {
				// This should catch function calls with empty argument list...
				// Eg. `add()` or `pi()`. For constants just leave the parens out.
				if self.fns.last().map(|f| f.pre == Order::FnBarrier && f.nargs == 1).unwrap_or(false) {
					Err(Error::BadArgument)
				}
				else {
					Err(Error::NaExpression)
				}
			},
		}
	}
	fn parse_op(&mut self, tok: Token) -> Result<(), Error> {
		match tok {
			Token::Unk(_) => {
				Err(Error::InvalidToken)
			},
			Token::Lit(_) => {
				Err(Error::ExpectOperator)
			},
			Token::Op(op) => {
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
				self.next = State::Val;
				Ok(())
			},
			Token::Var(_) => {
				// Insert implicit multiplication token
				self.parse_op(Token::Op(Operator::IMul))?;
				// Retry inserting this token
				self.parse_val(tok)
			},
			Token::Open(_) => {
				// Insert implicit multiplication token
				self.parse_op(Token::Op(Operator::IMul))?;
				// Retry inserting this token
				self.parse_val(tok)
			},
			Token::Comma => {
				// Eval until an fn barier
				self.eval_gt(Order::FnBarrier)?;
				// Increment nargs for that fn
				self.fns.last_mut().ok_or(Error::MisplacedComma)?.nargs += 1;
				// Followed by a value
				self.next = State::Val;
				Ok(())
			},
			Token::Close => {
				// Eval everything until the fn barrier and push past it
				self.eval_gt(Order::FnBarrier)?;
				self.eval_apply()?;
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
		}
	}
	// Eval all fns with higher or equal precedence.
	fn eval_ge(&mut self, pre: Order) -> Result<(), Error> {
		while self.fns.last().map(|f| f.pre >= pre).unwrap_or(false) {
			self.eval_apply()?;
		}
		Ok(())
	}
	// Eval all fns with strictly higher precedence.
	fn eval_gt(&mut self, pre: Order) -> Result<(), Error> {
		while self.fns.last().map(|f| f.pre > pre).unwrap_or(false) {
			self.eval_apply()?;
		}
		Ok(())
	}
	// Pop and eval a single fn.
	fn eval_apply(&mut self) -> Result<(), Error> {
		if let Some(f) = self.fns.pop() {
			// Find its arguments
			if f.nargs as usize > self.vals.len() {
				// This should never happen... Panic instead?
				// Indicates a logic error when manipulating the nargs.
				return Err(Error::InternalError);
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
			Err(Error::UnbalancedParens)
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

#[test]
fn basics() {
	let env = crate::BasicEnv::default();
	assert_eq!(eval(&env, "2 + 3"), Ok(5.0));
	assert_eq!(eval(&env, "2-3*4"), Ok(-10.0));
	assert_eq!(eval(&env, "2*3+4"), Ok(10.0));
	assert_eq!(eval(&env, "3^2-2"), Ok(7.0));
	assert_eq!(eval(&env, "2+---2"), Ok(0.0));
	assert_eq!(eval(&env, "-1"), Ok(-1.0));
}
#[test]
fn funcs() {
	let env = crate::BasicEnv::default();
	assert_eq!(eval(&env, "2*(3+4)"), Ok(14.0));
	assert_eq!(eval(&env, "mul(2,add(3,4))"), Ok(14.0));
}
#[test]
fn errors() {
	let env = crate::BasicEnv::default();
	assert_eq!(eval(&env, ""), Err(Error::UnfinishedExpression));
	assert_eq!(eval(&env, "12 5"), Err(Error::ExpectOperator));
	assert_eq!(eval(&env, ","), Err(Error::NaExpression));
	assert_eq!(eval(&env, ")"), Err(Error::NaExpression));
	assert_eq!(eval(&env, "*2"), Err(Error::DisallowedUnary));
	assert_eq!(eval(&env, "2 +"), Err(Error::UnfinishedExpression));
	assert_eq!(eval(&env, "!&"), Err(Error::InvalidToken));
	assert_eq!(eval(&env, "(2"), Err(Error::UnbalancedParens));
	assert_eq!(eval(&env, "(3))"), Err(Error::UnbalancedParens));
	assert_eq!(eval(&env, "2,"), Err(Error::MisplacedComma));
	assert_eq!(eval(&env, "pi()"), Err(Error::BadArgument));
	assert_eq!(eval(&env, "mean"), Err(Error::EnvErrorBuiltinFn));
	assert_eq!(eval(&env, "hello(5)"), Err(Error::EnvErrorNotFound));
	assert_eq!(eval(&env, "hi"), Err(Error::EnvErrorNotFound));
}
