# Hello World in the Browser

[Full Example Project Source Code](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/client/examples/hello-world)

In this introductory example, we will develop a Browser-based application that uses the following call chain.

`JavaScript` --&gt; `WebAssembly` --&gt; `Native "OS" function`

In this case, we will invoke the WASM module [`as-echo`](https://github.com/torch2424/as-echo) that receives a text string and does nothing more than echo it back by writing it to standard out.

{% hint style="info" %}
#### Under the Hood

The WASM function `as-echo` calls the native "OS" function `fd_write` that writes data to a particular file descriptor \(hence `fd` in the function name\)
{% endhint %}

However, interaction with file descriptors such as "standard in" and "standard out" is not normally possible for a WebAssembly module, since this type of functionality belongs to the underlying "OS". Therefore, we must make use of the following two packages:

| Package Name | Description |
| :--- | :--- |
| `@wasmer/wasi` | A set of JavaScript polyfills to bridge the gap between the black-box world of a WebAssembly module and functionality available in the host environment |
| `@wasmer/wasmfs` | Provide access to a sand-boxed filesystem with which `@wasmer/wasi` can interact |

{% hint style="warning" %}
#### Reminder

The term "OS" used above is in quotes because in this particular case, the native function called by `as-echo` that writes to standard out, belongs to the JavaScript runtime, and not the actual underlying operating system.
{% endhint %}

This example will be bundled and served by [Parcel](https://parceljs.org/) and run in the browser.

## Setup Instructions

### Prerequisites

Make sure [Parcel](https://parceljs.org/) has been installed and is available from the command line

```bash
$ npm install -g parcel
```

> #### Mac users
>
> Before the installation of Parcel will work on a Mac, you must first install the [Xcode Command Line Tools](https://developer.apple.com/download/more/?=for%20Xcode)

### Step-By-Step Guide

1. Change into some development directory

   ```bash
    $ cd <some_development_directory>
   ```

2. Create and then change into a new project directory, then run `npm init`

   ```bash
    $ mkdir wasmer-js-hello-world
    $ cd wasmer-js-hello-word
    $ npm init
   ```

   After answering all the questions from `npm init`, you will have a configured `package.json` file.

3. For the purposes of testing, we need to install both the `parcel-bundler` and `parcel-plugin-static-files-copy` packages.

   These packages allow `parcel` to serve our WASM files as static assets:

   ```bash
    npm install --save-dev parcel-bundler parcel-plugin-static-files-copy
   ```

   This command both installs the required packages and updates the `devDependencies` section of your `package.json` file.

4. Create a bare-bones `index.html` file that contains nothing more than the request to load the JavaScript file `index.js`:

   ```markup
    <html>
    <body>
      <script src="./index.js"></script>
    </body>
    </html>
   ```

5. Create the file `index.js` and add the following single line of code:

   ```javascript
    console.log('I am working')
   ```

6. Let's test that the basic file structure of our project is correct:

   ```bash
    $ parcel index.html
    Server running at http://localhost:1234 
    ✨  Built in 1.15s.
   ```

   Point your browser to [`http://localhost:1234`](http://localhost:1234) and you should see a blank page.

   Open your browser's Developer Tools and look at the JavaScript console. Here, you should see `"I am working"`, which means everything is working!

7. Now that the basic file structure of our project has been set up correctly, we must next declare the use of packages `@wasmer/wasi` and `@wasmer/wasmfs`.

   To install these packages as runtime dependencies to our project, run the following command:

   ```bash
   $ npm install --save @wasmer/wasi @wasmer/wasmfs
   ```

8. Create a new directory called `static`

   ```bash
    $ mkdir static
   ```

9. Download the WebAssembly module [`as-echo.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/docs/wasmer-js/wasm_lib/as-echo.wasm) and store it in this directory
10. Now we need to change the contents of `index.js` to implement the required functionality.

    > #### Code Sample
    >
    > Seeing as this is demo code, it uses meaningful variable names and contains additional explanatory comments — features that are often sadly missing from production code...
    >
    > Please take some time to read and understand these comments as they explain how the functionality has been constructed.
    >
    > Also, please read the comment explaining the use of `@wasmer/wasm-transformer`; we will cover this very important detail in a later example.

    ```javascript
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
       // Arguments passed to the WASM Module
       // The first argument is usually the filepath to the executable WASI module
       // we want to run.
       args: [wasmFilePath, echoStr],

       // Environment variables that are accesible to the WASI module
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

     // Implement our own console.log functionality that also writes to the DOM
     console.log = (...args) =>
       (logTxt => {
         consoleLog(logTxt)
         document.body.appendChild(
           document.createTextNode(logTxt)
         )
       })
       (args.join(' '))

     // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
     // Async function to run our WASI module/instance
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
         let stdout = await wasmFs.getStdOut()     // Get the contents of stdout
         console.log(`Standard Output: ${stdout}`) // Write stdout data to the DOM
       }

     // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
     // Everything starts here
     startWasiTask(wasmFilePath)
    ```

11. As long as `parcel` is still running, after saving `index.js`, your browser should automatically refresh and you should see `Standard Output: Hello World!` appear both on the browser screen and in the JavaScript console.

Next, let's take a look at transforming WASI modules that require transformations.

