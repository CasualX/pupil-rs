//! Operators.

use super::env::BuiltinFn;
use super::builtins::{builtin_add, builtin_sub, builtin_mul, builtin_div, builtin_rem, builtin_pow};

/// Operator precedence.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(i8)]
pub enum Order {
	/// Function barrier hack.
	///
	/// Prevents precedence rules from pushing past a function application.
	/// Only an explicit closing `)` can push past it.
	FnBarrier = 1,
	/// Lowest precedence for operators, helper for `FnBarrier`.
	Operators = 2,
	/// Addition and subtraction precedence.
	AddSub = 3,
	/// Multiplication and division precedence.
	MulDiv = 4,
	/// Implicit multiplication precedence.
	///
	/// This makes implicit mul bind tightly under division allowing `1/2ans` to be evaulated as `1/(2*ans)`.
	/// But not high enough to overpower exponentiation so `2ans^3` will be evaluated as `2*(ans^3)`.
	IMul = 5,
	/// Exponentiation precedence.
	Pow = 6,
	/// Helper for `Pow`’s right associativity.
	PowRightAssoc = 7,
	/// Unary operator precedence.
	Unary = 8,
}

/// Operator associativity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Assoc {
	/// Operator is left associative.
	Left,
	/// Operator is right associative.
	Right,
	/// Operator has no associativity, unimplemented.
	None,
}

/// Supported operator types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Operator {
	/// `+`
	Add,
	/// `-`
	///
	/// Also doubles as unary negation, disambiguated at parser level.
	Sub,
	/// `*`
	Mul,
	/// `/`
	Div,
	/// `%`
	Rem,
	/// Implicit multiplication.
	///
	/// Created on the fly as needed by the parser.
	IMul,
	/// `^`
	Pow,
}

/// Descriptor for an operator’s builtin, precedence, associativity and if available as unary operator.
pub struct OpDesc {
	pub pfn: BuiltinFn,
	pub pre: Order,
	pub assoc: Assoc,
	pub unary: bool,
}

static OP_DESC: [OpDesc; 7] = [
	OpDesc { pfn: builtin_add, pre: Order::AddSub, assoc: Assoc::Left, unary: true },
	OpDesc { pfn: builtin_sub, pre: Order::AddSub, assoc: Assoc::Left, unary: true },
	OpDesc { pfn: builtin_mul, pre: Order::MulDiv, assoc: Assoc::Left, unary: false },
	OpDesc { pfn: builtin_div, pre: Order::MulDiv, assoc: Assoc::Left, unary: false },
	OpDesc { pfn: builtin_rem, pre: Order::MulDiv, assoc: Assoc::Left, unary: false },
	OpDesc { pfn: builtin_mul, pre: Order::IMul, assoc: Assoc::Left, unary: false },
	OpDesc { pfn: builtin_pow, pre: Order::Pow, assoc: Assoc::Right, unary: false },
];

impl Operator {
	/// Get a descriptor for an operator.
	#[inline]
	pub fn desc(self) -> &'static OpDesc {
		&OP_DESC[self as usize]
	}
}
