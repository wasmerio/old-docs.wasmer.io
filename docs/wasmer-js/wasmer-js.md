---
id: wasmer-js
title: Wasmer-JS Introduction
sidebar_label: Wasmer-JS Introduction
---

![Wasmer JS Announcement Image](/img/wasmer-js/wasmer-js-announcement.png)

## Scenario

In this use-case, we wish to use a JavaScript program to invoke a WebAssembly module that further invokes native "OS" functionailty.  In other words, we wish to implement the following call chain:

![Wasmer JS Schematic Image](/img/wasmer-js/wasmer-js-schematic1.png)

JavaScript has native support for WebAssembly, but without a tool such as the WebAssembly System Interface ([`WASI`](https://github.com/webassembly/wasi)), a WebAssembly module does not normally have access to any OS-level functionality.

Within the context of a JavaScript runtime environment, WASI functionality must be provided by means of a set of JavaScript polyfills &mdash; and this is the role of Wasmer-js.

![Wasmer JS Schematic Image](/img/wasmer-js/wasmer-js-schematic2.png)

> ### IMPORTANT
> The JavaScript environment shown here could either be on the client (within a browser), or on the server (provided by [Node.js](https://nodejs.org/en/)); either way, the native functions invoked from WebAssembly via Wasmer-JS and `WASI` belong to the ***JavaScript*** runtime environment, ***not*** the underlying operating system.

This project has been set up as mono-repo of multiple JavaScript packages.

* [Github Repo](https://github.com/wasmerio/wasmer-js)
