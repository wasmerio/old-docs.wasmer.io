# Testing

Thanks to [spec tests](https://github.com/wasmerio/wasmer/tree/master/lib/spectests/spectests) we can ensure 100% compatibility with the WebAssembly spec test suite.

You can run all the tests with:

```text
make test
```

{% hint style="info" %}
`make test` will automatically detect the backends installed on your system.

Please follow the Building from Source guide see how you can[ prepare your system with the requirements needed for each of the backends](./#all-backends-default).
{% endhint %}

### Testing backends

Each backend can be tested separately:

* **Singlepass**: `make singlepass`
* **Cranelift**: `make cranelift`
* **LLVM**: `make llvm`

### Testing integrations

Each integration can be tested separately:

* Spec tests: `make spectests`
* WASI: `make wasitests`
* Emscripten: `make emtests`
* Middleware: `make middleware`
* C API: `make capi`



