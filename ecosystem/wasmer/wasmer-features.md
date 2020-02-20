# Wasmer Features

Wasmer WebAssembly runtime provides multiple features for their users/developers:

* **Backends:** Wasmer supports multiple backends: _Singlepass_, _Cranelift_ and _LLVM_. Each of those with different tradeoffs for compilation and runtime speed.
* **Caching**: it allows to reuse the compiled WebAssembly code, so subsequent runs of a Wasm file will run as fast as possible
* **Metering**: it allows to track and limit the usage of functions within the host. This is also known as "gas metering"
* _WebAssembly Features_:
  * **Multi-value return**: it's a proposal that allows function to return more than one value at a time
  * **SIMD**: Single Instruction, Multiple data
* ABIs: it allows to run different programs compiled to WebAssembly, with ABIs such as:
  * **Emscripten**
  * **WASI**

### Support of features by Backend

|  | Singlepass | Cranelift | LLVM |
| :--- | :--- | :--- | :--- |
| Caching | ✅ | ✅ | ✅ |
| Emscripten | ✅ | ✅ | ✅ |
| Metering | ✅ | ⬜ | ✅ |
| Multi-value return | 🔄 | 🔄 | 🔄 |
| SIMD | ⬜ | ⬜ | ✅ |
| WASI | ✅ | ✅ | ✅ |
| `WASMER_BACKTRACE` | ✅ | ⬜ | ⬜ |

{% hint style="info" %}
### Legend

* ✅ Supported
* 🔄 On the works
* ⬜ Not yet supported \(please ping us if you need this feature!\)
{% endhint %}

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



