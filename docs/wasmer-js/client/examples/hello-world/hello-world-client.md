---
id: wasmer-js-client-hello-world
title: Wasmer-JS in the Browser
sidebar_label: Hello World
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/client/examples/hello-world)

# Hello World! in the Browser

In this very simple example, we want to develop a Browser-based application that calls a WebAssembly module that in turn calls a native "OS" function.

`JavaScript` --> `WebAssembly` --> `Native "OS" function`

In this case, we will invoke the simple WASM module [`as-echo`](https://github.com/torch2424/as-echo) that receives a text string and echoes it back by writing it to standard out.

However, files descriptors such as "standard in" and "standard out" are not normally available to a WebAssembly module since they belong to the underlying "OS".  Therefore, we must make use of the following two packages:

| Package Name | Description
|---|---|
| `@wasmer/wasi` | Bridges the gap between the black-box world of a WebAssembly module and functionality available from the host environment
| `@wasmer/wasmfs` | A sandboxed filesystem with which `@wasmer/wasi` can interact

This example will be bundled and served by [Parcel](https://parceljs.org/) and run in the browser.

# Setup Instructions

## Prerequisites

Make sure [Parcel](https://parceljs.org/) has been installed and is available from the command line

```bash
$ npm install -g parcel
```

> ### Mac users
> Before the installation of Parcel will work on a Mac, you must first install the [Xcode Command Line Tools](https://developer.apple.com/download/more/?=for%20Xcode)

## Step-By-Step Guide

1. Change into some development directory

    ```bash
    $ cd <some_development_directory>
    ```

1. Create and then change into a new project directory, then run `npm init`

    ```bash
    $ mkdir wasmer-js-hello-world
    $ cd wasmer-js-hello-word
    $ npm init
    ```

    After answering all the questions from `npm init`, you will have a configured `package.json` file.
 
1. For the purposes of testing, we need to install both the `parcel-bundler` and `parcel-plugin-static-files-copy` packages.  These packages allows us to serve our wasm files as static assets:

    ```bash
    npm install --save-dev parcel-bundler parcel-plugin-static-files-copy
    ```

    This command both installs the required packages and updates the `devDependencies` section of your `package.json` file.

1. Create a bare-bones `index.html` file that contains nothing more than the request to load the JavaScript file `index.js`:

    ```html
    <html>
    <body>
      <script src="./index.js"></script>
    </body>
    </html>
    ```

1. Create the file `index.js` and add the following single line of code:

    ```JavaScript
    console.log('I am working')
    ```

1. Let's test that the basic file structure of our project is correct:

    ```bash
    $ parcel index.html
    Server running at http://localhost:1234 
    ✨  Built in 1.15s.
    ```

    Point your browser to [`http://localhost:1234`](http://localhost:1234) and you should see a blank page.
    
    Open your browser's Developer Tools and look at the JavaScript console. Here, you should see ```"I am working"```, which means everything is working!

1.  Now that the basic file structure of our project has been set up correctly, we must next declare the use of packages `@wasmer/wasi` and `@wasmer/wasmfs`.

    To install these packages as runtime dependencies to our project, run the following:

    ```bash
    $ npm install --save @wasmer/wasi @wasmer/wasmfs
    ```

1. Create a new directory called `static`

    ```bash
    $ mkdir static
    ```
    
1. Download the WebAssembly module [`as-echo.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/wasm_lib/as-echo.wasm) and store it in this directory

1. Now we need to change our `index.js` to implement the required functionality.

    > ### Code Sample
    > Seeing as this is demo code, it uses meaningful variable names and contains additional explanatory comments &mdash; features that are often sadly missing from production code...
    > 
    > Please take some time to read and understand these comments as they explain how the functionality has been constructed.
    >
    > Also, make a note of the comment explaining the use of `@wasmer/wasm-transformer`; we will cover this very important detail in a later example.

    ```JavaScript
    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Imports
    import { WASI }   from '@wasmer/wasi'
    import { WasmFs } from '@wasmer/wasmfs'

    const wasmFilePath = './as-echo.wasm'  // Path to our WASI module
    const echoStr      = 'Hello World!'    // Text string to echo

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Instantiate new WASI and WasmFs Instances
    // IMPORTANT:
    // Instantiating WasmFs is only needed when running in a browser.
    // When running on the server, NodeJS's native FS module is assigned by default
    const wasmFs = new WasmFs()

    let wasi = new WASI({
      // Arguments passed to the Wasm Module
      // The first argument is usually the filepath to the executable wasi module
      // we want to run.
      args: [wasmFilePath, echoStr],

      // Environment variables that are accesible to the Wasi module
      env: {},

      // Bindings that are used by the WASI Instance (fs, path, etc...)
      bindings: {
        ...WASI.defaultBindings,
        fs: wasmFs.fs
      }
    })

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Preserve the original console.log functionality
    const consoleLog = console.log

    // Implement our own console.log functionality
    console.log = (...args) =>
      (logTxt => {
        consoleLog(logTxt)
        document.body.appendChild(
          document.createTextNode(logTxt)
        )
      })
      (args.join(' '))

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Async function to run our wasi module/instance
    const startWasiTask =
      async pathToWasmFile => {
        // Fetch our WASM File
        let response  = await fetch(pathToWasmFile)
        let wasmBytes = new Uint8Array(await response.arrayBuffer())

        // IMPORTANT:
        // Some WASI module interfaces use datatypes that cannot yet be transferred
        // between environments (for example, you can't yet send a JavaScript BigInt
        // to a WebAssembly i64).  Therefore, the interface to such modules has to
        // be transformed using `@wasmer/wasm-transformer`, which we will cover in
        // a later example

        // Instantiate the WebAssembly file
        let { instance } = await WebAssembly.instantiate(wasmBytes, {
          wasi_unstable: wasi.wasiImport
        })

        wasi.start(instance)                      // Start the WASI instance
        let stdout = await wasmFs.getStdOut()     // Get the contents of /dev/stdout
        console.log(`Standard Output: ${stdout}`) // Write WASI's stdout to the DOM
      }

    // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    // Everything starts here
    startWasiTask(wasmFilePath)
    ```

1. After saving `index.js` and as long as `parcel` is still running, your browser should automatically refresh and you should see `Standard Output: Hello World!` appear both on the browser screen and in the JavaScript console.




Next, let's take a look at transforming WASI modules that require transformations.
