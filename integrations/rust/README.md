# Rust

You can use Wasmer in your Rust projects to execute WebAssembly modules securely and conveniently.

{% hint style="info" %}
### Did you know ...?

Some of our language extensions are using the Wasmer Rust crate under the hood.

* [Wasmer Python Extension](https://github.com/wasmerio/wasmer-python)
* [Wasmer Ruby Extension](https://github.com/wasmerio/wasmer-ruby)
* [Wasmex Elixir Extension](https://github.com/tessi/wasmex)
{% endhint %}

In this section we will go through the instructions on how to setup your Rust environment, to then visit different examples of how to use Wasmer in Rust.

{% page-ref page="setup.md" %}

{% embed url="https://docs.rs/wasmer/1.0.0-alpha01.0/wasmer/" caption="" %}

## Published Crates

Wasmer publishes various Crates:

* [`wasmer`](https://crates.io/crates/wasmer/): The Wasmer Runtime
* **Compilers**:
  * [`wasmer-compiler-singlepass`](https://crates.io/crates/wasmer-compiler-singlepass): The Singlepass compiler \(fast compilation, normal runtime\)
  * [`wasmer-compiler-cranelift`](https://crates.io/crates/wasmer-compiler-cranelift): The Cranelift compiler \(normal compilation, a bit faster runtime\)
  * [`wasmer-compiler-llvm`](https://crates.io/crates/wasmer-compiler-llvm): The LLVM compiler \(slower compilation, super fast runtime\)
* **Engines**:
  * [`wasmer-engine-jit`](https://crates.io/crates/wasmer-engine-jit): The JIT Engine
  * [`wasmer-engine-native`](https://crates.io/crates/wasmer-engine-native): The Native Engine
* **Integrations**:
  * [`wasmer-wasi`](https://crates.io/crates/wasmer-wasi): Wasmer's implementation of the WASI standard. This allows you to run Wasm in a POSIX-like environment with a file system and permissions.
  * [`wasmer-emscripten`](https://crates.io/crates/wasmer-emscripten): Wasmer's implementation of the Emscripten ABI. This allows you to run Wasm in a less sandboxed way in a 32bit Linux-like environment.

Now let's setup your Rust environment!

