# Setup your Rust environment

To be able to run Wasmer inside our Rust application, we will need Rust installed in our system.

The easiest way to get [Rust](https://www.rust-lang.org/) in your system is via [Rustup](https://rustup.rs/).  
To get Rustup on Linux and macOS, you can run the following:

```bash
curl https://sh.rustup.rs -sSf | sh
```

{% hint style="info" %}
To install Rust on Windows, download and run [rustup-init.exe](https://win.rustup.rs/), then follow the onscreen instructions.
{% endhint %}

To ensure this is installed, let's run the following:

```bash
rustc -V # This will show Rust version
cargo -V # This will show Cargo version
```

{% hint style="success" %}
If these commands work, Rust is successfully installed!
{% endhint %}

## Start a Rust Project

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

{% page-ref page="../examples/" %}



