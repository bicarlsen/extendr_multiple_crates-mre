# `extendr` multiple crates MRE
MRE to test allowing multiple local crates for `extendr`.
See [Issue #532](https://github.com/extendr/extendr/issues/532).

## Use
Currently adding a local dependency to an `extendr` project causes it to fail.
This can be tested by un/commenting the line in `myextendr/src/rust/Cargo.toml > dependencies` for `my-extendr-lib`.