---
id: runtime-cli-building-from-source
title: Runtime Command Line Building from Source
sidebar_label: Building from Source
---

# Building from Source

## Installing Rustup

Building Wasmer requires [Rust](https://rustup.rs/) for building it from source.

The easiest way to get Rust in your system is via Rustup.  
To get rustup on Linux and macOS, you can run the following:

```bash
curl https://sh.rustup.rs -sSf | sh
```

{% hint style="info" %}
To install Rust on Windows, download and run [rustup-init.exe](https://win.rustup.rs/), then follow the onscreen instructions.
{% endhint %}

## Installing Additional Dependencies

### MacOS

If you have [Homebrew](https://brew.sh/) installed:

```text
brew install cmake
```

Or, if you have [MacPorts](https://www.macports.org/install.php):

```bash
sudo port install cmake
```

### Debian-based

```bash
sudo apt install cmake pkg-config libssl-dev
```

### FreeBSD

```text
pkg install cmake
```

### Windows

Windows support is _experimental_. WASI is fully supported, but Emscripten support is in the works.

1. Install [Visual Studio](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)
2. Install [Rust for Windows](https://win.rustup.rs/)
3. Install [Git for Windows](https://git-scm.com/download/win). Allow it to add `git.exe` to your PATH \(default settings for the installer are fine\).
4. Install [CMake](https://cmake.org/download/). Ensure CMake is in your PATH.
5. Install [LLVM 8.0](https://prereleases.llvm.org/win-snapshots/LLVM-8.0.0-r351033-win64.exe)

## Building the Wasmer Runtime

Wasmer is built with [Cargo](https://crates.io/), the Rust package manager.

First, let's clone Wasmer:

```text
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
```

Wasmer has three different backends at the moment:

### Singlepass Backend

The Singlepass backend requires nightly, so if you want to use it, set Rust Nightly:

```bash
rustup default nightly
```

And then, build Wasmer with the singlepass backend:

```text
make release-singlepass
```

{% hint style="info" %}
The singlepass backend requires Rust nightly, as it's using the [Dynasm crate](https://github.com/CensoredUsername/dynasm-rs) which depends on Rust features only available in Rust nightly
{% endhint %}

### Cranelift Backend

The Cranelift backend will work with both nightly and stable versions of Rust.

```text
make release-clif
```

### LLVM Backend

If you want support for the Wasmer LLVM backend, then you will also need to ensure:

* Ensure that LLVM 8.0.x &gt; is installed on your system
  * You can also [download and use a prebuilt LLVM binary](https://releases.llvm.org/download.html)
* Set the correct environment variable for LLVM to access
  * For example, the environment variable for LLVM 8.0.x would be: `LLVM_SYS_80_PREFIX=/path/to/unpacked/llvm-8.0` 

And create a Wasmer release

```bash
make release-llvm
```

### All backends \(default\)

If you want to support all backends \(by default\), you can just run:

```bash
make release
```

{% hint style="info" %}
For compiling with all backends, you will need to set nightly as the default \(for Singlepass\) and you will also need LLVM installed in your system \(LLVM\)
{% endhint %}

## Running your Wasmer binary

Once you run a `release-*` \(or simply `release`\) command, you will have a new binary ready to be used!

```text
./target/release/wasmer quickjs.wasm
```

## Building Wasmer C-API from source

Wasmer provides a pre-compiled version for the C-API in it's [release page](https://github.com/wasmerio/wasmer/releases), but you can also compile it from source:

```text
make capi
```

