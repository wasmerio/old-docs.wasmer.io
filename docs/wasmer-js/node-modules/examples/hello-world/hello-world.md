---
id: wasmer-js-node-modules-hello-world
title: Wasmer-JS Node Modules Hello World
sidebar_label: Hello World
---

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/node-modules/examples/hello-world)

In this example, we will run the WASI module [as-echo](https://github.com/torch2424/as-echo), using `@wasmer/wasi`. 

The WASI module will echo "Hello World", and we will get the standard output from `/dev/stdout` using the `@wasmer/wasmfs` module. 

This example will be run in the browser, bundled and served by [Parcel](https://parceljs.org/). 

However, `@wasmer/wasi` and `@wasmer/wasmfs` also work in Node, and the code examples from this guide can be used as a loose example, as long as the ES6 syntax is replaced with what would be the node equivalent.

## Initial Setup

1. Make sure [Parcel](https://parceljs.org/) has been installed and is available from the command line

    ```bash
    $ npm install -g parcel
    ```

    > ### Information for Mac users
    > Before the installation of Parcel will work on a Mac, you must first install the [Xcode Command Line Tools](https://developer.apple.com/download/more/?=for%20Xcode)

1. Create a small JavaScript project.
    1. Change into your development directory
    2. Create a new subdirectory
    3. Initialize the new subdirectory as a Node project using `npm`:

    ```bash
    $ cd <some_development_directory>
    $ mkdir wasmer-hello-world
    $ cd wasmer-hello-world
    $ npm init
    ```

    After you answer the questions from `npm init`, a new `package.json` file will be created.

1. Install both the `parcel-bundler` and `parcel-plugin-static-files-copy` packages.  These allow `parcel` to serve our wasm files as static assets.  The following command both installs the required packages and updates the `devDependencies` section of your `package.json` file:

    `npm install --save-dev parcel-bundler parcel-plugin-static-files-copy`

1. Create an `index.html` file that contains only the following request to load the JavaScript file `index.js`:

    ```html
    <html>
    <body>
      <script src="./index.js"></script>
    </body>
    </html>
    ```

1. Since we have references a JavaScript file called `index.js`, we need to create it.  At the moment, this file only needs to contain a single line of code to demonstrate that our file structure is set up correctly:

    ```javascript
    console.log('I am working')
    ```

1. Finally, let's serve our minimal project using the `parcel` CLI:

    `parcel index.html`
    
    You should see a response that tells you the server's URL and the build time.  Something like this:
    
    ```bash
    Server running at http://localhost:1234 
    ✨  Built in 982ms.
    ```

    Point your browser to the URL shown above [`http://localhost:1234`](`http://localhost:1234`), and you should see a blank page.
    
    Excellent, its working!  &nbsp;&nbsp;&nbsp;&nbsp;Uh, wait a minute...
    
    Just to be sure, open the Developer Tools and you should see `"I am working"` written to the JavaScript console, meaning everything really is working!

## Using `wasmer-js`

1. Now that our Javascript project has been set up, let's add the following `wasmer-js` packages: 

    - `@wasmer/wasi` - A polyfill to implement any `wasi` bindings your browser might not support
    - `@wasmer/wasm` - A sandboxed filesystem with which our `wasi` module can interact

    To install these packages as dependencies to your project, run the following command:

    `npm install --save @wasmer/wasi @wasmer/wasmfs`
    
    As we've seen before, this command updates the `dependencies` section of your `package.json` file

1. We now need to include a `wasm` file to be run from the browser.

    In your project directory `wasmer-hello-world`, create a new subdirectory called `static`

    ```bash
    $ mkdir static
    ```

1. Download [`as-echo.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/node-modules/examples/hello-world/static/as-echo.wasm) and store this file in the `static` directory

1. Now that these packages are installed, let's replace `index.js` with the following:

    ```javascript
    // *****************************************************************************
    // Imports
    import { WASI }   from '@wasmer/wasi'
    import { WasmFs } from '@wasmer/wasmfs'

    const wasmFilePath = './as-echo.wasm'  // Path to our wasi module
    const echoStr      = 'Hello World!'    // Text string to echo

    // *****************************************************************************
    // Instantiate new WASI and WasmFs Instances
    // NOTE:
    // If running in NodeJS, WasmFs is not needed.  In this case, Node's native FS
    // module is assigned by default.
    // Here however, we want to show off how to use WasmFs within the browser.
    // This also means that all our file system operations are sand-boxed.
    // In other words, the wasi module running in the browser does not have any
    // access to the file system of the machine running the browser
    const wasmFs = new WasmFs()

    let wasi = new WASI({
      // Arguments passed to the Wasm Module
      // The first argument is usually the filepath to the "executable wasi module"
      // we want to run.
      args: [wasmFilePath, echoStr],

      // Environment variables that are accesible to the Wasi module
      env: {},

      // Bindings that are used by the Wasi Instance (fs, path, etc...)
      bindings: {
        ...WASI.defaultBindings,
        fs: wasmFs.fs
      }
    })

    // *****************************************************************************
    // Preserve the original console.log functionality
    const consoleLog = console.log

    // Implement our own console.log functionality
    console.log = (...args) =>
      (logTxt => {
        consoleLog(logTxt)
        document.body.appendChild(
          document.createTextNode(`JavaScript Console: ${logTxt}`)
        )
      })
      (args.join(' '))

    // *****************************************************************************
    // Async function to run our wasi module/instance
    const startWasiTask =
      async pathToWasmFile => {
        // Fetch our Wasm File
        let response  = await fetch(pathToWasmFile)
        let wasmBytes = new Uint8Array(await response.arrayBuffer())

        // IMPORTANT:
        // Some wasi modules import datatypes that cannot yet be supplied by all
        // JavaScript environments (for example, you can't yet import a JavaScript
        // BigInt into WebAssembly).  Therefore, the interface to such modules has
        // to be transformed using `@wasmer/wasm-transformer`, which we will cover
        // in later examples

        // Instantiate the WebAssembly file
        let { instance } = await WebAssembly.instantiate(wasmBytes, {
          wasi_unstable: wasi.wasiImport
        })

        wasi.start(instance)                      // Start the WASI instance
        let stdout = await wasmFs.getStdOut()     // Get the contents of /dev/stdout
        console.log(`Standard Output: ${stdout}`) // Write wasi's stdout to the DOM
      }

    // *****************************************************************************
    // Everything starts here
    startWasiTask(wasmFilePath)
    ```

    Please take some time to read and understand the comments in the example above as this will help you understand how the functionality has been constructed.
    
    Also, make a note of the comment explaining `@wasmer/wasm-transformer`; we will cover this very important detail in a later example.

    Save `index.js`  and reload your browser window.  You should see the `Hello World` message in both the Javascript console and in the webpage itself!

Next, let's take a look at transforming Wasi modules that require transformations.
