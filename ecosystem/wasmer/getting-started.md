---
id: runtime-cli-getting-started
title: Runtime Command Line Interface Getting Started
sidebar_label: Getting Started
---

# Getting Started

You can install the Wasmer **Standalone** runtime by following the instructions below:

If you haven't done so already, install the Wasmer Command Line Interface \(CLI\) for macOS/Linux:

```text
curl https://get.wasmer.io -sSfL | sh
```

Or you can use this PowerShell command if you are in Windows:

```text
iwr https://win.wasmer.io -useb | iex
```

{% hint style="info" %}
Note: All Wasmer executables can be found on out [Github Releases Page](https://github.com/wasmerio/wasmer/releases).
{% endhint %}

{% hint style="warning" %}
Wasmer needs some requirements to be met to run properly. On Linux, you will have to be sure you have a compatible 
`libc`/`libstdc++`. Wasmer expects the following ABI versions to be available:

* GLIBC: `>= 3.4.21`
* GLIBCXX: `>= 2.27`

To verify if your environments provides the correct versions you can use the commands below:

```bash
LIBC_PATH=$(gcc --print-file-name=libc.so.6)
strings $LIBC_PATH | grep -e '^GLIBC'

LIBCXX_PATH=$(gcc --print-file-name=libstdc++.so.6)
strings $LIBCXX_PATH | grep -e '^GLIBCXX'
```

Note that you will need to install `gcc` and `binutils` for these commands to work.

If you can't match these requirements you will have to [build Wasmer](./building-from-source/README.md) manually.
{% endhint %}

Once the Wasmer CLI is installed, you can run Wasm modules from the command line!

To do this, you want to find a Wasm Module compiled down to an ABI that the Wasmer runtime supports, such as WASI or Emscripten. For instance, we can search for a module on WAPM, and go to the module page, and then click on the "Browse modules" tab.

In this example, we will be using [QuickJS](https://wapm.io/package/quickjs) compiled to WebAssembly. To do this we [download the module from WAPM](https://wapm.io/package/quickjs#explore), and then run:

```text
wasmer qjs.wasm
```

Which should bring up the QuickJS prompt which you can then interact with. See an example below:

![](../../.gitbook/assets/screen-shot-2020-02-17-at-3.54.10-pm.png)

Next, we can take a look at the command line flags and arguments for the CLI, for more advanced usage.

