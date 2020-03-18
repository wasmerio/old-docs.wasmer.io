# Compile C/C++ to Wasm WASI

Now that you have [Wasienv installed](getting-started.md), it should be really easy to start using it!

### Compiling C files with wasic

If you want to compile a very simple C file, you can just create the following:

{% tabs %}
{% tab title="example.c" %}
```c
#include <stdio.h>

int main(int argc, char **argv)
{
    if (argc < 2) {
        printf("Hello, WASI!\n");
    } else {
        printf("Hello, %s!\n", argv[1]);
    }
}
```
{% endtab %}
{% endtabs %}

Now that you have this file created, you can execute `wasicc`

```bash
wasicc example.c -o example
```

And voilá... you will have a new file `example.wasm` ready to be executed with your favorite WebAssembly runtime!

```text
$ wasmer example.wasm
Hello, WASI!
```

### Compiling C++ files with wasic++

Wasienv also allows you to compile C++ files to Wasm WASI, just run it with `wasic++`

## Using Configure and Make

In some projects, there might be already `./configure` files that will configure the system \(creating a Makefile normally\) to compile a certain project.

If you want to use Wasienv there, you just need to wrap your `./configure` call with `wasiconfigure`!

```text
wasiconfigure ./configure
```

Wasienv also has a wrapper for Makefiles

```text
wasimake make
```

## Using CMake

Similarly to configure and make, we also have another command that will automatically compile all your projects made with CMake

```text
wasicmake cmake .
make
```

Here's an example project: quickjs that was compiled to WebAssembly WASI thanks to Wasienv!

{% embed url="https://github.com/saghul/wasi-lab/tree/master/qjs-wasi" %}

And the [build.sh file they used](https://github.com/saghul/wasi-lab/blob/master/qjs-wasi/build.sh) for generating the wasm file:

{% tabs %}
{% tab title="build.sh" %}
```bash
#!/bin/bash

# Install wasienv
curl https://raw.githubusercontent.com/wasienv/wasienv/master/install.sh | sh

mkdir -p build
cd build
wasimake cmake ..
cd ..
make -C build
```
{% endtab %}
{% endtabs %}

