---
id: runtime-cli-building-from-source
title: Runtime Command Line Building from Source
sidebar_label: Building from Source
---

# Prerequisites: Installing Rustup

In order to build Wasmer from source, you must first install [rustup](https://rustup.rs/).

## Windows

To install Rustup on Windows, download and run [rustup-init.exe](https://win.rustup.rs/), then follow the onscreen instructions.

## *NIX Operating Systems

To install rustup on *NIX systems, run:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

# Install Additional Dependencies

## MacOS

If you have [Homebrew](https://brew.sh/) installed:

```bash
$ brew install cmake
```

Or, if you have [MacPorts](https://www.macports.org/install.php):

```bash
$ sudo port install cmake
```

## Debian Based Linuxes

```bash
$ sudo apt install cmake pkg-config libssl-dev
```

## FreeBSD

```bash
$ pkg install cmake
```

## Windows

Windows support is ***experimental!***

WASI is fully supported, but Emscripten support is in the works (this means `nginx` and Lua do not work on Windows - you can track the progress on [this issue](https://github.com/wasmerio/wasmer/issues/176)).

1. Install [Visual Studio](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)
2. Install [Rust for Windows](https://win.rustup.rs/)
3. Install [Git for Windows](https://git-scm.com/download/win). Allow it to add `git.exe` to your PATH (default settings for the installer are fine).
4. Install [CMake](https://cmake.org/download/). Ensure `cmake` is in your PATH!
5. Install [LLVM 8.0](https://prereleases.llvm.org/win-snapshots/LLVM-8.0.0-r351033-win64.exe)

# Building the Wasmer Runtime

Wasmer is built using the Rust's package manager called [Cargo](https://crates.io/), .

The Singlepass backend requires nightly, so if you want to use it, you must make the Rust nightly build the default using the command:

```bash
$ rustup default nightly
```

Otherwise an up to date (see badge above) version of stable Rust will work.

If you want support for the Wasmer LLVM backend, then you will also need to check the following: 

1. Ensure that LLVM version 8.0.x or higher is installed on your system.
    
    A prebuilt binary can be downloaded from the [LLVM website](https://releases.llvm.org/download.html)

1. Set the correct environment variable for LLVM access.

    For example, for LLVM 8.0.x, the environment variable would be: `LLVM_SYS_80_PREFIX=/path/to/unpacked/llvm-8.0` 

And install Wasmer

```bash
# checkout code
$ git clone https://github.com/wasmerio/wasmer.git
$ cd wasmer
# install tools
$ make release-clif          # To build with cranelift (default)
$ make release-llvm          # To build with llvm support
$ make release-singlepass    # To build with singlepass support
# or
$ make release               # To build with singlepass, cranelift and llvm support
```
