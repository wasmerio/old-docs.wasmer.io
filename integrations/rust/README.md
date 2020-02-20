# Rust

Wasmer lets you use WebAssembly modules in your Rust projects.

{% hint style="info" %}
### Did you know ...?

Some of our language extensions are using the Wasmer Rust crate under the hood.

* [Wasmer Python Extension](https://github.com/wasmerio/python-ext-wasm)
* [Wasmer Ruby Extension](https://github.com/wasmerio/ruby-ext-wasm)
* [Wasmex Elixir Extension](https://github.com/tessi/wasmex)
{% endhint %}

In this section we will go through the instructions on how to setup your Rust environment, to then visit different examples of how to use Wasmer in Rust.

{% page-ref page="setup.md" %}

{% page-ref page="examples/" %}

{% embed url="https://docs.rs/wasmer-runtime/0.13.1/wasmer\_runtime/" caption="" %}

## Published Crates

Apart from this, Wasmer publishes also different Packages to Crates:

* [wasmer-runtime](https://crates.io/crates/wasmer-runtime/): The Wasmer Runtime 
* Backends:
  * [wasmer-singlepass-backend](https://crates.io/crates/wasmer-singlepass-backend): The Singlepass backend \(fast compilation, normal runtime\)
  * [wasmer-clif-backend](https://crates.io/crates/wasmer-clif-backend): The Cranelift backend \(normal compilation, a bit faster runtime\)
  * wasmer-llvm-backend: The LLVM backend \(slow compilation, super fast runtime\)
* Integrations:
  * [wasmer-wasi](https://crates.io/crates/wasmer-wasi): The integration of WASI into Wasmer

Let's now setup your Rust environment!

