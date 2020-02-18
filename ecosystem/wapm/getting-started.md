---
id: wapm-getting-started
title: WAPM Getting Started
sidebar_label: Getting Started
---

# Getting Started

This guide will introduce the `wapm` tool and show a few common workflows. The wapm client can install packages, manage WebAssembly dependencies, and expose WebAssembly behavior with commands.

## **What is WAPM?**

The WAPM ecosystem makes WebAssembly more accessible to developers. The system is enabled by a couple tools:

* wapm package registry for storing and serving packages
* wapm package client \(called _wapm-cli_\) for installing and managing packages

The tool comes bundled with [wasmer: the WebAssembly runtime](https://wasmer.io/), but it works great with the other [server-side runtimes](https://github.com/mbasso/awesome-wasm#non-web-embeddings) and the web!

The name wapm is an abbreviation for WebAssembly Package Manager, but then name represents the ecosystem of packages and the tools.

## **Basic Usage**

With the tools installed, one can easily start using universal wasm binaries by using the `install` command of _wapm-cli_:

```text
wapm install cowsay
```

Installing a package creates a local package directory called `wapm_packages` where all packages are installed. While in a directory with wapm packages, one may execute them with the `run` command:

```
wapm run cowsay hello wapm!
```

```text
< hello wapm! >
 -------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||
```

Great! You used a WAPM package in your system... now let's try to create our own package and publish it to WAPM so other users can use it

