---
id: runtime-cli-getting-started
title: Runtime Command Line Interface Getting Started
sidebar_label: Getting Started
---

# Prerequisites

If you haven't already done so, install the Wasmer CLI using the instructions on the [installation page](./runtime-cli-installation).

# Test a WASI Module

There are two ways to test a WASI module:

1. Using your web browser at [WebAssembly.sh](https://webassembly.sh/), or

1. From the command line using the Wasmer CLI

Here, we assume you have already installed the Wasmer CLI and that you can run WASM modules from the command line!

To do this, you want to find a WASM Module compiled down to an ABI that the Wasmer runtime supports, such as WASI or Emscripten.

The easiest place to locate such a module is from the [WebAssembly Package Manager](https://wapm.io) website.

![wapm.io WebAssembly Package Manager](/img/wapm/wapm_screenshot_1.png)

Here, we're searching for the WebAssembly compiled JavaScript interpreter called `quickjs`.  Select the module name from the drop-down list underneath the search field to display the module's details.

![wapm.io WebAssembly Package Manager](/img/wapm/wapm_screenshot_2.png)

Select the "EXPLORE" tab.

![wapm.io WebAssembly Package Manager](/img/wapm/wapm_screenshot_3.png)

Now you can download the WASM module and run it locally.

Assuming your browser saves the WASM module in a directory such as `Downloads`, you can run it using the following commands (suitably adapted for your OS)...

```bash
$ cd ~/Downloads/
✔ ~/Downloads 
$ wasmer qjs.wasm 
QuickJS - Type "\h" for help
qjs > console.log("WASM is cool!")
console.log("WASM is cool!")
WASM is cool!
undefined
qjs > ^C
✘-INT ~/Downloads 
$ 
```

Alternatively, you could run this module directly in the browser by selecting the "WEBASSEMBLY SHELL" tab.

![wapm.io WebAssembly Package Manager](/img/wapm/wapm_screenshot_4.png)

> ***IMPORTANT***
>
> If you visit [wapm.io](https://wapm.io) using the [Brave browser](https://brave.com/), you must put the shields down before the embedded WebAssembly Shell will work!
>
> ![Brave shields down](/img/wapm/brave_shields_down.png)



Next, we can take a look at the command line flags and arguments for the CLI, for more advanced usage.
