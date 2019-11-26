---
id: wasmer-js-node-modules-hello-world
title: Wasmer-JS Node Modules Hello World
sidebar_label: Hello World
---

In this example, we will run the wasi module [as-echo](https://github.com/torch2424/as-echohttps://github.com/torch2424/as-echo), using `@wasmer/wasi`. The wasi module will echo "Hello World", and we will get the standard output from `/dev/stdout` using the `@wasmer/wasmfs` module. This example will be run in the browser, bundled and served by Parcel. However, `@wasmer/wasi` and `@wasmer/wasmfs` also work in Node, and the code examples from this guide can be used as a loose example, as long as the ES6 syntax is replaced with what would be the node equivalent.

First, let's set up a small JavaScript project using Parcel. To do this, we will create a new directory, and initialize it as an node project with npm:

```bash
mkdir my-wasmer-js-app
cd my-wasmer-js-app
npm init
```

Then, answer the prompts from `npm init`. After this, we will install the `parcel-bundler` package, as well as `parcel-plugin-static-files-copy` to allow serving our wasm files as a static asset:

`npm install --save-dev parcel-bundler parcel-plugin-static-files-copy`

Let's add some files to ensure parcel is working! First, let's add the `index.html` shown one their homepage:

```html
<html>
<body>
  <script src="./index.js"></script>
</body>
</html>
```

Then, let's add our `index.js` script tag:

``console.log('I am working')``

Let's serve our project so far using the parcel CLI:

`parcel index.html`

If we open the URL in the browser like parcel suggests, `http://localhost:1234`, we should get a blank page. But if we open the JavaScript console, we should see "I am working" logged, meaning everything is working!

Now that we got our Javascript project set up, let's go ahead and add the following Wasmer-JS packages: 

- `@wasmer/wasi` - which will act as a polyfill for wasi bindings that the browser does not support.
- `@wasmer/wasm` - which provides a sandboxed filesystem that our wasi module will interact with

To install these packages as dependencies to the project, run the following:

    npm install --save @wasmer/wasi @wasmer/wasmfs

Now that these packages are installed, let's change our index.js to the following:

```javascript
// Imports
import { WASI } from '@wasmer/wasi';
import { WasmFs } from '@wasmer/wasmfs';

// The file path to the wasi module we want to run
const wasmFilePath = './as-echo.wasm';

// A quick wrapper for console.log, to also output logs to the body
const consoleLog = console.log;
console.log = function() {
  const args = Array.prototype.slice.call(arguments);
  consoleLog(args);
  const log = args.join(' ');
  consoleLog(log);
  document.body.appendChild(document.createTextNode('JavaScript Console: ' + log));
}

// Instantiate a new WASI and WasmFs Instance
// NOTE: For node WasmFs is not needed, and the native Fs module is assigned by default
// In this case, we want to show off WasmFs for the browser use case, and we want to
// "Sandbox" our file system operations
const wasmFs = new WasmFs();
let wasi = new WASI({
  // Arguments to pass to the Wasm Module
  // The first argument usually should be the filepath to the "executable wasi module"
  // That we want to run.
  args: [wasmFilePath, 'Hello World!'],
  // Environment variables that are accesible to the Wasi module
  env: {},
  // Bindings that are used by the Wasi Instance (fs, path, etc...)
  bindings: {
    ...WASI.defaultBindings,
    fs: wasmFs.fs
  }
});

// Async Function to run our wasi module/instance
const startWasiTask = async () => {
  // Fetch our Wasm File
  const response = await fetch(wasmFilePath);
  const responseArrayBuffer = await response.arrayBuffer();
  const wasmBytes = new Uint8Array(responseArrayBuffer).buffer;

  // NOTE: For some wasi modules, they have wasi imports that are not supported in
  // all JavaScript environments. Meaning we will have to use `@wasmer/wasm-transformer`,
  // which we will cover in later examples

  // Instantiate the WebAssembly file
  let { instance } = await WebAssembly.instantiate(wasmBytes, {
    wasi_unstable: wasi.wasiImport
  });

  // Start the WebAssembly WASI instance!
  wasi.start(instance);

  // Output what's inside of /dev/stdout!
  const stdout = await wasmFs.getStdOut();
  // Add the Standard output to the dom
  console.log('Standard Output: ' + stdout);
};
startWasiTask();
```

Please take a look at the comments in the example above for an explanation of how things are written and working. Also, the comment explaining `@wasmer/wasm-transformer`, which we will cover in a later example.

After the `index.js` is written to be the above example, reload your browser window, and you should see the hello world message in both the Javascript console, and in the webpage itself!

Next, let's take a look at transforming Wasi modules that require transformations.
