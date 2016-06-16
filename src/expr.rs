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
	/// Create a new expression and bind it to an environment.
	pub fn new(env: &'a Env) -> Expr<'a> {
		Expr {
			env: env,
			fns: Vec::new(),
			vals: Vec::new(),
			next: State::Val,
		}
	}
	/// Parse a token.
	pub fn parse(&mut self, tok: Token) -> Result<(), Error> {
		match self.next {
			State::Op => self.parse_op(tok),
			State::Val => self.parse_val(tok),
		}
	}
	/// Feed new input to be parsed and evaluated.
	pub fn feed(&mut self, input: &str) -> Result<(), Error> {
		// Tokenize and parse the input
		for tok in tokenize(input) {
			// Dispatch based on a simple state machine:
			//  expect either an operator or value like token.
			try!(self.parse(tok));
		}
		Ok(())
	}
	/// Finalize the expression and calculate the final result.
	pub fn result(mut self) -> Result<Value, Error> {
		// Must end at a value like token
		if self.next == State::Val {
			return Err(Error::UnfinishedExpression);
		}
		// Evaluate all pending operators
		try!(self.eval_while(Order::Operators));
		// Expect exactly one result
		if self.vals.len() != 1 || self.fns.len() != 0 {
			return Err(Error::UnbalancedParens);
		}
		// Return the result
		Ok(self.vals[0])
	}
	/// Convenience method combines `feed` and `result`.
	pub fn eval(mut self, input: &str) -> Result<Value, Error> {
		try!(self.feed(input));
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
				// Lookup the symbol
				let pfn = try!(self.env.find(id).ok_or(Error::UnknownSymbol));
				// Just evaluate it now
				let result = try!(pfn(self.env, &mut []));
				// And push the resulting value
				self.vals.push(result);
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
			Token::Open(id) => {
				// Lookup the symbol
				let pfn = try!(self.env.find(id).ok_or(Error::UnknownSymbol));
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
				let pre = match desc.assoc {
					Assoc::Left => desc.pre,
					Assoc::Right => Order::PowRightAssoc,
					Assoc::None => return Err(Error::InternalCorruption),
				};
				// Evaluate all lower precedence fns
				// HACK! This evaluates precedence with >= which is correct for left associativity
				//       Right associativity needs a plain > which is basically the same as precedence + 1 but it’s an enum so bleh...
				try!(self.eval_while(pre));
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
				try!(self.parse_op(Token::Op(Operator::IMul)));
				// Retry inserting this token
				self.parse_val(tok)
			},
			Token::Open(_) => {
				// Insert implicit multiplication token
				try!(self.parse_op(Token::Op(Operator::IMul)));
				// Retry inserting this token
				self.parse_val(tok)
			},
			Token::Comma => {
				// Eval until an fn barier
				try!(self.eval_while(Order::Operators));
				// Increment nargs for that fn
				try!(self.fns.last_mut().ok_or(Error::MisplacedComma)).nargs += 1;
				// Followed by a value
				self.next = State::Val;
				Ok(())
			},
			Token::Close => {
				// Eval everything until the fn barrier and push past it
				try!(self.eval_while(Order::Operators));
				try!(self.eval_apply());
				// Followed by an operator
				self.next = State::Op;
				Ok(())
			},
		}
	}
	// Eval all fns with higher or equal precedence.
	fn eval_while(&mut self, pre: Order) -> Result<(), Error> {
		while self.fns.last().map(|f| f.pre >= pre).unwrap_or(false) {
			try!(self.eval_apply());
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
				try!((f.pfn)(self.env, vals))
			};
			// Pop vals and push result
			{ self.vals.drain(args.clone()) };
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
		let env = Env::new();
		assert_eq!(Expr::new(&env).eval("2 + 3"), Ok(5.0));
		assert_eq!(Expr::new(&env).eval("2-3*4"), Ok(-10.0));
		assert_eq!(Expr::new(&env).eval("2*3+4"), Ok(10.0));
		assert_eq!(Expr::new(&env).eval("3^2-2"), Ok(7.0));
		assert_eq!(Expr::new(&env).eval("2+---2"), Ok(0.0));
		assert_eq!(Expr::new(&env).eval("-1"), Ok(-1.0));
	}
	#[test]
	fn funcs() {
		let env = Env::default();
		assert_eq!(Expr::new(&env).eval("2*(3+4)"), Ok(14.0));
		assert_eq!(Expr::new(&env).eval("mul(2,add(3,4))"), Ok(14.0));
	}
	#[test]
	fn errors() {
		let env = Env::default();
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
		assert_eq!(Expr::new(&env).eval("hello(5)"), Err(Error::UnknownSymbol));
		assert_eq!(Expr::new(&env).eval("hi"), Err(Error::UnknownSymbol));
	}
}
