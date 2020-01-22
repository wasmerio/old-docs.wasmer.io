---
id: wasmer-js
title: Wasmer-JS Introduction
sidebar_label: Wasmer-JS Introduction
---

![Wasmer JS Announcement Image](/img/wasmer-js/wasmer-js-announcement.png)

## Purpose

[Wasmer-JS](https://github.com/wasmerio/wasmer-js) is the bridge that allows a JavaScript program to invoke a WebAssembly module that further invokes native "OS" functionailty, making the following call chain possible:

![Wasmer JS Schematic Image](/img/wasmer-js/wasmer-js-schematic.png)

> ### IMPORTANT
> The JavaScript environment shown here could either be on the client (within a browser), or on the server (provided by [Node.js](https://nodejs.org/en/)); either way, the native functions invoked from WebAssembly via Wasmer-JS and `WASI` belong to the ***JavaScript*** runtime environment, ***not*** the underlying operating system.

This project has been set up as mono-repo of multiple JavaScript packages.

* [Github Repo](https://github.com/wasmerio/wasmer-js)
