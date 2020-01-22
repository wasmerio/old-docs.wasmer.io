---
id: wasmer-js-modules-how-do-i-know-if-a-module-needs-transformation
title: Wasmer-JS in the Browser
sidebar_label: How Do I Know if a Module Needs Transformation?
---

# How Do I Know if a WASM Module Needs Transformation?

Good question!

Normally, you would look at the well-written interface documentation for the WebAssembly module to see which native "OS" functions it calls and to discover what data types are used for the interface parameters.

Ok, back in reality...

## The `clock_time_get` WebAssembly Module

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

On line 2, we can see the declaration of a type definition called `$t0`.  This type definition represents the interface to some `func`tion that takes three, signed integers as parameters and returns a signed integer.

```WebAssemblyText
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter.  Uh oh! Its an `i64`; that is, a 64-bit, signed integer!

So now we know that somewhere in this WebAssembly module, there is a call to function that uses this interface declaration.

Next, look a little further down to line 5.  Here we can see an `import` statement.

```WebAssemblyText
(import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
```

This statement tells us three things:

1. Within the native "OS" library `wasi_unstable`, there is a function called `clock_time_get`
1. Within our WebAssembly module, this function will be know by the alias `$wasi_unstable.clock_time_get`
1. The interface to this function is described by the type declaration `$t0`

We know from the definition of `$t0` (on line 2) that this function must be passed an `i64` as its second parameter; therefore, we can be certain that before function `clock_time_get` can be called from JavaScript, the interface to this module must be transformed.
