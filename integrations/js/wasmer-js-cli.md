# Wasmer-JS CLI

The Wasmer-JS command line interface \(CLI\) uses a modern Javascript Workflow, which depends on [Node.js](https://nodejs.org/en/).

## Install Node.js

First, we will start by installing the latest Long Term Support \(LTS\) version of Node.js which includes Node's Package Manager `npm`.

There several are different ways to install Node.js. You could:

* Download a binary installer from the [Node.js download site](https://nodejs.org/en/download/), or
* You can use a tool such as `nvm`. \(Mac and Linux: [here](https://github.com/creationix/nvm), Windows: [here](https://github.com/coreybutler/nvm-windows)\).

  ```bash
  nvm install --lts
  ```

Once we have the latest version of Node installed, we can verify that it is working by running the command `node -v && npm -v`. You should see output similar to the following:

```bash
$ node -v && npm -v
v10.16.2
6.9.0
```

## Install Wasmer-JS CLI

To install the Wasmer-JS CLI globally on your system, run the following command:

```bash
$ npm install -g @wasmer/cli
```

The Wasmer-JS CLI tool should now be installed. To test it, run the command `wasmer-js --help` and you should see output similar to the following:

```bash
$ wasmer-js --help
wasmer-js 0.7.1
The Wasmer Engineering Team <engineering@wasmer.io>
Node.js Wasm execution runtime.

USAGE:
    wasmer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help           Prints this message or the help of the given subcommand(s)
    run            Run a WebAssembly file. Formats accepted: wasm, wat
    validate       Validate a Web Assembly binary
```

Please take a look at the help output to start running Wasm modules.

The same help output is also available in this documentation as the [User Guide](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/wasmer-js/cli/wasmer-js-cli-user-guide/README.md) on the sidebar.

