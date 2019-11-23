---
id: runtime-c-integration-installation
title: Runtime C/C++ Integration: Installation
sidebar_label: Installation
---

To build C applications that use the Wasmer runtime as a host for guest wasm modules, you will need Rust installed. We suggest Cmake, and make for building your host application. Rust is needed to build the wasmer runtime, Cmake and make will be used to build the host application itself.

# Installing Rust

First, let's install Rust. The easiest way to install rust, is through [rustup](https://rustup.rs/).

To build install Rustup on Windows, download and run `[rustup-init.exe](https://win.rustup.rs/)` then follow the onscreen instructions.

To get rustup on other systems, run:

`curl [https://sh.rustup.rs](https://sh.rustup.rs/) -sSf | sh`

To ensure this is installed, let's run the following:

`rustc -v`

and

`cargo -v`

If these commands work, rust is successfully installed! Please note, You must have a rust version ≥ 1.38 to use the Wasmer C API.

# Installing CMake

First, you can test if make is installed already by running:

`cmake --help`

If this command does not return the make version, then see the following:

Please take a look at the CMake documentation for [installing CMake on Windows, Mac, and Unix](https://cmake.org/install/). CMake may already be installed on your machine by default.

You can test that cmake is installled by running: `cmake --help`. Which should output the help output for cmake!

# Installing Make

First, you can test if make is installed already by running:

`make -v`

If this command does not return the make version, then see the following:

## MacOS

Make is usually installed by default. However, if it is not, you can [install make by installing xcode-select](http://osxdaily.com/2014/02/12/install-command-line-tools-mac-os-x/) on your mac.

## Debian / Debian Based / Linux

To install make on a debian based, you will want to run the following:

`sudo apt-get install build-essential`

Make should be installable on your favorite linux distro as well. Please search for the correct command for your desired distribution.

## Windows

To install Make on Windows, you will probably want to install a GNU environment on windows. We suggest using [MinGW](http://www.mingw.org/).

Now that all of our dependencies are installed, let's take a look at the hello world example!

- Make a directory called build for Cmake
- cd into build
- cmake ..
- Must be rust 1.38 (rustc —version)
- make -j
- Done! Run `./hello-world`.
