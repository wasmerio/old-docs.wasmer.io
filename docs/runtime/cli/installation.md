---
id: runtime-cli-installation
title: Runtime Command Line Interface Installation
sidebar_label: Installation
---

## Recommended Installation

For Mac and Linux, Wasmer can be installed from an Automatic Install Script:

`curl [https://get.wasmer.io](https://get.wasmer.io/) -sSfL | sh`

For Windows, executable can be found on out [Github Releases Page](https://github.com/wasmerio/wasmer/releases).

## Building from Source

### Installing Rustup

Building Wasmer requires [rustup](https://rustup.rs/).

To build install Rustup on Windows, download and run `[rustup-init.exe](https://win.rustup.rs/)` then follow the onscreen instructions.

To get rustup on other systems, run:

`curl [https://sh.rustup.rs](https://sh.rustup.rs/) -sSf | sh`

### Installing Additional Dependencies

**MacOS**

If you have [Homebrew](https://brew.sh/) installed:

`brew install cmake`

Or, if you have [MacPorts](https://www.macports.org/install.php):

`sudo port install cmake`

**Debian Based Linuxes**

`sudo apt install cmake pkg-config libssl-dev`

**FreeBSD**

`pkg install cmake`

**Windows**

Windows support is *experimental*. WASI is fully supported, but Emscripten support is in the works (this means nginx and Lua do not work on Windows - you can track the progress on [this issue](https://github.com/wasmerio/wasmer/issues/176)).

1. Install [Visual Studio](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=15)
2. Install [Rust for Windows](https://win.rustup.rs/)
3. Install [Git for Windows](https://git-scm.com/download/win). Allow it to add `git.exe` to your PATH (default settings for the installer are fine).
4. Install [CMake](https://cmake.org/download/). Ensure CMake is in your PATH.
5. Install [LLVM 8.0](https://prereleases.llvm.org/win-snapshots/LLVM-8.0.0-r351033-win64.exe)

### Building the Wasmer Runtime

Wasmer is built with [Cargo](https://crates.io/), the Rust package manager.

The Singlepass backend requires nightly, so if you want to use it,

Set Rust Nightly:

`rustup default nightly`

Otherwise an up to date (see badge above) version of stable Rust will work.

If you want support for the Wasmer LLVM backend, then you will also need to ensure: 

* Ensure that LLVM 8.0.x > is installed on your system
    * You can also [download and use a prebuilt LLVM binary](https://releases.llvm.org/download.html)
* Set the correct environment variable for LLVM to access
    * For example, the environtment variable for LLVM 8.0.x would be: `LLVM_SYS_80_PREFIX=/path/to/unpacked/llvm-8.0` 

And install Wasmer

```bash
# checkout code
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
# install tools
make release-clif # To build with cranelift (default)
make release-llvm # To build with llvm support
make release-singlepass # To build with singlepass support
# or
make release # To build with singlepass, cranelift and llvm support
```
