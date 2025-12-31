Pupil
=====

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/pupil.svg)](https://crates.io/crates/pupil)
[![docs.rs](https://docs.rs/pupil/badge.svg)](https://docs.rs/pupil)
[![Build status](https://github.com/CasualX/pupil-rs/workflows/CI/badge.svg)](https://github.com/CasualX/pupil-rs/actions)

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

This library can be found on [crates.io](https://crates.io/crates/pupil).

A practical example can be found in [examples/pupil.rs](examples/pupil.rs).

Documentation can be found on [docs.rs](https://docs.rs/pupil).

In your `Cargo.toml` put:

```toml
[dependencies]
pupil = "1.0"
```

Usage
-----

Simple usage:

```rust
// Create a basic environment with the default builtins
let mut env = pupil::BasicEnv::default();

// Evaluate expressions in this environment
let result = pupil::eval(&env, "2 + 3");
assert_eq!(result, Ok(5.0));
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
