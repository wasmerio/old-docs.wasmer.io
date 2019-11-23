---
id: wasmer-js-cli-installation
title: Wasmer-JS Command Line Interface Installation
sidebar_label: Installation
---

The Wasmer-JS CLI uses a modern Javascript Workflow, which depends on Node.Js.

First, we will start with installing the latest LTS version of [Node.js](https://nodejs.org/en/) (which includes npm). An easy way to do so is with nvm. (Mac and Linux: [here](https://github.com/creationix/nvm), Windows: [here](https://github.com/coreybutler/nvm-windows)).

`nvm install --lts`

Once we have the latest node installed, we can verify that it is working, by running the following:

```bash
$ node -v
# Your node version would appear here.
$ npm -v
# Your npm (Node Package Manager) version would appear here.
```

The Wasmer-JS CLI is installed globally on your system. To do this, run the following command:

`npm install -g @wasmer/cli`

The CLI should then be installed. To test the CLI run:

`wasmer-js help`

The help message should then be outputted to the console, meaning everything was installed successfully!

Please take a look at the help output to start running Wasm modules. The help output is also available as the user guide on the sidebar.
