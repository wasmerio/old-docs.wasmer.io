# Setup

To build C applications that use the Wasmer runtime as a host for guest wasm modules, you will need a C compiler installed \(clang or gcc, for example\).

Once you have it installed, please download our SDK for the C-API from [Wasmer releases page](https://github.com/wasmerio/wasmer/releases). Depending on your system, you will need to download:

* Linux: `wasmer-c-api-linux-amd64.tar.gz` 
* macOS: `wasmer-c-api-darwin-amd64.tar.gz` 
* Linux: `wasmer-c-api-windows.tar.gz` 

You can also [build the C-API from source](../../ecosystem/wasmer/building-from-source/#building-the-c-api-from-source).

{% hint style="info" %}
We suggest [Cmake](https://cmake.org/), and make for building your host application
{% endhint %}

## Installing CMake

First, you can test if make is installed already by running:

```text
cmake --help
```

If this command does not return the make version, then see the following:

Please take a look at the CMake documentation for [installing CMake on Windows, Mac, and Unix](https://cmake.org/install/). CMake may already be installed on your machine by default.

## Installing Make

First, you can test if make is installed already by running:

```text
make -v
```

If this command does not return the make version, then see the following:

### MacOS

Make is usually installed by default. However, if it is not, you can [install make by installing xcode-select](http://osxdaily.com/2014/02/12/install-command-line-tools-mac-os-x/) on your mac.

### Debian / Debian Based / Linux

To install make on a debian based, you will want to run the following:

```text
sudo apt-get install build-essential
```

Make should be installable on your favorite linux distro as well. Please search for the correct command for your desired distribution.

### Windows

To install Make on Windows, you will probably want to install a GNU environment on windows. We suggest using [MinGW](http://www.mingw.org/).

Now that all of our dependencies are installed, let's take a look at the hello world example!

* Make a directory called build for Cmake
* cd into build
* cmake ..
* Must be rust 1.38 \(rustc —version\)
* make -j
* Done! Run `./hello-world`.
