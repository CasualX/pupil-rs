/*!
Arithmetic expression evaluator
===============================

Simple usage:

```
// Create a basic environment with the default builtins
let mut env = pupil::BasicEnv::default();

// Evaluate expressions in this environment
let result = pupil::eval(&env, "2 + 3");
assert_eq!(result, Ok(5.0));
```
*/

use std::f64;

mod env;
mod error;
mod expr;
mod lexer;
mod native;
mod op;

pub use self::env::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::lexer::*;
pub use self::op::*;

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}
