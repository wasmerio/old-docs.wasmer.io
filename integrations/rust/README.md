# Rust

You can use Wasmer in your Rust projects to execute WebAssembly modules securely and conveniently.

{% hint style="info" %}
#### Did you know ...?

Some of our language extensions are using the Wasmer Rust crate under the hood.

* [Wasmer Python Extension](https://github.com/wasmerio/wasmer-python)
* [Wasmer Ruby Extension](https://github.com/wasmerio/wasmer-ruby)
* [Wasmex Elixir Extension](https://github.com/tessi/wasmex)
{% endhint %}

Check out the Wasmer Rust docs here:

{% embed url="https://docs.rs/wasmer/2.0.0/wasmer/" %}

## Setup your Rust Environment

To be able to run Wasmer inside our Rust application, we will need Rust installed in our system.

The easiest way to get [Rust](https://www.rust-lang.org/) in your system is via [Rustup](https://rustup.rs/).\
To get Rustup on Linux and macOS, you can run the following:

```bash
curl https://sh.rustup.rs -sSf | sh
```

{% hint style="info" %}
To install Rust on Windows, download and run [rustup-init.exe](https://win.rustup.rs/), then follow the onscreen instructions.
{% endhint %}

To ensure this is installed, let's run the following:

```bash
rustc -V # This will display the Rust version
cargo -V # This will display the Cargo version
```

{% hint style="success" %}
If these commands work, Rust is successfully installed!
{% endhint %}

## Published Rust Crates

Wasmer publishes various Crates:

* [`wasmer`](https://crates.io/crates/wasmer/): The Wasmer Runtime
* **Compilers**:
  * [`wasmer-compiler-singlepass`](https://crates.io/crates/wasmer-compiler-singlepass): The Singlepass compiler (fast compilation, normal runtime)
  * [`wasmer-compiler-cranelift`](https://crates.io/crates/wasmer-compiler-cranelift): The Cranelift compiler (normal compilation, a bit faster runtime)
  * [`wasmer-compiler-llvm`](https://crates.io/crates/wasmer-compiler-llvm): The LLVM compiler (slower compilation, super fast runtime)
* **Integrations**:
  * [`wasmer-wasi`](https://crates.io/crates/wasmer-wasi): Wasmer's implementation of the WASI standard. This allows you to run Wasm in a POSIX-like environment with a file system and permissions.
  * [`wasmer-emscripten`](https://crates.io/crates/wasmer-emscripten): Wasmer's implementation of the Emscripten ABI. This allows you to run Wasm in a less sandboxed way in a 32bit Linux-like environment.

Now let's setup your Rust environment!

## Start a Rust Project with Wasmer

Now it's time to create a new project and add Wasmer as a dependency:

```bash
cargo new wasmer-project --bin
```

This should generate two important files for us, `Cargo.toml` and `src/main.rs`. The `Cargo.toml` is a file that describes your project and its dependencies. The `src/main.rs` is the entry point for your project, and contains the `fn main() { .. }` that is run when the project is executed.

Now, edit the `Cargo.toml` file to add `wasmer` as a dependency:

```bash
[dependencies]
# The Wasmer API
wasmer = "3.0"
```

Next, let's take a look at some examples!

{% content-ref url="../examples/" %}
[examples](../examples/)
{% endcontent-ref %}
