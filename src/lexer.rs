use std::str;
use crate::*;

//----------------------------------------------------------------

/// Token types.
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

#[derive(Clone, Debug)]
struct TokenIterator<'a> {
	string: &'a str,
}

impl<'a> TokenIterator<'a> {
	fn skip_whitespace(&mut self) -> bool {
		// Use Clones instead of Peekable...
		let mut iter = self.string.chars();
		while let Some(chr) = iter.next() {
			if !chr.is_whitespace() {
				return true;
			}
			// Overwrite with previous iterator
			self.string = iter.as_str();
		}
		return false;
	}
	fn lex_lit(&mut self) -> Option<Token<'a>> {
		strtod(self.string).map(|(num, tail_s)| {
			// Update the iterator to right after the number
			self.string = tail_s;
			Token::Lit(num)
		})
	}
	fn lex_op(&mut self) -> Option<Token<'a>> {
		let mut iter = self.string.chars();
		iter.next().and_then(|chr| {
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
			self.string = iter.as_str();
			Some(tok)
		})
	}
	fn lex_id(&mut self) -> Option<Token<'a>> {
		let s = self.string;
		// Scan for a non-alphanumeric character, take whole string otherwise
		let end = s.char_indices()
			.find(|&(_, chr)| !chr.is_alphanumeric())
			.map(|(pos, _)| pos)
			.unwrap_or(s.len());
		// Slice the identifier
		let (s_id, s_rem) = s.split_at(end);
		// Check for opening parenthesis
		let mut paren_it = s_rem.chars();
		// Parenthesis means a function begin
		if paren_it.next() == Some('(') {
			self.string = paren_it.as_str();
			Some(Token::Open(s_id))
		}
		// Otherwise is a variable
		else {
			// Variables can’t have length zero
			if s_id.len() == 0 {
				None
			}
			else {
				self.string = s_rem;
				Some(Token::Var(s_id))
			}
		}
	}
	fn lex_unk(&mut self) -> Option<Token<'a>> {
		// Unknown tokens handled upstream
		// Set the iterator to finish on next() otherwise it would never end
		let s_rem = self.string;
		self.string = "";
		Some(Token::Unk(s_rem))
	}
}

fn strtod(s: &str) -> Option<(f64, &str)> {
	// Yeah let’s go `strtod`!
	// ...
	// Fun fact: Rust strings aren’t zero-terminated, but `strtod` cares...
	// To ‘fix’ this, copy at most 31 bytes form input and zero terminate it.
	// Alternatively malloc some memory with CString but are you mad? It’s just a few bytes.
	// A test was added, I guess that means it’s all good :)
	use std::{mem, ptr};
	unsafe {
		let mut s_num: [libc::c_char; 32] = [0; 32];
		let s_len = usize::min(s.len(), 31);
		(&mut s_num[..s_len]).clone_from_slice(mem::transmute(&s.as_bytes()[..s_len]));
		s_num[s_len] = 0;
		let mut s_end: *mut libc::c_char = ptr::null_mut();
		let num = libc::strtod(s_num.as_ptr(), &mut s_end);
		let read = s_end as usize - s_num.as_ptr() as usize;
		if read != 0 {
			Some((num as f64, &s[read..]))
		}
		else {
			None
		}
	}
}

impl<'a> Iterator for TokenIterator<'a> {
	type Item = Token<'a>;
	fn next(&mut self) -> Option<Token<'a>> {
		// Start by skipping over the whitespace
		if self.skip_whitespace() {
			// Try lexing as various tokens
			self.lex_op()
				.or_else(|| self.lex_lit())
				.or_else(|| self.lex_id())
				.or_else(|| self.lex_unk())
		}
		// End of string
		else {
			None
		}
	}
}

/// Creates an iterator over the tokens in a string.
pub fn tokenize<'a>(string: &'a str) -> impl 'a + Iterator<Item = Token<'a>> {
	TokenIterator { string }
}

#[test]
fn units() {
	use crate::Token::*;
	use crate::Operator::*;
	// Literals, RIP "inf" support
	assert_eq!(tokenize("12.4 45 -0.111").collect::<Vec<_>>(),
		vec![Lit(12.4), Lit(45.0), Op(Sub), Lit(0.111)]);
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
#[test]
fn regressions() {
	// Regression test: fixed `strtod` from reading past the real input
	assert_eq!(strtod(&"1234"[..2]), Some((12.0, "")));
}
