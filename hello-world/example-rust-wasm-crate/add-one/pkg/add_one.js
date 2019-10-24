import * as wasm from './add_one_bg.wasm';

/**
* @param {number} value
* @returns {number}
*/
export function add_one(value) {
    const ret = wasm.add_one(value);
    return ret;
}

