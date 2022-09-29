---
id: wasmer-js-module-transformation
title: Transforming WASI Modules
sidebar_label: Data Transfer Between JavaScript & Wasm
---

# Module Transformation and 64-bit Integers

For a variety or reasons, the specification for passing 64-bit integers between WebAssembly and its host environment is still at the proposal stage.
\(See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details\).

In spite of this, different vendors have gone ahead and implemented their own partial solutions.
For instance, programs running in NodeJS are able (with certain limitations) to work with `BigInt` values in the WebAssembly interface; however, that same WebAssembly program will probably fail if instantiated by a browser with an error such as `TypeError: Cannot convert a BigInt value to a number`

Since the standard has not yet been finalised, it is safest to assume that you will always fall into this implementation gap anytime a WebAssembly module needs to pass an `i64` value out to a host function (typically implemented through WASI).

A good example here is the WASI function [clock\_time\_get](https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---resulttimestamp-errno).

## What is Module Transformation?

"Module Transformation" is the interim solution in which all `i64` integers appearing in function interfaces are treated as if they are arrays of 8, unsigned bytes.
In JavaScript therefore, this means that each `i64` value has a `Uint8Array` laid over top of it, and the data then accessed as you would the elements of an array.

{% hint style="warning" %}
### Important

Irrespective of whether your JavaScript app runs on the client or the server, the interface to any WASI module call that has been declared to use an `i64` must first be _**transformed**_ before it can be called.

This is the purpose of the `@wasmer/wasm-transformer` package.
{% endhint %}

Remember, in the context of a JavaScript program, the WASI bridge between WebAssembly and native "OS" functions has been implemented using a set of JavaScript polyfills.

## How Do I Know if a Wasm Module Needs Transformation?

One way is to call the WebAssembly and see if it crashes...

However, a more rigorous approach is to examine the WebAssembly Text source code[^1] and check the type defintions of the imported/exported functions.
If you find an `i64` value then that module will need to be transformed.

### Calling the WASI Function [`clock_time_get`](https://raw.githubusercontent.com/wasmerio/docs.wasmer.io/master/docs/wasmer-js/wasm_lib/clock_time_get.wat)

In this example, we will look at a demo WebAssembly module called [`clocktimeget.wat`](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/wat/wasi/clocktimeget.wat) that obtains the raw system clock time by calling the WASI function [`clock_time_get`](https://raw.githubusercontent.com/wasmerio/docs.wasmer.io/master/docs/wasmer-js/wasm_lib/clock_time_get.wat).

{% hint style="info" %}
### Aside

We make no attempt to teach you WebAssembly Text here!

If you want to know about the inner workings of a WebAssembly module, then please visit the [WebAssembly.org](https://webassembly.org/) website and read the documentation there.

We now continue with your scheduled program...
{% endhint %}

The first few lines of the WebAssembly Text file look like this:

```wast
(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_fd_write (type $t1)))

  ;; snip...
)
```

On line 2, we can see the declaration of a type definition called `$t0`. This type definition represents the interface to some function that takes three, signed integers as parameters and returns a signed integer.

```wast
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter. Uh oh! Its an `i64`!

So now we know that somewhere in this WebAssembly module, there is a function that has this type of interface.

Next, look a little further down to line 5. Here we can see an `import` statement.

```wast
(import "wasi_unstable" "clock_time_get" (func $wasi_clock_time_get (type $t0)))
```

This `import` statement tells us several things:

1. This WebAssembly modules needs access to an external function provided by the host environment
1. This external function:
   1. is called `clock_time_get`
   1. lives in a library called `wasi_unstable`
   1. will be referred to using the alias `$wasi_clock_time_get`
   1. has an interface described by the type declaration `$t0`

So we have identified a function call that uses an `i64` value in its interface; therefore, this module must be transformed.

[^1]: If you only have the compiled WebAssembly module, then you can generate the WebAssembly Text using the `wasm2wat` program (which is part of the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt)): `wasm2wat <wasm_module> --generate-names -o <output_file_name>`
