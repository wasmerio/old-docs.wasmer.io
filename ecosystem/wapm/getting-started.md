---
id: wapm-getting-started
title: WAPM Getting Started
sidebar_label: Getting Started
---

# Getting Started

This guide will introduce the `wapm` tool and show a few common workflows. The wapm client can install packages, manage WebAssembly dependencies, and expose WebAssembly behavior with commands.

## Install WAPM

WAPM comes bundled with Wasmer, so you just need to install Wasmer in your system to have the `wapm` CLI!

{% page-ref page="../wasmer/getting-started.md" %}

## **What is WAPM?**

The WAPM ecosystem makes WebAssembly more accessible to developers. The system is enabled by a couple tools:

* WAPM package registry for storing and serving packages
* WAPM package client \(called `wapm` CLI\) for installing and managing packages

The tool comes bundled with [Wasmer: the WebAssembly runtime](https://wasmer.io/), but it works great with the other [server-side runtimes](https://github.com/mbasso/awesome-wasm#non-web-embeddings) and the web!

The name WAPM is an abbreviation for WebAssembly Package Manager, but then name represents the ecosystem of packages and the tools.

## **Basic Usage**

With the tools installed, one can easily start using universal Wasm binaries by using the `install` command of `wapm` CLI:

```text
wapm install cowsay
```

Installing a package creates a local package directory called `wapm_packages` where all packages are installed. While in a directory with wapm packages, one may execute them with the `run` command:

```text
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

