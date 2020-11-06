---
id: runtime-cli-usage
title: Runtime Command Line Interface Usage
sidebar_label: Usage
---

# CLI Usage

This is the help output from the Wasmer CLI for `wasmer --help` :

```text
WebAssembly standalone runtime.

USAGE:
    wasmer <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cache          Wasmer cache
    compile        Compile a WebAssembly binary
    config         Get various configuration information needed to compile programs which use Wasmer
    create-exe     Compile a WebAssembly binary into a native executable
    help           Prints this message or the help of the given subcommand(s)
    inspect        Inspect a WebAssembly file
    run            Run a WebAssembly file. Formats accepted: wasm, wat
    self-update    Update wasmer to the latest version
    validate       Validate a WebAssembly binary
    wast           Run spec testsuite
```

## `wasmer cache`

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

## `wasmer compile`

Compile a WebAssembly binary

```text
USAGE:
    wasmer compile [FLAGS] [OPTIONS] <FILE> -o <OUTPUT PATH>

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --enable-verifier           Enable compiler internal verification
    -h, --help                      Prints help information
        --jit                       Use JIT Engine
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --native                    Use Native Engine
        --object-file               Use ObjectFile Engine
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
        --header <HEADER PATH>               Output path for generated header file
    -o <OUTPUT PATH>                         Output file
    -m <cpu-features>...                     
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to
        --target <target-triple>             Compilation Target triple

ARGS:
    <FILE>    Input file
```

## `wasmer config`

Get various configuration information needed to compile programs which use Wasmer

```text
USAGE:
    wasmer config [FLAGS]

FLAGS:
        --bindir        Directory containing Wasmer executables
        --cflags        C compiler flags for files that include Wasmer headers
    -h, --help          Prints help information
        --includedir    Directory containing Wasmer headers
        --libdir        Directory containing Wasmer libraries
        --libs          Libraries needed to link against Wasmer components
        --pkg-config    It outputs the necessary details for compiling and linking a program to Wasmer, using the `pkg-
                        config` format
        --prefix        Print the installation prefix
    -V, --version       Prints version information
```

## `wasmer create-exe`

Compile a WebAssembly binary into a native executable

```text
USAGE:
    wasmer create-exe [FLAGS] [OPTIONS] <FILE> -o <OUTPUT PATH>

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --enable-verifier           Enable compiler internal verification
    -h, --help                      Prints help information
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
    -o <OUTPUT PATH>                         Output file
    -m <cpu-features>...                     
    -l <libraries>...                        Additional libraries to link against. This is useful for fixing linker
                                             errors that may occur on some systems
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to
        --target <target-triple>             Compilation Target triple

ARGS:
    <FILE>    Input file
```

## `wasmer inspect`

Inspect a WebAssembly file

```text
USAGE:
    wasmer inspect [FLAGS] [OPTIONS] <FILE>

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --enable-verifier           Enable compiler internal verification
    -h, --help                      Prints help information
        --jit                       Use JIT Engine
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --native                    Use Native Engine
        --object-file               Use ObjectFile Engine
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to

ARGS:
    <FILE>    File to validate as WebAssembly
```

## `wasmer run`

Run a WebAssembly file. Formats accepted: wasm, wat

```text
USAGE:
    wasmer run [FLAGS] [OPTIONS] <FILE> [--] [--]...

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --disable-cache             Disable the cache
        --enable-verifier           Enable compiler internal verification
    -h, --help                      Prints help information
        --jit                       Use JIT Engine
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --native                    Use Native Engine
        --object-file               Use ObjectFile Engine
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
        --dir <DIR>...                       WASI pre-opened directory
        --mapdir <GUEST_DIR:HOST_DIR>...     Map a host directory to a different location for the wasm module
        --env <KEY=VALUE>...                 Pass custom environment variables
    -i, --invoke <invoke>                    Invoke a specified function
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to

ARGS:
    <FILE>     File to run
    <-->...    Application arguments
```

## `wasmer self-update`

It auto-updates Wasmer to the most-recent published version.

## `wasmer validate <path>`

Validate a WebAssembly binary

```text
USAGE:
    wasmer validate [FLAGS] [OPTIONS] <FILE>

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --enable-verifier           Enable compiler internal verification
    -h, --help                      Prints help information
        --jit                       Use JIT Engine
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --native                    Use Native Engine
        --object-file               Use ObjectFile Engine
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to

ARGS:
    <FILE>    File to validate as WebAssembly
```

## `wamser wast`

Run spec testsuite

```text
USAGE:
    wasmer wast [FLAGS] [OPTIONS] <FILE>

FLAGS:
        --enable-all                Enable support for all pre-standard proposals
        --enable-bulk-memory        Enable support for the bulk memory proposal
        --cranelift                 Use Cranelift compiler
        --enable-verifier           Enable compiler internal verification
    -f, --fail-fast                 A flag to indicate wast stop at the first error or continue
    -h, --help                      Prints help information
        --jit                       Use JIT Engine
        --llvm                      Use LLVM compiler
        --enable-multi-value        Enable support for the multi value proposal
        --native                    Use Native Engine
        --object-file               Use ObjectFile Engine
        --enable-reference-types    Enable support for the reference types proposal
        --enable-simd               Enable support for the SIMD proposal
        --singlepass                Use Singlepass compiler
        --enable-threads            Enable support for the threads proposal
    -V, --version                   Prints version information

OPTIONS:
        --llvm-debug-dir <llvm-debug-dir>    LLVM debug directory, where IR and object files will be written to

ARGS:
    <FILE>    Wast file to run
```

