---
id: runtime-cli-getting-started
title: Runtime Command Line Interface Getting Started
sidebar_label: Getting Started
---

If you haven't already, Install the Wasmer CLI.

    curl https://get.wasmer.io -sSfL | sh

Once you have the Wasmer CLI installed, you can run wasm modules from the command line!

To do this, you want to find a Wasm Module compiled down to an ABI that the Wasmer runtime supports, such as WASI or Emscripten. For instance, we can search for a module on WAPM, and go to the module page, and then click on the "Browse modules" tab. 

In this example, we will be using the [WASI Compiled QuickJS](https://wapm.io/package/quickjs). To do this we would download the module, and then run:

`wasmer run quickjs.wasm`

Which should bring up the QuickJS prompt which you can then interact with.

Next, we can take a look at the command line flags and arguments for the CLI, for more advanced usage.
