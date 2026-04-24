# no_link

This crate intentionally breaks any linking to it. However, it does NOT break just compilation stage
of it.

Import it (under some circumstances that can be determined by conditional compilation) in the
following when you don't want linking stage to break:

- your binary crates,
- your doctests, unit tests, integration tests, procedural macros or shared libraries
- your library crate when used (imported) from your or 3rd party binary crate(s), doctests, unit
  tests, integration tests, procedural macros or shared libraries.

However, this crate does NOT break _compilation_ stage, only the **linking** stage. If you use it in
your (standard, static) **library** crate, `cargo build` _will_ succeed - because it does _not_
involve linking. (The only exception: If you import this crate in a procedural macro, and then you
use that macro in any other crate, that other crate's `cargo build` and even `cargo check` **will**
fail, because those stages execute the procedural macro.)

So, if you want to ensure that your (standard, static) **library** crate fails to link (under your
chosen compilation/configuration conditions), import it from a binary crate, a doctest, a unit test
or an integration test, or from a separate local crate. You may want to have exclude such file(s)
from being published to crates.io. See [The Cargo Book > The Manifest Format > The exclude and
include
fields](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields). You
can also check which files would get published by running `cargo package --list`. See [The Cargo
Book > cargo-package](https://doc.rust-lang.org/cargo/commands/cargo-package.html).

## How to use

1. Add it to your crate's `Cargo.toml` -> `[dependencies]` or `[dev-dependencies]`, like:

   ```toml
   [dependencies]
   no-link = version = "0.1.0"
   ```

   or (for tests/doctests only):

   ```toml
   [dev-dependencies]
   no-link = version = "0.1.0"
   ```

   You _don't_ need to make it an optional dependency - it's enough to make its import conditional.
   See below.
2. Import it. You most likely want that to be under some [conditional
   compilation](https://doc.rust-lang.org/nightly/reference/conditional-compilation.html). For
   example:

   ```rust
   #[cfg( feature = "feature-name-when-you-want-linking-to-fail" )]
   #[allow(unused_imports)]
   use no_link::*;
   ```

   or, to make it explicit that it doesn't touch your namespace:

   ```rust
   #[cfg( feature = "feature-name-when-you-want-linking-to-fail" )]
   use no_link::no_link as _;
   ```

## Zero cost abstraction

- No dependencies.
- Zero size/memory/speed cost and no modifications to any binaries/shared libraries (since those can
  get build only if `no-link` is _not_ imported).
- Minimum build time cost: Small, no proc macros used. Its build script [build.rs](build.rs) is
  lightweight.
