# no_link

This crate intentionally breaks any linking to it. However, it does NOT break just compilation stage
of it.

When you _want_ linking stage to break (under some circumstances that can be determined by
conditional compilation), import `no_link`. That works in the following types of crates:

- binary crates,
- doctests, unit tests, integration tests, procedural macros or shared (dynamic) libraries
- a library crate when used (imported) from your or 3rd party binary crate(s), doctests, unit tests,
  integration tests, procedural macros or shared libraries.

However, this crate does NOT break _compilation_ stage, only the **linking** stage. So, if you use
it in your (standard, static) **library** crate, `cargo build` _will_ succeed - because it does
_not_ involve linking. (The only exception: If you import this crate in a _procedural macro crate_,
and then you use its macro(s) in any other crate, that other crate's `cargo build`, and even `cargo
check`, **will** fail, because those stages execute the procedural macro(s), which _involves_
linking.)

So, if you want to ensure that your (standard, static) **library** crate fails to link (under your
chosen compilation/configuration conditions), import it from a binary crate, a doctest, a unit test
or an integration test, or from a separate local crate. You may want to have exclude such file(s)
from being published to crates.io. See [The Cargo Book > The Manifest Format > The exclude and
include
fields](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields).

You can also check which files would get published by running `cargo package --list`. See [The Cargo
Book > cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html).

## How to use

1. Add it to your crate's `Cargo.toml` -> `[dependencies]` or `[dev-dependencies]`, like:

   ```toml
   [dependencies]
   no-link = version = "0.2.0"
   ```

   or (for tests/doctests only):

   ```toml
   [dev-dependencies]
   no-link = version = "0.2.0"
   ```

   You _don't_ need to make it an optional dependency - it's enough to make its import conditional.
   See below.
2. Import it. You most likely want that to be under some [conditional
   compilation](https://doc.rust-lang.org/nightly/reference/conditional-compilation.html). For
   example:

   ```rust
   #[cfg( feature = "feature-name-when-you-want-linking-to-fail" )]
   use no_link as _;
   ```

See also [`separate_test_crate/`](separate_test_crate/).

## Negative tests

If you want to test that (under some configuration) your crate will _not_ link, don't use `trybuild`
crate (see [`prudent-rs/readme-code-extractor` -> negative_tests_runner/src/lib.rs (commit
ff8935f)](https://github.com/prudent-rs/readme-code-extractor/blob/ff8935ff314133ddc432c32d0ec89c41f4dd0dd0/negative_tests_runner/src/lib.rs)).
Use crate [snapbox](https://docs.rs/snapbox/latest/snapbox/).

See also [`prudent-rs/readme-code-extractor` -> negative_tests_runner/src/lib.rs (commit
1793008)](https://github.com/prudent-rs/readme-code-extractor/blob/1793008f755a6854543b39162b5917f686265184/negative_tests_runner/src/lib.rs),
which does _not_ use `no_link`, but it verifies/tests an expected linking failure (generated with
`dtolnay/no_panic`).

## Zero cost abstraction

- No dependencies.
- Zero size/memory/speed cost and no modifications to any binaries/shared libraries (since those can
  get build only if `no-link` is _not_ imported).
- Minimum build time cost: Small, no proc macros used. Its build script [build.rs](build.rs) is
  lightweight.
