#![no_main]

#[no_mangle]
pub fn throw_wasm_error() {
    panic!("Causing a Wasm trap in `throw_wasm_error`");
}
