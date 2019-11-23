---
id: wasmer-js-node-modules-installation
title: Wasmer-JS Node Modules Installation
sidebar_label: Installation
---

The Wasmer-JS Stack uses a modern Javascript Workflow, which depends on Node.Js.

First, we will start with installing the latest LTS version of [Node.js](https://nodejs.org/en/) (which includes npm). An easy way to do so is with nvm. (Mac and Linux: [here](https://github.com/creationix/nvm), Windows: [here](https://github.com/coreybutler/nvm-windows)).

`nvm install --lts`

Once we have the latest node installed, we can verify that it is working, by running the following:

```bash
$ node -v
# Your node version would appear here.
$ npm -v
# Your npm (Node Package Manager) version would appear here.
```

Next, let's go ahead and create a small node project, you can skip this setup if you are installing wasmer-js to an existing project.

```bash
$ mkdir my-project
$ cd my-project
$ npm init # Fill our the prompts here
```

Now that we have a node project, we can install Node modules to it! To do this, we can install the appropriate Wasmer-JS package with the following command:

```bash
npm install --save @wasmer/wasmer-js-package-here
# For Example: npm install --save @wasmer/wasi
```

You should now be able to use the wasmer-js package! Please take a look at the examples, or one of the tutorials to get started.
