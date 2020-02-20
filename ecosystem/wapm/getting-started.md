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

