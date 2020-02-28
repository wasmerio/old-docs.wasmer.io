# Wasmer-JS CLI

The Wasmer-JS command line interface \(CLI\) uses a modern Javascript Workflow, which depends on [Node.js](https://nodejs.org/en/).

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

