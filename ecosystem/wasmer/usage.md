---
id: runtime-cli-usage
title: Runtime Command Line Interface Usage
sidebar_label: Usage
---

# CLI Usage

This is the help output from the Wasmer CLI for `wasmer --help` :

```text
USAGE:
    wasmer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cache          Wasmer cache
    help           Prints this message or the help of the given subcommand(s)
    run            Run a WebAssembly file. Formats accepted: wasm, wast
    self-update    Update wasmer to the latest version
    validate       Validate a Web Assembly binary
```

### `wasmer cache`

```text
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

### `wasmer run`

It runs a WebAssembly file. Formats accepted: wasm, wat

```text
USAGE:
    wasmer run [FLAGS] [OPTIONS] <path> [--] []...

FLAGS:
        --enable-all        Enable support for all pre-standard proposals.
        --disable-cache     Disable the cache
    -h, --help              Prints help information
        --enable-simd       Enable support for the SIMD proposal.
        --enable-threads    Enable support for the threads proposal.
        --track-state       Whether or not state tracking should be disabled during compilation. State tracking is
                            necessary for tier switching and backtracing.
    -V, --version           Prints version information

OPTIONS:
        --backend <backend>                   [default: cranelift]  [possible values: cranelift, singlepass, llvm]
        --em-entrypoint <em-entrypoint>      Begin execution at the specified symbol
        --em-symbol-map <em-symbol-map>      Emscripten symbol map
        --env <env-vars>...                  Pass custom environment variables
        --loader <loader>                    Custom code loader [possible values: local, kernel]
        --mapdir <mapped-dirs>...            Map a host directory to a different location for the wasm module
        --llvm-object-file <obj-file>        Emit LLVM generated native code object file.
        --llvm-post-opt-ir <post-opt-ir>     Emit LLVM IR after optimization pipeline.
        --dir <pre-opened-directories>...    WASI pre-opened directory
        --llvm-pre-opt-ir <pre-opt-ir>       Emit LLVM IR before optimization pipeline.

ARGS:
    <path>    Input file
    <>...     Application arguments
```

### `wasmer self-update`

It auto-updates Wasmer to the most-recent published version.

### `wasmer validate <path>`

It validates that the provided Wasm file is valid.

```text
USAGE:
    wasmer validate [FLAGS] <path>

FLAGS:
        --enable-all        Enable support for all pre-standard proposals.
    -h, --help              Prints help information
        --enable-simd       Enable support for the SIMD proposal.
        --enable-threads    Enable support for the threads proposal.
    -V, --version           Prints version information

ARGS:
    <path>    Input file
```

