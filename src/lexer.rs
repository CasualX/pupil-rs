//! Lexing.

use ::std::{str, mem};
use ::libc;
use super::env::Value;
use super::op::Operator;

//----------------------------------------------------------------

/// Supported tokens types.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
	/// Unknown token.
	///
	/// It’s the caller’s responsibility to handle this with an error of some kind.
	///
	/// This will swallow the entire remainder of the input, such that the tokenization finishes on next iteration.
	Unk(&'a str),
	/// Value literal token.
	///
	/// Negative literals are `Token::Op(Operator::Sub)` followed by a positive literal.
	Lit(Value),
	/// Operator token.
	Op(Operator),
	/// Variable token.
	///
	/// Alphanumeric characters only. Not followed by a `(`.
	Var(&'a str),
	/// Function token.
	///
	/// Alphanumeric characters only. Implicitly followed by a `(`.
	Open(&'a str),
	/// Comma token `,`.
	///
	/// Used to provide multiple arguments to a function.
	Comma,
	/// Function closing token `)`.
	Close,
}

//----------------------------------------------------------------

/// Iterator over tokens in a string.
pub struct TokenIterator<'a> {
	iter: str::Chars<'a>,
}

impl<'a> TokenIterator<'a> {
	fn skip_whitespace(&mut self) -> bool {
		// Use Clones instead of Peekable...
		let mut iter = self.iter.clone();
		while let Some(chr) = iter.next() {
			if !chr.is_whitespace() {
				return true;
			}
			// Overwrite with previous iterator
			self.iter = iter.clone();
		}
		return false;
	}
	fn lex_lit(&mut self) -> Option<Token<'a>> {
		let s = self.iter.as_str();
		// Yeah let’s go `strtod`!
		unsafe {
			let s_num = s.as_bytes().as_ptr() as *const libc::c_char;
			let mut s_end: *mut libc::c_char = mem::uninitialized();
			let num = libc::strtod(s_num, &mut s_end);
			if s_num != s_end {
				// Update the iterator
				self.iter = s.slice_unchecked(s_end as usize - s_num as usize, s.len()).chars();
				Some(Token::Lit(num))
			}
			else {
				None
			}
		}
	}
	fn lex_op(&mut self) -> Option<Token<'a>> {
		let mut iter = self.iter.clone();
		if let Some(chr) = iter.next() {
			let tok = match chr {
				'+' => Token::Op(Operator::Add),
				'-' => Token::Op(Operator::Sub),
				'*' => Token::Op(Operator::Mul),
				'/' => Token::Op(Operator::Div),
				'%' => Token::Op(Operator::Rem),
				'^' => Token::Op(Operator::Pow),
				',' => Token::Comma,
				')' => Token::Close,
				_ => return None,
			};
			self.iter = iter;
			Some(tok)
		}
		else {
			None
		}
	}
	fn lex_id(&mut self) -> Option<Token<'a>> {
		let s = self.iter.as_str();
		// Scan for a non-alphanumeric character, take whole string otherwise
		let end = s.char_indices()
			.find(|&(_, chr)| !chr.is_alphanumeric())
			.map(|(pos, _)| pos)
			.unwrap_or(s.len());
		// Slice the identifier
		let (s_id, s_rem) = s.split_at(end);
		// Check for opening parenthesis
		let mut paren_it = s_rem.chars();
		let paren = if let Some(chr) = paren_it.next() { chr == '(' } else { false };
		// Parenthesis means a function begin
		if paren {
			self.iter = paren_it;
			Some(Token::Open(s_id))
		}
		// Otherwise is a variable
		else {
			// Variables can’t have length zero
			if s_id.len() == 0 {
				None
			}
			else {
				self.iter = s_rem.chars();
				Some(Token::Var(s_id))
			}
		}
	}
	fn lex_unk(&mut self) -> Option<Token<'a>> {
		// Unknown tokens handled upstream
		// Set the iterator to finish on next() otherwise it would never end
		let s_rem = self.iter.as_str();
		self.iter = "".chars();
		Some(Token::Unk(s_rem))
	}
}

impl<'a> Iterator for TokenIterator<'a> {
	type Item = Token<'a>;
	fn next(&mut self) -> Option<Token<'a>> {
		// Start by skipping over the whitespace
		if self.skip_whitespace() {
			// Try lexing as various tokens
			if let tok @ Some(_) = self.lex_op() { tok }
			else if let tok @ Some(_) = self.lex_lit() { tok }
			else if let tok @ Some(_) = self.lex_id() { tok }
			else { self.lex_unk() }
		}
		// End of string
		else {
			None
		}
	}
}

/// Create a new TokenIterator for a string.
pub fn tokenize<'a>(input: &'a str) -> TokenIterator<'a> {
	TokenIterator {
		iter: input.chars(),
	}
}

#[cfg(test)]
mod tests {
	use super::tokenize;
	use super::Token::*;
	use super::super::op::Operator::*;

	#[test]
	fn units() {
		// Literals, can’t test NAN because reasons
		assert_eq!(tokenize("12.4 45 -0.111 inf").collect::<Vec<_>>(),
			vec![Lit(12.4), Lit(45.0), Op(Sub), Lit(0.111), Lit(::std::f64::INFINITY)]);
		// Functions and Variables
		assert_eq!(tokenize("fn(12, (2ans))-pi").collect::<Vec<_>>(),
			vec![Open("fn"), Lit(12.0), Comma, Open(""), Lit(2.0), Var("ans"), Close, Close, Op(Sub), Var("pi")]);
		// All Operators
		assert_eq!(tokenize("1%2+3-5*-4/2^1").collect::<Vec<_>>(),
			vec![Lit(1.0), Op(Rem), Lit(2.0), Op(Add), Lit(3.0), Op(Sub), Lit(5.0), Op(Mul), Op(Sub), Lit(4.0), Op(Div), Lit(2.0), Op(Pow), Lit(1.0)]);
		// Unknown
		assert_eq!(tokenize("2 + 3 * !èè&").collect::<Vec<_>>(),
			vec![Lit(2.0), Op(Add), Lit(3.0), Op(Mul), Unk("!èè&")]);
	}
}