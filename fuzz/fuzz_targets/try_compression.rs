#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let s = String::from_utf8_lossy(data);
    println!("Fuzz: {}", s);
    let _ = atm::directives::gen::try_compression_from_str(&s);
});