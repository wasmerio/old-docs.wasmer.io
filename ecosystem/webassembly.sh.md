# WebAssembly.sh

The WebAssembly shell is an online shell where you can drag-and-drop your WebAssembly modules to try them out, and also execute [all the commands of WASI modules available in WAPM](https://wapm.io/interface/wasi).

![](../.gitbook/assets/image%20%281%29.png)

{% embed url="https://webassembly.sh" caption="" %}

[WebAssembly.sh](https://webassembly.sh/) it's powered by Wasmer-JS.

It currently supports running modules with the following Application Binary Interfaces \(ABIs\):

* WASI

## Features

The WebAssembly shell has a lot of nice features!

* The execution is entirely done in your **Browser**, no server-side interaction
* All WASI commands published to **WAPM** will be automatically available
* It works **offline**!
* You can **drag and drop** any WebAssembly WASI file to execute on your browser
* It has a **filesystem**! You can drag and drop any files into it and they will be added into the `/tmp` folder

## Prebuilt commands

You can execute the following pre-built commands on the WebAssembly shell:

### `help`

Will show you a help command

### `wapm`

Will let you interact with all modules in WAPM, installing them and uninstalling automatically

### `about`

It will show you the about page!

### `<program_name>`

It will search the command in WAPM, and if any WASI module is published with that command, it will automatically attach it to the shell! 🎉

