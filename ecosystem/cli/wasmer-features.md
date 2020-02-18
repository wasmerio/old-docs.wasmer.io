# Wasmer Features

Wasmer runtime has three different backends: Singlepass, Cranelift and LLVM. Each of those with a different tradeoff between compilation and runtime speed.

Also, Wasmer provides multiple features for their users:

* Caching: it allows to reuse the compiled WebAssembly code, so subsequent runs of a Wasm file will super fast
* Metering: it allows to track and limit the usage of functions within the host. This is also known as "gas metering"
* WebAssembly Features:
  * Multi-value return: it's a proposal that allows function to return more than one value at a time
  * SIMD: Single Instruction, Multiple data
* ABIs: it allows to run different programs compiled to WebAssembly, with ABIs such as:
  * Emscripten
  * WASI

### Support of features by Backend

|  | Singlepass | Cranelift | LLVM |
| :--- | :--- | :--- | :--- |
| Caching | ✅ | ✅ | ✅ |
| Emscripten | ✅ | ✅ | ✅ |
| Metering | ✅ | ⬜ | ✅ |
| Multi-value return | ⬜ | ⬜ | ⬜ |
| SIMD | ⬜ | ⬜ | ✅ |
| WASI | ✅ | ✅ | ✅ |
| `WASMER_BACKTRACE` | ✅ | ⬜ | ⬜ |

### Support by Operating System

|  | GNU Linux | Mac OSX | Windows NT |
| :--- | :--- | :--- | :--- |
| Cranelift Backend | ✅ | ✅ | ✅ |
| LLVM Backend | ✅ | ✅ | ✅ |
| Singlepass Backend | ✅ | ✅ | [\#347](https://github.com/wasmerio/wasmer/issues/347) |
| WASI | ✅ | ✅ | ✅\* |

* `poll_fd` is not fully implemented for Windows yet

### Language integration

TODO: define a set of features that are relevant and mark them here

Current ideas:

* Callbacks
* Metering
* Caching

> TODO: expand this table, it's focused on new features that we haven't implemented yet and doesn't list all language integrations

|  | Rust | C / C++ | Go | Python | Ruby |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Terminate in host call | ✅ | ⬜ | ⬜ | ⬜ | ⬜ |
| WASI | ✅ | ✅ | 🔄 | ⬜ | ⬜ |
| WASI FS API | ✅ | ⬜ | ⬜ | ⬜ | ⬜ |



