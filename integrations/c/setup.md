# Setup your C/C++ environment

To build C applications that use the Wasmer runtime as a host for guest Wasm modules, you will need a C compiler installed \(`clang` or `gcc`, for example\). Check the compiler installation instructions [here](setup.md#installing-clang-gcc).

Starting with Wasmer 1.0 **all the releases ship with the required libraries and headers** to use Wasmer from C and C++.

If you haven't yet, let's install Wasmer: 

```text
curl https://get.wasmer.io -sSfL | sh
```

{% hint style="info" %}
You can follow the [detailed Wasmer installation instructions here](../../ecosystem/wasmer/getting-started.md).
{% endhint %}

Once Wasmer is installed, you can get the `pkg-config` easily:

```bash
$ wasmer config --pkg-config
prefix=/Users/USER/.wasmer
exec_prefix=/Users/USER/.wasmer/bin
includedir=/Users/USER/.wasmer/include
libdir=/Users/syrus/.wasmer/lib

Name: wasmer
Description: The Wasmer library for running WebAssembly
Version: 1.0.0-beta1
Cflags: -I/Users/USER/.wasmer/include/wasmer
Libs: -L/Users/syrus/.wasmer/lib -lwasmer
```

## Manual Installation

Download our SDK for the C-API from [Wasmer releases page](https://github.com/wasmerio/wasmer/releases).  
Depending on your system, you will need to download:

* Linux: `wasmer-linux-amd64.tar.gz` or `wasmer-linux-aarch64.tar.gz` 
* macOS: `wasmer-darwin-amd64.tar.gz` or `wasmer-darwin-arm64.tar.gz`
* Windows: `wasmer-windows.tar.gz` 

Once you have downloaded the c-api file, you can now extract its contents and set the `WASMER_DIR` environment variable to the path of the wasmer directory \(this will be very useful when running the examples\):

```bash
# Extract the contents to a dir
mkdir wasmer
tar -C wasmer -zxvf wasmer-*.tar.gz

export WASMER_DIR=$(pwd)/wasmer

# Update LD_LIBRARY_PATH to link against the libwasmer.so in the examples
export LD_LIBRARY_PATH=$WASMER_DIR/lib/:$LD_LIBRARY_PATH
```

{% hint style="info" %}
Note: You can also [build the C-API from source](../../ecosystem/wasmer/building-from-source/#building-wasmer-c-api-from-source).
{% endhint %}

## Installing Clang/GCC

First, you can test if `clang` or `gcc` is installed already by running:

```text
gcc --version
```

If this command does not return the version, then see the following:

### MacOS

`gcc` / `clang` is usually installed by default. However, if it is not, you can [install gcc/clang by installing xcode-select](http://osxdaily.com/2014/02/12/install-command-line-tools-mac-os-x/) on your mac.

### Debian / Debian Based / Linux

To install gcc on a Debian based, you will want to run the following:

```text
sudo apt-get install build-essential
```

Gcc should be installable on your favorite linux distro as well. Please search for the correct command for your desired distribution.

### Windows

To install gcc on Windows, you will probably want to install a GNU environment on windows. We suggest using [MinGW](http://www.mingw.org/).

```bash
gcc --version

# This should output: "ATTRIBUTIONS LICENSE      bin          include      lib"
ls $WASMER_DIR
```

{% hint style="success" %}
If these commands work, The compiler and the Wasmer C API are successfully installed!
{% endhint %}

Next, let's take a look at some examples!

