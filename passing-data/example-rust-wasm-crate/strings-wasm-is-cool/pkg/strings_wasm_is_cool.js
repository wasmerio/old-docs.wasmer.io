import * as wasm from './strings_wasm_is_cool_bg.wasm';

/**
* @returns {number}
*/
export function get_wasm_memory_buffer_pointer() {
    const ret = wasm.get_wasm_memory_buffer_pointer();
    return ret;
}

/**
* @param {number} passed_string_length
* @returns {number}
*/
export function add_wasm_is_cool(passed_string_length) {
    const ret = wasm.add_wasm_is_cool(passed_string_length);
    return ret >>> 0;
}

