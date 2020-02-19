#![no_main]

#[no_mangle]
pub fn add_one(value: i32) -> i32 {
    return value + 1;
}
