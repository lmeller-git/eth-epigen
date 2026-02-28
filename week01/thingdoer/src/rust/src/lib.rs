use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// add to nums
/// @export
#[extendr]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// does the thing
/// @export
#[extendr]
fn do_the_thing() -> Result<Robj> {
    let f = R!("sessionInfo")?;
    f.call(pairlist!())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod thingdoer;
    fn hello_world;
    fn add;
    fn do_the_thing;
}
