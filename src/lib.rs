//! Arithmetic expression evaluator.
//!
//! Start things off by creating its environment which will hold the available builtins and the last answer.
//!
//! ```
//! extern crate pupil;
//! 
//! // Creates an empty environment.
//! let empty = pupil::Env::new();
//! // Creates an environment initialized with the default builtins.
//! let env = pupil::Env::default();
//! ```
//!
//! Create an expression and bind it to its environment.
//!
//! ```
//!# extern crate pupil;
//!# let env = pupil::Env::default();
//! let mut expr = pupil::Expr::new(&env);
//! 
//! // Feed it input, note that you cannot give it partial tokens.
//! expr.feed("2 +").unwrap();
//! expr.feed("3").unwrap();
//! 
//! // Calculate the final result.
//! let result = expr.result().unwrap();
//! ```
//!
//! You can perform the expression evaluation in a single step.
//!
//! ```
//!# extern crate pupil;
//!# let mut env = pupil::Env::default();
//! let result = pupil::Expr::new(&env).eval("2 + 3").unwrap();
//! 
//! // Note that you must update the ‘last answer’ manually like so:
//! env.ans = result;
//! ```

extern crate libc;

pub mod env;
pub mod expr;
pub mod lexer;
pub mod op;
pub mod builtins;

pub use expr::Expr;
pub use env::Env;
