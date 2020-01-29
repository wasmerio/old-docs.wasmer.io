---
id: wasmer-js-module-transformation
title: Transforming WASI Modules
sidebar_label: Data Transfer Between JavaScript & WASM
---

# What is Module Transformation and Why is it Necessary?

In the Browser-based [Hello World](./client/examples/hello-world/wasmer-js-client-hello-world) example, we call a WASM module called `as-echo` that does nothing more than receive a text string as an argument, and echo it back via standard out.

In this case, the values passed from WebAssembly to the native "OS" function that writes to standard out, are all compatible with JavaScript data types.  However, some WASI modules might contain function calls whose interfaces are not compatible, and therefore, such modules cannot immediately be called.

## 64-bit Integers

The real issue here centres on transferring 64-bit integers between the two runtime environments.

Both JavaScript and WebAssembly use this data type (known as `i64` in WebAssembly and `BigInt` in JavaScript); but for a variety of reasons, the transfer of this data type has not yet been implemented and is still at the proposal stage.  (See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details).

> ### IMPORTANT
>
> Irrespective of whether your JavaScript app runs on the client or the server, the interface to any WASI module call that has been declared to use an `i64` must first be ***transformed*** before it can be called.

Remember, in the context of a JavaScript program, the WASI bridge between WebAssembly and native "OS" functions has been implemented using a set of JavaScript polyfills.  Consequently, you will experience this problem if you try for example to invoke a WebAssembly module that then invokes a native "OS" function such as [clock\_time\_get](https://github.com/AssemblyScript/assemblyscript/blob/899e97ab28a857da07a533551eb937d771677f93/std/assembly/bindings/wasi_unstable.ts#L36).

As a temporary fix, this data transfer issue is solved by the `@wasmer/wasm-transformer` package.

> ### Under The Hood  
> Technically, this transformation adapts the WebAssembly interface so that it can send and receive JavaScript `BigInt`s (64-bit, signed integers) without data loss.
>
> This is acheived by transforming a JavaScript `BigInt` into a `Uint8Array` containing 8, unsigned, 8-bit integers.

# How Do I Know if a WASM Module Needs Transformation?

Good question!

Normally, you would discover what data types a native "OS" function interface uses by looking at the well-written interface documentation for the WebAssembly module.

Ok, back in reality...

## The [`clock_time_get`](https://raw.githubusercontent.com/wasmerio/docs.wasmer.io/master/docs/wasmer-js/wasm_lib/clock_time_get.wat) WebAssembly Module

In order to understand whether or not this module needs transformation, we need to take a look at the WebAssembly module.

In the Transforming WASI Modules examples, there is an example [AssemblyScript](https://github.com/AssemblyScript/assemblyscript) module, in which is uses the [clock_time_get WASI binding from the wasi_unstable phase](https://github.com/AssemblyScript/assemblyscript/blob/899e97ab28a857da07a533551eb937d771677f93/std/assembly/bindings/wasi_unstable.ts#L36). For example:

```typescript
import {
  proc_exit,
  fd_write,
  clock_time_get
} from "bindings/wasi_unstable";
```

And then, in the `_start` function, we call the binding:

```typescript
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
```

When we compile the AssemblyScript to a binary WebAssembly module, and to the [WebAssembly Text](https://webassembly.github.io/spec/core/text/index.html) format, the first few lines of the `.wat` would be similar to something like this:

```WebAssemblyText
(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t1)))

  ;; snip...
```

We can see the declaration of a type definition called `$t0`.  This type definition represents the interface to some `func`tion that takes three, signed integers as parameters and returns a signed integer.

```WebAssemblyText
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter.  Uh oh! Its an `i64`; that is, a 64-bit, signed integer!

So now we know that somewhere in this WebAssembly module, there is a call to function that uses this interface declaration.

Next, look a little further down, and here we can see an `import` statement.

```WebAssemblyText
(import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
```

This `import` statement tells us several things:

1. This WASM module needs to call an external function.  
    In this particular case, this is a native "OS" function accessible through WASI
1. The native "OS" function is called `clock_time_get` and lives in an external library called `wasi_unstable`
1. Within our WebAssembly module, this external function will be referred to using the alias `$wasi_unstable.clock_time_get`
1. The interface to this function is described by the type declaration `$t0`

We know from the definition of `$t0` (on line 2) that this function must be passed an `i64` as its second parameter; therefore, we can be certain that before this WASM module can call function `clock_time_get` (using the Wasmer-js polyfill), the interface must first be transformed.
