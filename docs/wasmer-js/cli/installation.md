---
id: wasmer-js-cli-installation
title: Wasmer-JS Command Line Interface Installation
sidebar_label: Installation
---

The Wasmer-JS command line interface (CLI) uses a modern Javascript Workflow, which depends on [Node.js](https://nodejs.org/en/).

## Install Node.js

First, we will start with installing the latest Long Term Support (LTS) version of Node.js which includes Node's Package manager `npm`.

There are different ways of installing Node.js.  You can either:

* Download a binary installer from the [Node.js download site](https://nodejs.org/en/download/), or
* You can use a tool such as `nvm`. (Mac and Linux: [here](https://github.com/creationix/nvm), Windows: [here](https://github.com/coreybutler/nvm-windows)).

    ```bash
    $ nvm install --lts
    ```

Once we have the latest version of Node installed, we can verify that it is working by running the command `node -v && npm -v`.  You should see output similar to the following:

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
The CLI for executing Wasmer-JS

VERSION
  @wasmer/cli/0.6.0 darwin-x64 node-v10.16.2

USAGE
  $ wasmer-js [COMMAND]

COMMANDS
  help  display help for wasmer-js
  run   Run a WebAssembly file with Wasmer-JS
```

Please take a look at the help output to start running Wasm modules.

The same help output is also available in this documentation as the [User Guide](./wasmer-js-cli-user-guide) on the sidebar.
