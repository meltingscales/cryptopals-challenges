#![cfg_attr(
    debug_assertions,
    allow(
        dead_code,
        unused_variables,
        unused_imports,
        unused_parens,
        non_snake_case
    )
)]

// see https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64#Javascript

mod base64;
mod bitstring;
mod byteutil;
mod util;

mod set01;

fn main() {
    set01::challenge01::runTest();
    set01::challenge02::runTest();
}
