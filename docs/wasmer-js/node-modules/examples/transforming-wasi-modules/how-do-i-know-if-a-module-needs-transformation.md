---
id: wasmer-js-modules-how-do-i-know-if-a-module-needs-transformation
title: Wasmer-JS in the Browser
sidebar_label: How Do I Know if a Module Needs Transformation?
---

# How Do I Know if a WASM Module Needs Transformation?

## The `clock_time_get` WebAssembly Module

In this case, the native "OS" function we want to call is `clock_time_get` and hopefully, the interfaces it uses have been well documented.

Meanwhile, back in reality...

In order to understand whether or not this module needs transformation, we need to take a look inside the WebAssembly module.

When converted to [WebAssembly Text](https://webassembly.github.io/spec/core/text/index.html) format, the first few lines of `clock_time_get.wasm` looks like this:

```WebAssemblyText
(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t1)))

  ;; snip...
```

On line 2, we can see the declaration of a type definition called `$t0`.  This type definition represents the interface to some `func`tion that takes three, signed integers as parameters and returns an integer.

```WebAssemblyText
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter.  Uh oh! Its a 64-bit signed integer!

Then on line 5, we can see the declaration of the call to `clock_time_get`:

```WebAssemblyText
(import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
```

Two things are important to notice here:

1. The `import` keyword indicates that the native function `clock_time_get` lives in an external module called `wasi_unstable`

1. The interface to this function is described by the previously declared type definition `$t0`.  In other words, we know for certain that function `clock_time_get` must be passed an `i64` as its second parameter; therefore, the interface to this function must be transformed

Having established exactly what the interface is to `clock_time_get`, we now know that we cannot call this WASM module without first transforming it.

### Important

This example is somewhat contrived because the WebAssembly module has been hard-coded to return the text string `Done!` rather than the value returned from `clock_time_get`.  This is because this module writes its output to standard out, which in turn, expects to receive printable strings followed by a carriage return character, not the raw `i32` value returned from `clock_time_get`.
