#![no_main]

// Define the functions that this module will use from the outside world.
// In general, the set of this functions is what we define as an ABI.
// Here we define the "host" namespace for the imports,
// Otherwise it will be "env" by default
#[link(wasm_import_module = "host")]
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
