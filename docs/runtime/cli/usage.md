---
id: runtime-cli-usage
title: Runtime Command Line Interface Usage
sidebar_label: CLI Usage
---

This is the help output from the Wasmer CLI:

```bash
$ wasmer --help
wasmer 0.13.1
The Wasmer Engineering Team <engineering@wasmer.io>
Wasm execution runtime.

USAGE:
    wasmer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cache          Wasmer cache
    help           Prints this message or the help of the given subcommand(s)
    run            Run a WebAssembly file. Formats accepted: wasm, wat
    self-update    Update wasmer to the latest version
    validate       Validate a Web Assembly binary
```

## `wasmer cache`

By default, Wasmer will always run the copy of a WASM module from its cache if present.  You may, from time to time, need to clean this cache...

```bash
$ wasmer cache --help
wasmer-cache 0.13.1
Wasmer cache

USAGE:
    wasmer cache <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clean    Clear the cache
    dir      Display the location of the cache
    help     Prints this message or the help of the given subcommand(s)
```

## `wasmer run`

The `run` command is assumed if no other command is provided.

WebAssembly modules can be executed either in their binary from `.wasm` files, or as WebAssembly Text from `.wat` files

```bash
$ wasmer run --help
wasmer-run 0.13.1
Run a WebAssembly file. Formats accepted: wasm, wat

USAGE:
    wasmer run [FLAGS] [OPTIONS] <path> [--] [--]...

FLAGS:
        --enable-all                        Enable support for all pre-standard proposals
        --block-trace                       
        --call-trace                        
        --disable-cache                     Disable the cache
        --enable-experimental-io-devices    Enable non-standard experimental IO devices
    -h, --help                              Prints help information
        --enable-simd                       Enable support for the SIMD proposal
        --enable-threads                    Enable support for the threads proposal
        --track-state                       Whether or not state tracking should be disabled during compilation. State
                                            tracking is necessary for tier switching and backtracing
    -V, --version                           Prints version information

OPTIONS:
        --backend <backend>                  Name of the backend to use (x86_64) [default: auto]  [possible values:
                                             singlepass, cranelift, llvm, auto]
        --em-entrypoint <em-entrypoint>      Begin execution at the specified symbol
        --em-symbol-map <em-symbol-map>      Emscripten symbol map
        --env <env-vars>...                  Pass custom environment variables
    -i, --invoke <invoke>                    Invoke a specified function
        --loader <loader>                    Custom code loader [possible values: local, kernel]
        --mapdir <mapped-dirs>...            Map a host directory to a different location for the wasm module
        --llvm-object-file <obj-file>        Emit LLVM generated native code object file
        --llvm-post-opt-ir <post-opt-ir>     Emit LLVM IR after optimization pipeline
        --dir <pre-opened-directories>...    WASI pre-opened directory
        --llvm-pre-opt-ir <pre-opt-ir>       Emit LLVM IR before optimization pipeline

ARGS:
    <path>     Input file
    <-->...    Application arguments
```

## `wasmer self-update`

Updates Wasmer to the latest version.

```bash
$ wasmer self-update 
Fetching latest installer
Updating Wasmer and WAPM
> Getting wasmer releases... ✓
You are already on the latest release of wasmer: 0.13.1
```

## `wasmer validate <path>`

Checks that the provided WASM file is valid.

``` 
$ wasmer validate --help
wasmer-validate 0.13.1
Validate a Web Assembly binary

USAGE:
    wasmer validate [FLAGS] <path>

FLAGS:
        --enable-all        Enable support for all pre-standard proposals
    -h, --help              Prints help information
        --enable-simd       Enable support for the SIMD proposal
        --enable-threads    Enable support for the threads proposal
    -V, --version           Prints version information

ARGS:
    <path>    Input file
```
