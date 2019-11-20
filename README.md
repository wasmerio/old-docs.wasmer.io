# wasmer_rust_playground
Playground for the Wasmer Rust Integration

# Notes

* Maybe make a quick start for C? Since there are soooo many large amounts of code with moving parts. It may be good to have a "Hey, copy paste this code, and it'll work like 99% of the time if yoou fill in the gaps correctly".

## Passing Memory

Awesome tip to do in the guest for better allocation. Thanks Mark!

```
fn allocate_memory(size: u32) -> u32 {
  let buffer: Vec<u8> = std::iter::repeat(0).take(size as usize).collect();
  let boxed_slice = buffer.as_boxed_slice();
  Box::into_raw(boxed_slice) as u32
}
```

```
fn free_memory(ptr: u32) -> u32 {
  let _ = unsafe { Box::from_raw( ptr as *const Box<[u8]>) } ;
}
```
