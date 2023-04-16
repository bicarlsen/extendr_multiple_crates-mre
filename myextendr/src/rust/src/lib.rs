use extendr_api::prelude::*;

/// Return string `"Hello extendr"` to R.
/// @export
#[extendr]
fn hello() -> &'static str {
    "Hello extendr"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod myextendr;
    fn hello;
}
