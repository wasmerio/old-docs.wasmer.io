# WASI

![](../../../.gitbook/assets/image%20%283%29.png)

## Scenario

In this use-case we wish to use a JavaScript program to invoke a WebAssembly module that further invokes native "OS" functionailty. In other words, we wish to implement the following call chain:

![](../../../.gitbook/assets/image%20%284%29.png)

JavaScript has native support for WebAssembly, but without a tool such as the WebAssembly System Interface \([`WASI`](https://github.com/webassembly/wasi)\), a WebAssembly module does not normally have access to any OS-level functionality.

Within the context of a JavaScript runtime environment, WASI functionality must be provided by means of a set of JavaScript polyfills — and this is the role of Wasmer-js.

![](../../../.gitbook/assets/image%20%285%29.png)

{% hint style="warning" %}
#### Important

The JavaScript environment shown here could either be on the client \(within a browser\), or on the server \(provided by Node.js\); either way, the native functions invoked from WebAssembly via Wasmer-JS and WASI belong to the JavaScript runtime environment, not the underlying operating system.
{% endhint %}

This project has been set up as mono-repo of multiple JavaScript packages.

* [Github Repo](https://github.com/wasmerio/wasmer-js)
