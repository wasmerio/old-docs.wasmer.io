---
id: wasmer-js-node-modules-installation
title: Wasmer-JS Node Modules Installation
sidebar_label: Installation
---

The Wasmer-JS Stack uses a modern Javascript Workflow, which depends on Node.Js.

## Prerequisites

It is assumed that you already have Node.js installed.  If this is not the case, please follow the installation instructions on the [Wasmer-JS CLI installation page](../cli/wasmer-js-cli-installation) before returning to this page to continue with the instructions below.

## Create a New Project

Now that Node.js is up and running, let's go ahead and create a small node project, you can skip this setup if you are installing Wasmer-JS to an existing project.

```bash
$ mkdir my-project
$ cd my-project
$ npm init # Fill our the prompts here
```

Now that we have a new project, we can install the required Node modules to it!

To do this, we can install the appropriate Wasmer-JS package with the following command:

```bash
$ npm install --save @wasmer/<wasmer-js-package-name>
```

For example, all your Node.js projects will require the Wasmer-JS base package

```bash
$ npm install --save @wasmer/wasi
```

You should now be able to use the Wasmer-JS package! Please take a look at the examples, or one of the tutorials to get started.
