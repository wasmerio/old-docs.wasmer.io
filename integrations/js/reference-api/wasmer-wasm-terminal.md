---
description: >-
  Do you want to run a full terminal with WebAssembly files? This is your
  package!
---

# @wasmer/wasm-terminal

## WasmTerminal

`import { WasmTerminal } from "@wasmer/wasm-terminal"`.

### Constructor

`new WasmTerminal(jsObject: WasmTerminalConfig): WasmTerminal`

Constructor that returns an instance of the `WasmTerminal`.

The [WasmTerminalConfig](https://github.com/wasmerio/wasmer-js/blob/master/packages/wasm-terminal/src/wasm-terminal-config.ts) can be described as the following:

```typescript
{
  // Function that is called whenever a command is entered and returns a Promise,
  // It takes in the name of the command being run, and expects a Uint8Array of a Wasm Binary, or a
  // CallbackCommand (see the api below) to be returned.
  fetchCommand: (options: {
    args: string,
    env?: {[key: string]: string}
  }) => Promise<Uint8Array | CallbackCommand>
  // Only for Optimized Bundles: URL to the /node_modules/wasm-terminal/workers/process.worker.js . This is used by the shell to
  // to spawn web workers in Comlink, for features such as piping, /dev/stdin reading, and general performance enhancements.
  processWorkerUrl?: string;
}
```

`callbackCommands` are functions that can be returned in the `fetchCommand` config property. They are simply Javascript callbacks that take in the command name, command arguments, environment variables, and return a `Promise` that resolves `stdout`.

Since these callback commands handle `stdin` and `stdout`, they can be used as normal commands that can be piped!

```typescript
export type CallbackCommand = (options: {
  args: string,
  env?: {[key: string]: string}
}) => Promise<string | undefined>;
```

### Instance Properties

#### open

`wasmTerminal.open(containerElement: Element): void`

Sets the container of the `wasmTerminal.` The `containerElement` can be any [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element).

#### fit

`wasmTerminal.fit()`: void

Resizes the terminal to fit the size of its container.

#### focus

`wasmTerminal.focus()`: void

Gives the `wasmTerminal` element focus, and allows input into the shell.

#### print

`wasmTerminal.print(message: string)`: void

Prints text to the `wasmTerminal`.

Useful for things such as showing a welcome message before the `wasmTerminal` is opened.

#### scrollToCursor

`wasmTerminal.scrollToCursor()`:void

Scrolls the terminal cursor into view.

#### runCommand

`wasmTerminal.runCommand(commandString: string)`: void

Runs the supplied string as if it had been entered as a command, typed into the WASM terminal.

## fetchCommandFromWAPM

```typescript
import { fetchCommandFromWAPM } from "@wasmer/wasm-terminal";

fetchCommandFromWAPM(options: {
  args: string,
  env?: {[key: string]: string}
}): Promise<Uint8Array>
```

Exported function from the `@wasmer/wasm-terminal` package. This function is meant to be returned in the `fetchCommand` config property of the `WasmTerminal` Class. This takes in the name of command, the command arguments, and the environment variables, and returns a `Promise` that resolves a `Uint8Array` of binary WASM bytes from WAPM.

