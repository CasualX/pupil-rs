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

mod env;
mod expr;
mod lexer;
mod op;
pub mod builtins;

pub use env::*;
pub use expr::*;
pub use lexer::*;
pub use op::*;

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}
