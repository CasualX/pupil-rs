use std::{error, fmt};

/// Error kinds.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
	/// Something went wrong unexpectedly.
	///
	/// This is a bug.
	InternalError,
	/// Expected an operator-like thing.
	///
	/// Eg. `12 5`. Expected an operator instead of `5`.
	ExpectOperator,
	/// Expected a value-like thing.
	NaExpression,
	/// Disallowed unary operator.
	///
	/// Only `+` and `-` are allowed as unary operators.
	DisallowedUnary,
	/// Expression isn’t finished, cannot end with an operator.
	///
	/// Eg. `2 +`.
	UnfinishedExpression,
	/// Tokenization failed to lex a token.
	InvalidToken,
	/// Different number of `(` and `)`.
	UnbalancedParens,
	/// Misplaced a comma token outside of a function application.
	MisplacedComma,
	/// Bad number of arguments.
	BadArgument,
	/// A variable or function symbol wasn’t found.
	NameNotFound,
}

impl error::Error for ErrorKind {}

impl fmt::Display for ErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let desc = match self {
			ErrorKind::InternalError => "internal error",
			ErrorKind::ExpectOperator => "expected an operator",
			ErrorKind::NaExpression => "not an expression",
			ErrorKind::DisallowedUnary => "not an unary operator",
			ErrorKind::UnfinishedExpression => "unfinished expression",
			ErrorKind::InvalidToken => "invalid token",
			ErrorKind::UnbalancedParens => "unbalanced parens",
			ErrorKind::MisplacedComma => "misplaced comma",
			ErrorKind::BadArgument => "bad argument",
			ErrorKind::NameNotFound => "name not found",
		};
		desc.fmt(f)
	}
}

/// Error structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Error {
	pub kind: ErrorKind,
	pub position: usize,
}

impl Error {
	/// Create a diagnostic display for this error with the given input string.
	///
	/// Example "1+":
	/// ```text
	/// 1+
	///  ^
	/// error: unfinished expression
	/// ```
	#[inline]
	pub fn diagnostic<'a>(self, input: &'a str) -> impl fmt::Display + 'a {
		let (line, carret_pos) = compute_carret_pos(input, self.position);
		ErrorWithInput { error: self, line, carret_pos, show_input: true }
	}

	/// Create a compact diagnostic display for this error with the given input string.
	///
	/// Does not show the input line, only the caret and error message.
	///
	/// Example "1+":
	/// ```text
	///  ^
	/// error: unfinished expression
	/// ```
	#[inline]
	pub fn compact_diagnostic<'a>(self, input: &'a str) -> impl fmt::Display + 'a {
		let (line, carret_pos) = compute_carret_pos(input, self.position);
		ErrorWithInput { error: self, line, carret_pos, show_input: false }
	}
}

impl error::Error for Error {}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "error at position {}: {}", self.position, self.kind)
	}
}

fn compute_carret_pos(input: &str, position: usize) -> (&str, usize) {
	// Walk the string to find the start of the line containing the error
	let mut line_start = 0;
	for (idx, b) in input.bytes().enumerate() {
		if idx >= position {
			break;
		}

		if b == b'\n' {
			line_start = idx + 1;
		}
	}

	// Find the end of that line
	let line_end = input[line_start..]
		.find('\n')
		.map(|i| line_start + i)
		.unwrap_or(input.len());
	let line = &input[line_start..line_end];

	let column = position.saturating_sub(line_start);
	(line, column)
}

struct ErrorWithInput<'a> {
	error: Error,
	line: &'a str,
	carret_pos: usize,
	show_input: bool,
}

impl<'a> fmt::Display for ErrorWithInput<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.show_input {
			writeln!(f, "{}", self.line)?;
		}
		writeln!(f, "{:>width$}", "^", width = self.carret_pos + 1)?;
		writeln!(f, "error: {}", self.error.kind)
	}
}
