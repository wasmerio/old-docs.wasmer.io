---
id: wapm-getting-started
title: WAPM Getting Started
sidebar_label: Getting Started
---

This guide will introduce the *wapm-cli* tool and show a few common workflows. The wapm client can install packages, manage WebAssembly dependencies, and expose WebAssembly behavior with commands.

## **What is wapm?**

The wapm ecosystem makes WebAssembly more accessible to developers. The system is enabled by a couple tools:

- wapm package registry for storing and serving packages
- wapm package client (called *wapm-cli*) for installing and managing packages

The tool comes bundled with [wasmer: the WebAssembly runtime](https://wasmer.io/), but it works great with the other [server-side runtimes](https://github.com/mbasso/awesome-wasm#non-web-embeddings) and the web!

The name wapm is an abbreviation for WebAssembly Package Manager, but then name represents the ecosystem of packages and the tools.

## **Basic Usage**

With the tools installed, one can easily start using universal wasm binaries by using the `install` command of *wapm-cli*:

`wapm install cowsay`

Installing a package creates a local package directory called `wapm_packages` where all packages are installed. While in a directory with wapm packages, one may execute them with the `run` command:

`wapm run cowsay hello wapm!`

```
 _____________
< hello wapm! >
 -------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||
```

## **Package with `wapm.toml`**

The manifest file enables one to publish a package to the [wapm.io](https://wapm.io/) public registry.

Login is required for publishing. Signup for an account at [wapm.io](https://wapm.io/) and login to the server with `login` command.

The reference shows all the required fields for the manifest, but it's easy to get started with `init` command.

`wapm init`

This command generates a wapm manifest like the following:

```
[package]
name = "username/my_package"
version = "0.1.0"
description = ""
```

All packages on [wapm.io](https://wapm.io/) are namespaced by username. This is the minimum required data for a manifest file. A module is required for publishing. Add a module section:

```
[[module]]
name = "my_module"
source = "path/to/my_module.wasm"
```

Publish the project to [wapm.io](https://wapm.io/)!

`wapm publish`

## **Commands**

Commands (not to be confused with *wapm-cli* subcommands) are a feature that enables easily executing wasm code from a wapm package.

Commands are what allows one to call the `run` subcommand, like above when running `wapm run cowsay hello wapm!`.

A command requires a name and module to reference:

```
[[command]]
name = "my_cmd"
module = "my_module"
```

Now called `wapm run my_cmd` will execute the module defined with the name `my_module`. Under the hood, *wapm-cli* calls *wasmer*, the WebAssembly server runtime.
