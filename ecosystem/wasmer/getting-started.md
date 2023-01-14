---
id: runtime-cli-getting-started
title: Runtime Command Line Interface Getting Started
sidebar_label: Getting Started
---

# Getting Started

You can install the Wasmer **Standalone** runtime by following the instructions below:

If you haven't done so already, install the Wasmer Command Line Interface (CLI) for macOS/Linux:

```
curl https://get.wasmer.io -sSfL | sh
```

Or you can use this PowerShell command if you are in Windows:

```
iwr https://win.wasmer.io -useb | iex
```

{% hint style="info" %}
Note: All Wasmer executables can be found on our [Github Releases Page](https://github.com/wasmerio/wasmer/releases).

Alternative installation methods can be found in the [wasmer-install repository](https://github.com/wasmerio/wasmer-install).
{% endhint %}

Once the Wasmer CLI is installed, you can run Wasm modules and WAPM packages directly from the command line!

In this example, we will be using Python compiled to WebAssembly. To do this we are going to use the `python/python` package from wapm:

```
wasmer run python/python
```

Which should bring up the Python prompt which you can then interact with. See an example below:

<figure><img src="../../.gitbook/assets/Screenshot 2022-11-21 at 5.42.17 PM.png" alt=""><figcaption><p>wasmer CLI running the WAPM Python package</p></figcaption></figure>



Wasmer can also run standalone WASI or Emscripten Wasm apps, simply point `wasmer run` to the .wasm file!

```
wasmer run my-emscripten-or-wasi-file.wasm
```

Next, we can take a look at the command line flags and arguments for the CLI, for more advanced usage.

{% hint style="warning" %}
Wasmer needs some requirements to be met to run properly. On Linux, you will have to be sure you have a compatible `libc`/`libstdc++`. Wasmer expects the following ABI versions to be available:

* GLIBC: `>= 2.27`
* GLIBCXX: `>= 3.4.21`

To verify if your environments provides the correct versions you can use the commands below:

```bash
LIBC_PATH=$(gcc --print-file-name=libc.so.6)
strings $LIBC_PATH | grep -e '^GLIBC'

LIBCXX_PATH=$(gcc --print-file-name=libstdc++.so.6)
strings $LIBCXX_PATH | grep -e '^GLIBCXX'
```

Note that you will need to install `gcc` and `binutils` for these commands to work.

If you can't match these requirements you will have to [build Wasmer](building-from-source/) manually.
{% endhint %}
