Pupil
=====

Arithmetic expression evaluator written in Rust.

It implements a butchered [Shunting-yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm).

Pupil
-----

To build the pupil executable, run `cargo build`. Append the `--release` switch as needed for optimized builds.

It has three different use cases:

* Interactive mode.

  Enter expressions and press enter to evaluate them.

  ```text
  PATH/TO/CRATE/ROOT> cargo run
  Welcome to pupil, the arithmetic expression evaluator.

  Enter an expression, eg. 2 + 3, and press enter.
  Press ctrl-C to exit.

  >>> 2 + 3
  5
  >>> ^C
  ```

* Provide the expression to evaluate as command line arguments.

  This allows to evaluate a single expression and then exit.

  ```text
  PATH/TO/CRATE/ROOT> cargo run -- 2 + 3
  Welcome to pupil, the arithmetic expression evaluator.
  Ok: 5
  ```

* Pipe input.

  Evaluates every line as separate expressions and prints the result line by line.

  ```text
  PATH/TO/CRATE/ROOT> echo 2 + 3 | cargo run
  5
  ```

Library
-------

This library can be found on [crates.io](https://crates.io/crates/pupil). In your `Cargo.toml` put:

```
[dependencies]
pupil = "0.1"
```

A practical example can be found in `src/bin/pupil.rs`.

Documentation can be found online [here](https://casualx.github.io/pupil-rs/0.1.3/pupil).

Start things off by creating its environment which will hold the available builtins and the last answer.

```rust
extern crate pupil;

// Creates an empty environment.
let empty = pupil::Env::new();
// Creates an environment initialized with the default builtins.
let env = pupil::Env::default();
```

Create an expression and bind it to its environment.

```rust
let mut expr = pupil::Expr::new(&env);

// Feed it input, note that you cannot give it partial tokens.
expr.feed("2 +").unwrap();
expr.feed("3").unwrap();

// Calculate the final result.
let result = expr.result().unwrap();
```

You can perform the expression evaluation in a single step.

```rust
let result = pupil::Expr::new(&env).eval("2 + 3").unwrap();
```

That’s it.

License
-------

MIT - See license.txt
