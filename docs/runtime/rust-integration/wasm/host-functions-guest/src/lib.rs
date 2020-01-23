// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Define a function that is imported into the module.
// By default, the "env" namespace is used.
//
// We aren't using wasm-bindgen here, as we want to
// handle the imports ourselves with our host Wasmer app.
extern "C" {
    fn get_counter() -> i32;
    fn add_to_counter(value_to_add: i32) -> i32;
}

#[no_mangle]
pub fn increment_counter_loop(number_of_times: i32) -> i32 {
    let mut current_counter;
    unsafe {
        current_counter = get_counter();
    }

    for _i in 0..number_of_times {
        unsafe {
            current_counter = add_to_counter(1);
        }
    }

    current_counter
}
