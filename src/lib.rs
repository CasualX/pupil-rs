/*!
Arithmetic expression evaluator
===============================

Start things off by creating an environment which will hold the builtins and variables:

```
extern crate pupil;

// Creates a basic environment with the default builtins.
let env = pupil::BasicEnv::default();
```

Create an expression and bind it to its environment:

```
# let env = pupil::BasicEnv::default();
let mut expr = pupil::Expr::new(&env);

// Feed it input, note that you cannot give it partial tokens.
expr.feed("2 +").unwrap();
expr.feed("3").unwrap();

// Calculate the final result.
let result = expr.result().unwrap();
```

You can perform the expression evaluation in a single step:

```
# let mut env = pupil::BasicEnv::default();
let result = pupil::Expr::new(&env).eval("2 + 3").unwrap();
```
*/

extern crate libc;

pub mod env;
pub mod expr;
pub mod lexer;
pub mod op;
pub mod builtins;

pub use expr::Expr;
pub use env::BasicEnv;
