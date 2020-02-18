# Testing

Thanks to [spec tests](https://github.com/wasmerio/wasmer/tree/master/lib/spectests/spectests) we can ensure 100% compatibility with the WebAssembly spec test suite.

You can run all the tests with:

```text
make test
```

{% hint style="info" %}
`make test` will require all the backends to be installed. Please follow the Building from Source guide to make sure you have [all the requirements fulfilled](./#all-backends-default).
{% endhint %}

### Testing backends

Each backend can be tested separately:

* Singlepass: `make singlepass`
* Cranelift: `make cranelift`
* LLVM: `make llvm`

### Testing integrations

Each integration can be tested separately:

* Spec tests: `make spectests`
* Emscripten: `make emtests`
* WASI: `make wasitests`
* Middleware: `make middleware`
* C API: `make capi`



