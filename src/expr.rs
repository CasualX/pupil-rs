//! Expressions.

use super::env::{Value, Error, BuiltinFn, Env};
use super::lexer::{tokenize, Token};
use super::op::{Operator, Order, Assoc};

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
	env: &'a Env,
	fns: Vec<FnVal>,
	vals: Vec<Value>,
	next: State,
}

impl<'a> Expr<'a> {
	/// Creates a new expression and binds it to the environment.
	pub fn new(env: &'a Env) -> Expr<'a> {
		Expr {
			env: env,
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
	/// Evaluates and calculates the result in one step.
	pub fn eval(mut self, input: &str) -> Result<Value, Error> {
		self.feed(input)?;
		self.result()
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
			Token::Var(id) => {
				// Lookup the symbol variable
				let result = self.env.value(id)?;
				// And push the resulting value
				self.vals.push(result);
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
			Token::Open(id) => {
				// Lookup the symbol
				let pfn = self.env.builtin(id)?;
				// Push with very low precedence, acts as a barrier
				self.fns.push(FnVal {
					pfn: pfn,
					pre: Order::FnBarrier,
					nargs: 1,
				});
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
					Assoc::None => return Err(Error::InternalCorruption),
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
				return Err(Error::InternalCorruption);
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

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::env::*;

	#[test]
	fn basics() {
		let env = BasicEnv::default();
		assert_eq!(Expr::new(&env).eval("2 + 3"), Ok(5.0));
		assert_eq!(Expr::new(&env).eval("2-3*4"), Ok(-10.0));
		assert_eq!(Expr::new(&env).eval("2*3+4"), Ok(10.0));
		assert_eq!(Expr::new(&env).eval("3^2-2"), Ok(7.0));
		assert_eq!(Expr::new(&env).eval("2+---2"), Ok(0.0));
		assert_eq!(Expr::new(&env).eval("-1"), Ok(-1.0));
	}
	#[test]
	fn funcs() {
		let env = BasicEnv::default();
		assert_eq!(Expr::new(&env).eval("2*(3+4)"), Ok(14.0));
		assert_eq!(Expr::new(&env).eval("mul(2,add(3,4))"), Ok(14.0));
	}
	#[test]
	fn errors() {
		let env = BasicEnv::default();
		assert_eq!(Expr::new(&env).eval(""), Err(Error::UnfinishedExpression));
		assert_eq!(Expr::new(&env).eval("12 5"), Err(Error::ExpectOperator));
		assert_eq!(Expr::new(&env).eval(","), Err(Error::NaExpression));
		assert_eq!(Expr::new(&env).eval(")"), Err(Error::NaExpression));
		assert_eq!(Expr::new(&env).eval("*2"), Err(Error::DisallowedUnary));
		assert_eq!(Expr::new(&env).eval("2 +"), Err(Error::UnfinishedExpression));
		assert_eq!(Expr::new(&env).eval("!&"), Err(Error::InvalidToken));
		assert_eq!(Expr::new(&env).eval("(2"), Err(Error::UnbalancedParens));
		assert_eq!(Expr::new(&env).eval("(3))"), Err(Error::UnbalancedParens));
		assert_eq!(Expr::new(&env).eval("2,"), Err(Error::MisplacedComma));
		assert_eq!(Expr::new(&env).eval("pi()"), Err(Error::BadArgument));
		assert_eq!(Expr::new(&env).eval("mean"), Err(Error::EnvError(EnvError::Builtin)));
		assert_eq!(Expr::new(&env).eval("hello(5)"), Err(Error::EnvError(EnvError::NotFound)));
		assert_eq!(Expr::new(&env).eval("hi"), Err(Error::EnvError(EnvError::NotFound)));
	}
}
