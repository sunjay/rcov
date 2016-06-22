# rcov - Rust Code Coverage

Building off of the techniques in [this paper](http://www.semdesigns.com/Company/Publications/TestCoverage.pdf), I will build a code
coverage tool for Rust, in Rust.

## Installation
This won't work until this is properly finished, but I anticipate installation will look something like this:

**Note: This is currently only supported on Rust nightly because the plugin API is not on the stable channel**

Add the crate to your [`dev-dependencies` section](http://doc.crates.io/specifying-dependencies.html#development-dependencies) in `Cargo.toml`:

```
[dev-dependencies]
rcov = "stable"
```

Add these lines to the top of your executable's main file:

```
#![feature(plugin)]
#![plugin(rcov)]

#![coverage]
```

These lines enable the required feature, load the plugin and apply the coverage syntax extension to the entire crate.

Running `cargo test` for your project should now instrument your code and result in a code coverage file being produced.

TODO: There should be some way to make coverage optionally compile only in the test executable (using `cfg(test)`?)

## Build & Test
Standard `cargo` commands apply. 

Use this to run the example:

```
cargo run --example sample
```

Use this to run the tests and the examples too
```
cargo test
```

