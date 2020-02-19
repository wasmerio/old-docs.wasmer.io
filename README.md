# Introduction

Welcome to the Wasmer Documentation! 👋

[Wasmer](https://wasmer.io/) is an open-source runtime for executing WebAssembly on the Server.

> ![](.gitbook/assets/image%20%282%29.png)
>
> Wasmer mission is make all software universally available

For an overview of WebAssembly, and what WebAssembly is, [take a look here](https://webassembly.org/).

## Background

By design, the environment within which a WebAssembly module runs is completely isolated \(or _sandboxed_\) from the native functionality of the underlying host system. This means that _by default_, WASM modules are designed to perform nothing more than pure computation.

Consequently, access to OS-level resources such as file descriptors, network sockets, the system clock, and random numbers is not normally possible from WASM.

However, there are many cases in which a WebAssembly module needs to do more than perform pure computation; they must interact with native "OS" functionality.

Wasmer allows you to run Wasm modules either **Standalone** or **Embedded** within other language runtimes such as C/C++, Python, Go, PHP, Elixir, Rust...

This provide three key services:

1. Enables extremely portable binaries that can run unmodified in any OS that is supported by Wasmer \(Linux, macOS, Windows and FreeBSD\).
2. It acts as a secure bridge for WASM modules to interact with native "OS" functionality, via ABIs such as [`WASI`](https://github.com/webassembly/wasi) and [`Emscripten`](https://github.com/emscripten-core/emscripten)

{% hint style="warning" %}
### Important

The term "OS" used above is in quotes to indicate that the native function being called might not actually be provided by the host's operating system.

In reality, native functions always belong to the host environment within which the WebAssembly module is being run, and that could be either the host language's runtime environment \(for example, JavaScript, Python or Ruby\), or it might be the actual operating system.

Either way though, from a WebAssembly point of view, we don't need to care too much about this detail. All we need to know is that:

* The host can provide "imported" functions for the WebAssembly module
* Via Wasmer's included ABIs, WebAssembly modules can have access to a set of operating-system-like functions with varying levels of sandboxing
{% endhint %}

## Projects

We also have other projects such as:

1. The [WAPM \(WebAssembly Package Manager\)](https://wapm.io/)\]
2. The [WebAssembly Shell](https://webassembly.sh/)

to name but a few...

Also, for the latest blogs on Wasmer features and developments, check out our [Medium site](https://medium.com/wasmer).

## Tutorials

If you would like to see tutorials, examples, or reference API documentation about a specific Wasmer project, please use the sidebar to the left, or the search bar at the top of this page in the header, or take a look at the list below:

* [Wasmer Runtime](ecosystem/wasmer/)
* [WAPM](ecosystem/wapm/)

Language Integrations \(to use WebAssembly there\):

* [Wasmer in Rust](https://github.com/wasmerio/docs.wasmer.io/tree/5c49b94d489d174570aef618013adfbc144893a5/integrations/rust/README.md)
* [Wasmer in C](https://github.com/wasmerio/docs.wasmer.io/tree/5c49b94d489d174570aef618013adfbc144893a5/integrations/c/README.md)
* [Wasmer in Javascript](https://github.com/wasmerio/docs.wasmer.io/tree/5c49b94d489d174570aef618013adfbc144893a5/integrations/javascript/README.md)

