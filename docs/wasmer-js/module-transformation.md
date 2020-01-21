---
id: wasmer-js-module-transformation
title: Transforming WASI Modules
sidebar_label: Data Transfer Between JavaScript & WASM
---

# What is Module Transformation and Why is it Necessary?

In the Browser-based [Hello World](./client/examples/hello-world/wasmer-js-client-hello-world) example, we call a very basic WASM module called `as-echo` that receives a single text string as an argument, and simply echoes it back via standard out.

In this case, the values passed from WebAssembly to the native "OS" function that writes to standard out, are all compatible with JavaScript data types.  However, some WASI modules might contain function calls whose interfaces are not compatible, and therefore, such modules cannot immediately be called.

## 64-bit Integers

The real issue here centres on transferring 64-bit integers between the two runtime environments (whether signed or unsigned is not important here).

Both JavaScript and WebAssembly use this data type (known as `i64` or `u64` in WebAssembly and `BigInt` in JavaScript); but for a variety of reasons, the transfer of this data type has not yet been implemented and is still at the proposal stage.  (See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details).

> ### IMPORTANT
>
> Irrespective of whether your JavaScript app runs on the client or the server, the interface to any WASI module call that has been declared to use either an `i64` or a `u64` must first be ***transformed*** before it can be called.

For example, you will experience this problem if you try to pass a parameter value from JavaScript, through WebAssembly and then on to the native "OS" function [clock\_time\_get](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---errno-timestamp).

As a temporary fix, this data transfer issue is solved by the `@wasmer/wasm-transformer` package.

> ### Under The Hood  
> Technically, this transformation adapts the WebAssembly interface so that it can send and receive JavaScript `BigInt`s (64-bit, signed integers).
>
> No data loss occurs here because a JavaScript `BigInt` is transformed into a `Uint8Array` containing 8, unsigned, 8-bit integers.

# How Do I Know if a WASM Module Needs Transformation?

Good question!

Normally, you would look at the well-written interface documentation for the WebAssembly module to see which native "OS" functions it calls, and then discover what data types are used for the interface parameters.

Ok, back in reality...

## The `clock_time_get` WebAssembly Module

In order to understand whether or not this module needs transformation, we need to take a look inside the WebAssembly module.

> ### ASIDE
> We make no attempt to teach you WebAssembly here!
>
> If you want to know about the inner workings of WebAssembly, please visit the [WebAssembly.org](https://webassembly.org) website and read the documentation there.
>
> We now continue with your scheduled program...

When converted to [WebAssembly Text](https://webassembly.github.io/spec/core/text/index.html) format, the first few lines look like this:

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

1. Within a native "OS" library called `wasi_unstable`, there is a function called `clock_time_get`
1. Within our WebAssembly module, this function will be know by the alias `$wasi_unstable.clock_time_get`
1. The interface to this function is described by the type declaration `$t0`

We know from the definition of `$t0` (on line 2) that this function must be passed an `i64` as its second parameter; therefore, we can be certain that before function `clock_time_get` can be called from JavaScript, the interface to this module must be transformed.
