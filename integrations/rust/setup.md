# Setup

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
rustc -V # This will show the Rust version
cargo -V # This will work
```

{% hint style="success" %}
If these commands work, Rust is successfully installed!
{% endhint %}

Next, let's take a look at building a simple Hello World Example!

