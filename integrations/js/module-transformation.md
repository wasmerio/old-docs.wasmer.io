---
id: wasmer-js-module-transformation
title: Transforming WASI Modules
sidebar_label: Data Transfer Between JavaScript & WASM
---

# Module Transformation

## What is Module Transformation and Why is it Necessary?

In the Browser-based [Hello World](https://github.com/wasmerio/docs.wasmer.io/tree/ca2c9145ea511f3c00439b180be82cc5197a177f/docs/wasmer-js/client/examples/hello-world/wasmer-js-client-hello-world/README.md) example, we call a WASM module called `as-echo` that does nothing more than receive a text string as an argument, and echo it back via standard out.

In this case, the values passed from WebAssembly to the native "OS" function that writes to standard out, are all compatible with JavaScript data types. However, some WASI modules might contain function calls whose interfaces are not compatible, and therefore, such modules cannot immediately be called.

### 64-bit Integers

The real issue here centers on transferring 64-bit integers between the two runtime environments.

Both JavaScript and WebAssembly use this data type \(known as `i64` in WebAssembly and `BigInt` in JavaScript\); but for a variety of reasons, the transfer of this data type has not yet been implemented and is still at the proposal stage. \(See [here](https://github.com/WebAssembly/JS-BigInt-integration/issues/15) and [here](https://github.com/WebAssembly/proposals/issues/7) for details\).

{% hint style="warning" %}
### Important

Irrespective of whether your JavaScript app runs on the client or the server, the interface to any WASI module call that has been declared to use an `i64` must first be _**transformed**_ before it can be called.
{% endhint %}

Remember, in the context of a JavaScript program, the WASI bridge between WebAssembly and native "OS" functions has been implemented using a set of JavaScript polyfills. Consequently, you will experience this problem if you try for example to invoke a WebAssembly module that then invokes a native "OS" function such as [clock\_time\_get](https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---errno-timestamp).

As a temporary fix, this data transfer issue is solved by the `@wasmer/wasm-transformer` package.

{% hint style="success" %}
### Under the hood

Technically, this transformation adapts the WebAssembly interface so that it can send and receive JavaScript `BigInt`s \(64-bit, signed integers\) without data loss.

This is achieved by transforming a JavaScript `BigInt` into a `Uint8Array` containing 8, unsigned, 8-bit integers.
{% endhint %}

## How Do I Know if a WASM Module Needs Transformation?

Good question!

Normally, you would discover what data types a native "OS" function interface uses by looking at the well-written interface documentation for the WebAssembly module.

Ok, back in reality...

### The [`clock_time_get`](https://raw.githubusercontent.com/wasmerio/docs.wasmer.io/master/docs/wasmer-js/wasm_lib/clock_time_get.wat) WebAssembly Module

In order to understand whether or not this module needs transformation, we need to take a look inside the WebAssembly module.

{% hint style="info" %}
### Aside

We make no attempt to teach you WebAssembly here!

If you want to know about the inner workings of a WebAssembly module, then please visit the [WebAssembly.org](https://webassembly.org/) website and read the documentation there.

We now continue with your scheduled program...
{% endhint %}

When converted to [WebAssembly Text](https://webassembly.github.io/spec/core/text/index.html) format, the first few lines look like this:

```scheme
(module
  (type $t0 (func (param i32 i64 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32 i32) (result i32)))
  (type $t2 (func))
  (import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
  (import "wasi_unstable" "fd_write" (func $wasi_unstable.fd_write (type $t1)))

  ;; snip...
)
```

On line 2, we can see the declaration of a type definition called `$t0`. This type definition represents the interface to some `func`tion that takes three, signed integers as parameters and returns a signed integer.

```scheme
(type $t0 (func (param i32 i64 i32) (result i32)))
```

Notice the data type of the second parameter. Uh oh! Its an `i64`; that is, a 64-bit, signed integer!

So now we know that somewhere in this WebAssembly module, there is a call to function that uses this interface declaration.

Next, look a little further down to line 5. Here we can see an `import` statement.

```scheme
(import "wasi_unstable" "clock_time_get" (func $wasi_unstable.clock_time_get (type $t0)))
```

This `import` statement tells us several things:

1. This WASM module needs to call an external function.  

    In this particular case, this is a native "OS" function accessible through WASI

2. The native "OS" function is called `clock_time_get` and lives in an external library called `wasi_unstable`
3. Within our WebAssembly module, this external function will be referred to using the alias `$wasi_unstable.clock_time_get`
4. The interface to this function is described by the type declaration `$t0`

We know from the definition of `$t0` \(on line 2\) that this function must be passed an `i64` as its second parameter; therefore, we can be certain that before this WASM module can call function `clock_time_get` \(using the Wasmer-js polyfill\), the interface must first be transformed.
