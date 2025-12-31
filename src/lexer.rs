use super::*;

//----------------------------------------------------------------

/// Token kinds.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind<'a> {
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
	/// Variable symbol, not followed by `(`.
	///
	/// Character set: `[a-zA-Z0-9_.:?!$@#]`
	Var(&'a str),
	/// Function symbol, implicitly followed by `(`.
	///
	/// Character set: `[a-zA-Z0-9_.:?!$@#]`
	Open(&'a str),
	/// Comma token `,`.
	///
	/// Used to provide multiple arguments to a function.
	Comma,
	/// Function closing token `)`.
	Close,
}

/// Token structure.
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
	pub kind: TokenKind<'a>,
	pub position: usize,
}

//----------------------------------------------------------------

// Valid identifier characters lookup table.
static VALID_ID_CHARS: [u8; 16] = {
	const fn set_bit(table: &mut [u8; 16], chr: u8) {
		let index = (chr / 8) as usize;
		let bit = 1 << (chr % 8);
		table[index] |= bit;
	}
	const fn set_bits(table: &mut [u8; 16], start: u8, end: u8) {
		let mut chr = start;
		while chr <= end {
			set_bit(table, chr);
			chr += 1;
		}
	}
	let mut table = [0u8; 16];
	set_bits(&mut table, b'a', b'z');
	set_bits(&mut table, b'A', b'Z');
	set_bits(&mut table, b'0', b'9');
	set_bit(&mut table, b'_');
	set_bit(&mut table, b'.');
	set_bit(&mut table, b':');
	set_bit(&mut table, b'?');
	set_bit(&mut table, b'!');
	set_bit(&mut table, b'$');
	set_bit(&mut table, b'@');
	set_bit(&mut table, b'#');
	table
};

#[derive(Clone, Debug)]
struct TokenIterator<'a> {
	string: &'a str,
	position: usize,
}

impl<'a> TokenIterator<'a> {
	fn skip_whitespace(&mut self) -> bool {
		// Use Clones instead of Peekable...
		let mut iter = self.string.chars();
		while let Some(chr) = iter.next() {
			if !chr.is_whitespace() {
				return true;
			}
			// Track position and overwrite with previous iterator
			self.position += chr.len_utf8();
			self.string = iter.as_str();
		}
		return false;
	}
	fn lex_lit(&mut self) -> Option<TokenKind<'a>> {
		let (num, read) = fast_float::parse_partial(self.string).ok()?;
		self.string = &self.string[read..];
		self.position += read;
		Some(TokenKind::Lit(num))
	}
	fn lex_op(&mut self) -> Option<TokenKind<'a>> {
		let mut iter = self.string.chars();
		iter.next().and_then(|chr| {
			let tok = match chr {
				'+' => TokenKind::Op(Operator::Add),
				'-' => TokenKind::Op(Operator::Sub),
				'*' => TokenKind::Op(Operator::Mul),
				'/' => TokenKind::Op(Operator::Div),
				'%' => TokenKind::Op(Operator::Rem),
				'^' => TokenKind::Op(Operator::Pow),
				',' => TokenKind::Comma,
				')' => TokenKind::Close,
				_ => return None,
			};
			self.string = iter.as_str();
			self.position += chr.len_utf8();
			Some(tok)
		})
	}
	fn lex_id(&mut self) -> Option<TokenKind<'a>> {
		let s = self.string;
		// Scan for a non-alphanumeric character, take whole string otherwise
		let end = s.char_indices()
			.find(|&(_, chr)| if chr as u32 >= 128 {
				false
			} else {
				let byte = chr as u8;
				(VALID_ID_CHARS[(byte / 8) as usize] & (1 << (byte % 8))) == 0
			})
			.map(|(pos, _)| pos)
			.unwrap_or(s.len());
		// Slice the identifier
		let (s_id, s_rem) = s.split_at(end);
		// Check for opening parenthesis
		let mut paren_it = s_rem.chars();
		// Parenthesis means a function begin
		if paren_it.next() == Some('(') {
			self.string = paren_it.as_str();
			Some(TokenKind::Open(s_id))
		}
		// Otherwise is a variable
		else {
			// Variables can’t have length zero
			if s_id.len() == 0 {
				None
			}
			else {
				self.string = s_rem;
				Some(TokenKind::Var(s_id))
			}
		}
	}
	fn lex_unk(&mut self) -> Option<TokenKind<'a>> {
		// Unknown tokens handled upstream
		// Set the iterator to finish on next() otherwise it would never end
		let s_rem = self.string;
		self.position += s_rem.len();
		self.string = "";
		Some(TokenKind::Unk(s_rem))
	}
}

impl<'a> Iterator for TokenIterator<'a> {
	type Item = Token<'a>;
	fn next(&mut self) -> Option<Token<'a>> {
		// Start by skipping over the whitespace
		if self.skip_whitespace() {
			// Record position before lexing the token
			let position = self.position;
			// Try lexing as various tokens
			let kind = self.lex_op()
				.or_else(|| self.lex_lit())
				.or_else(|| self.lex_id())
				.or_else(|| self.lex_unk())?;
			Some(Token { kind, position })
		}
		// End of string
		else {
			None
		}
	}
}

/// Creates an iterator over the tokens in a string.
pub fn tokenize<'a>(string: &'a str) -> impl 'a + Iterator<Item = Token<'a>> {
	TokenIterator { string, position: 0 }
}

#[test]
fn units() {
	use TokenKind::*;
	use Operator::*;
	// Helper to extract just the kinds for comparison
	let kinds = |s: &'static str| tokenize(s).map(|t| t.kind).collect::<Vec<_>>();
	// Literals, RIP "inf" support
	assert_eq!(kinds("12.4 45 -0.111"),
		vec![Lit(12.4), Lit(45.0), Op(Sub), Lit(0.111)]);
	// Functions and Variables
	assert_eq!(kinds("fn(12, (2ans))-pi"),
		vec![Open("fn"), Lit(12.0), Comma, Open(""), Lit(2.0), Var("ans"), Close, Close, Op(Sub), Var("pi")]);
	// All Operators
	assert_eq!(kinds("1%2+3-5*-4/2^1"),
		vec![Lit(1.0), Op(Rem), Lit(2.0), Op(Add), Lit(3.0), Op(Sub), Lit(5.0), Op(Mul), Op(Sub), Lit(4.0), Op(Div), Lit(2.0), Op(Pow), Lit(1.0)]);
	// Unknown
	assert_eq!(kinds("2 + 3 * `èè&"),
		vec![Lit(2.0), Op(Add), Lit(3.0), Op(Mul), Unk("`èè&")]);
}
