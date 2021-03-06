#![no_main]

#[macro_use] extern crate libfuzzer_sys;
extern crate image;

fuzz_target!(|data| {
    let _ = image::load_from_memory(data);
});
