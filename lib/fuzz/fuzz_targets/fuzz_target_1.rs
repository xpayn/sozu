#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate sozu_lib;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _ = sozu_lib::parser::cookies::parse_request_cookies(data);
});
