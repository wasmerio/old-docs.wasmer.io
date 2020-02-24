# Hello World

{% hint style="success" %}
**Note**: The final code for this example can be found on [GitHub](https://github.com/wasmerio/docs.wasmer.io/tree/master/integrations/js/wasi/server/examples/hello-world).
{% endhint %}

In this introductory example, we will develop a NodeJS-based application that calls a WebAssembly module that in turn, calls a native "OS" function. This is exactly the same call chain as was used in the client-side example:

`JavaScript` --&gt; `WebAssembly` --&gt; `Native "OS" function`

In this case, we will invoke the a simple WASI module that does nothing more than writing `hello world` to standard out.

However, as we saw with the [client-side `hello-world` example](https://github.com/wasmerio/docs.wasmer.io/tree/e0f7639306bb4cf18cd0c23876b80f787d6b5876/integrations/js/wasi/browser/examples/hello-world/README.md), file descriptors such as "standard in" and "standard out" are not normally available to a WebAssembly module since they belong to the underlying "OS". Therefore, we must again make use of the following package:

| Package Name | Description |
| :--- | :--- |
| `@wasmer/wasi` | A set of JavaScript polyfills that bridge the gap between the black-box world of a WebAssembly module and functionality available from the host environment |

{% hint style="warning" %}
#### Important

Notice that for a server-side implementation, the `@wasmer/wasmfs` package is _**not**_ needed.
{% endhint %}

## Setup Instructions

### Step-By-Step Guide

1. Change into some development directory

   ```bash
    $ cd <some_development_directory>
   ```

2. Create and then change into a new project directory, then run `npm init`

   ```bash
    $ mkdir wasmer-js-node-hello-world
    $ cd wasmer-js-node-hello-world
    $ npm init
   ```

   After answering all the questions from `npm init`, you will have a configured `package.json` file.

3. Declare the use of package `@wasmer/wasi` as a runtime dependency by running the command:

   ```bash
   $ npm install --save @wasmer/wasi
   ```

4. Download the WebAssembly module [`helloworld.wasm`](https://github.com/wasmerio/docs.wasmer.io/raw/master/integrations/shared/wat/wasi/helloworld.wasm) and store it in this directory
5. Create the file `index.js` and add the coding shown below.

   > #### Important Difference
   >
   > In contrast to running in the browser, the server-side implementation of the same WASM module is noticeably smaller.
   >
   > When running server-side, we do not need to write any code to obtain the contents of standard out after the WASM module has executed, since when running server-side, anything written to standard out by a WASM module appears directly in the console.

   ```javascript
    const fs       = require("fs")
    const { WASI } = require("@wasmer/wasi")
    const nodeBindings = require("@wasmer/wasi/lib/bindings/node")

    const wasmFilePath = "./helloworld.wasm"

    // Instantiate a new WASI Instance
    let wasi = new WASI({
      args: [wasmFilePath],
      env: {},
      bindings: {
        ...nodeBindings,
        fs: fs
      }
    })

    // Async function to run our WASM module/instance
    const startWasiTask =
      async pathToWasmFile => {
        // Fetch our Wasm File
        let wasmBytes = new Uint8Array(fs.readFileSync(pathToWasmFile)).buffer

        // Instantiate the WebAssembly file
        let { instance } = await WebAssembly.instantiate(wasmBytes, {
          wasi_unstable: wasi.wasiImport
        })

        // Start the WASI instance
        wasi.start(instance)
      }

    // Everything starts here
    startWasiTask(wasmFilePath)
   ```

6. Save `index.js` and run it using:

   ```bash
    $ node index.js
    Hello World!
   ```

Next, let's take a look at running WASM modules whose interfaces require transformation.

{% hint style="info" %}
If you want to run the examples from the docs codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/docs.wasmer.io.git
cd docs.wasmer.io/integrations/js/wasi/server/examples/hello-world
npm run dev
```
{% endhint %}

