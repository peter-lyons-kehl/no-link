# no_link

This crate intentionally breaks any linking to it.

## How to use

In addition to listing it in your crate's `Cargo.toml` -> [dependencies] or [dev-dependencies], you also need to import it (with `use no_link::*`, or with `use no_link::no_link as _;` if you care about your namespace).

You most likely want that to be under some conditional compilation.

