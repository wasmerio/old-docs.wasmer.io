import {
  proc_exit,
  fd_write,
  clock_time_get
} from "bindings/wasi_unstable";

/**
 * Modified from MIT Licensed as-wasi: 
 * https://github.com/jedisct1/as-wasi/blob/master/assembly/as-wasi.ts#L300
 */
@global
function wasi_abort(
  message: string = "",
  fileName: string = "",
  lineNumber: u32 = 0,
  columnNumber: u32 = 0
): void {
  proc_exit(255);
}

/**
 * Modified from MIT Licensed as-wasi: 
 * https://github.com/jedisct1/as-wasi/blob/master/assembly/as-wasi.ts#L300
 *
 * Write a string to a stdout, after encoding it to UTF8
 * @param s string
 */
function println(s: string): void {
  let s_utf8_len: usize = String.UTF8.byteLength(s);
  let s_utf8 = changetype<usize>(String.UTF8.encode(s));
  let iov = changetype<usize>(new ArrayBuffer(4 * sizeof<usize>()));
  store<u32>(iov, s_utf8);
  store<u32>(iov + sizeof<usize>(), s_utf8_len);
  let lf = changetype<usize>(new ArrayBuffer(1));
  store<u8>(lf, 10);
  store<u32>(iov + sizeof<usize>() * 2, lf);
  store<u32>(iov + sizeof<usize>() * 3, 1);
  let written_ptr = changetype<usize>(new ArrayBuffer(sizeof<usize>()));
  fd_write(1, iov, 2, written_ptr);
}

// Entry point into the WASI Module
export function _start(): void {

  // Allocate the space for the current clock_time
  let clockTimeGetResponseBuffer: i32 = __alloc(4, 0);

  // Call the clock_time_get WASI binding
  let statusCode = clock_time_get(0, 1000, clockTimeGetResponseBuffer);

  if (statusCode === 0) {

    // Get the value that was placed into the buffer
    let clockTimeGetResponse = load<i32>(clockTimeGetResponseBuffer);

    // Output the response from clock_time_get
    println("Success running clock_time_get. Response: " + clockTimeGetResponse.toString());
  } else {
    // There was an error
    println("Error running clock_time_get, errno: " + statusCode.toString());
  }
}

