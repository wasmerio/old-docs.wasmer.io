---
id: wasmer-js-server-installation
title: Wasmer-JS Node Modules Installation
sidebar_label: Server-side Installation
---

# installation

The Wasmer-JS stack uses a modern Javascript workflow, which depends on Node.Js.

## Prerequisites

It is assumed that you already have Node.js installed.

If this is not the case, please follow the instructions for installing Node.js on the [Wasmer-JS CLI installation page](https://github.com/wasmerio/docs.wasmer.io/tree/ca2c9145ea511f3c00439b180be82cc5197a177f/docs/wasmer-js/cli/wasmer-js-cli-installation/README.md) before returning to this page to continue with the instructions below.

## Create a New Project

Now that Node.js is up and running, let's go ahead and create a small project.

You can skip this setup if you are installing Wasmer-JS to an existing project.

```bash
$ mkdir my-project
$ cd my-project
$ npm init
```

After answering all the questions from `npm init`, you will have a configured `package.json` file.

Now that we have a new project, we can install the required Node modules to it!

To do this, we can install the appropriate Wasmer-JS package with the following command:

```bash
$ npm install --save @wasmer/<wasmer-js-package-name>
```

For example, all your Node.js projects will require the Wasmer-JS base package

```bash
$ npm install --save @wasmer/wasi
```

You should now be able to use the Wasmer-JS package.

Please take a look at the examples, or one of the tutorials to get started.

