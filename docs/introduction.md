---
id: introduction
title: Introduction
sidebar_label: Introduction
---

![Wasmer Logo](/img/wasmer-logo.svg)

Hello! Welcome to the Wasmer Documentation!

[Wasmer](https://wasmer.io/) is an open-source runtime for executing WebAssembly on the Server. Our mission is make all software universally available. 

For an overview of WebAssembly, and what WebAssembly is, [take a look here](https://webassembly.org/).

## Background

By design, the environment within which a WebAssembly module runs is completely isolated (or "sand-boxed") from the native functionality of the underlying operating system.  This means that normally speaking, a WASM module has no access to OS-level resources such as file descriptors, network sockets, the system clock or random numbers.

However, there are many legitimate cases in which a WebAssembly module needs to interact with native "OS" functionality.  Wasmer therefore acts as the bridge allowing WASM modules to be run either:

1. Standalone via our runtime, or
1. By means of our langauge integration tools, embedded within language runtimes such as C/C++, Python and Rust

> ## IMPORTANT  
> The term "OS" used above is in double quotes to indicate that the native function being called might not actually belong to the underlying operating system.  In reality, native functions always belong to the host environment within which the WebAssembly module is being run, and that could be either the host langauge's runtime environment (for example, JavaScript, Python or Ruby), or it might be the actual operating system.
>
> Either way though, from a WebAssembly point of view, we don't need to care too much about this detail. All we need to know is that:
> 
> * Via Wasmer (and [`WASI`](https://wasi.dev)), WebAssembly can have access to a set of operating-system-like functions, and
> * as long as we're careful, we can call these functions


## Projects

We also have other projects such as:

1. The [WAPM (WebAssembly Package Manager)](https://wapm.io/)
1. [Wasmer-js](https://github.com/wasmerio/wasmer-js)
1. The [WebAssembly Shell](https://webassembly.sh/)

to name but a few...

## Tutorials

If you would like to see tutorials, examples, or reference API documentation about a specific Wasmer project, please use the sidebar to the left, or the search bar at the top of this page in the header, or take a look at the list below:

* [Wasmer Runtime Documentation](/runtime/runtime)
* [Wasmer JS Documentation](/wasmer-js/wasmer-js)
* [WAPM Documentation](/wapm/wapm)
