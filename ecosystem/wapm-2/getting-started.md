---
id: wapm-getting-started
title: WAPM Getting Started
sidebar_label: Getting Started
---

# Getting Started

This guide will introduce the `wapm` tool and show a few common workflows. The WAPM client can install packages, manage WebAssembly dependencies, and expose WebAssembly behavior with commands.

## Install WAPM

WAPM comes bundled with Wasmer, so you just need to install Wasmer in your system to have the `wapm` CLI!

{% content-ref url="../wasmer/getting-started.md" %}
[getting-started.md](../wasmer/getting-started.md)
{% endcontent-ref %}

## **Basic Usage**

With the tools installed, one can easily start using universal Wasm binaries by using the `install` command of `wapm` CLI:

```
wapm install cowsay
```

Installing a package creates a local package directory called `wapm_packages` where all packages are installed.

{% hint style="info" %}
#### Did you know ...?

A WAPM package can be installed globally by adding the `-g` flag.\
Eg:`wapm install -g cowsay`
{% endhint %}

While in a directory with WAPM packages, one may execute them with the `run` command:

```
wapm run cowsay hello wapm!
```

```
< hello wapm! >
 -------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
               ||----w |
                ||     ||
```

You can also install specific versions of packages using an `@`

```
wapm install cowsay@0.1.3
```

Great! You used a WAPM package in your system... now let's try to create our own package and publish it to WAPM so other users can use it

{% hint style="success" %}
When executing WAPM, you can also customize the WebAssembly runtime used under the hood.

For example, if you want to use [`wasmer-js`](../../integrations/js/wasmer-js-cli.md) as the runtime, you can do:

```
WAPM_RUNTIME=wasmer-js wapm run cowsay hello wapm!
```
{% endhint %}
