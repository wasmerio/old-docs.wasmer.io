# Compile C/C++ to Wasm WASI

Now that you have [Wasienv installed](getting-started.md), let's get our hands dirty!

Wasienv comes with C/C++ support preinstalled, so you just need to run one command to get things running 🙂

### Compiling C files with \`wasic\`

If you want to compile a simple C example, you can just create the following:

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

{% tab title="fizzbuzz.c" %}
```c
// CPP program to print Fizz Buzz 
#include <stdio.h> 
  
int main(void) 
{ 
    int i; 
    for (i=1; i<=100; i++) 
    { 
        // number divisible by 3 and 5 will 
        // always be divisible by 15, print  
        // 'FizzBuzz' in place of the number 
        if (i%15 == 0)         
            printf ("FizzBuzz\t");     
          
        // number divisible by 3? print 'Fizz' 
        // in place of the number 
        else if ((i%3) == 0)     
            printf("Fizz\t");                  
          
        // number divisible by 5, print 'Buzz'   
        // in place of the number 
        else if ((i%5) == 0)                        
            printf("Buzz\t");                  
      
        else // print the number             
            printf("%d\t", i);                  
  
    } 
  
    return 0; 
}
```
{% endtab %}
{% endtabs %}

Now that you have this file created, you can execute `wasicc`

```bash
wasicc example.c -o example
```

Et voilá... you will have a new file `example.wasm` ready to be executed with your favorite WebAssembly runtime!

```text
$ wasmer example.wasm
Hello, WASI!
```

{% hint style="success" %}
#### Did you know?

You can also execute the `example.wasm` file in your **browser** or in **Node.js** using [@wasmer/wasi](../../integrations/js/wasi/). Check out the examples on how to do it!

* [WASI in the browser](../../integrations/js/wasi/browser/examples/hello-world.md)
* [WASI in Node.js](../../integrations/js/wasi/server/examples/hello-world.md)
{% endhint %}

### Compiling C++ files with \`wasic++\`

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

{% hint style="info" %}
Here's an example of a C Project that was compiled to Wasm WASI with wasienv:

[https://github.com/wapm-packages/jq/blob/master/build.sh](https://github.com/wapm-packages/jq/blob/master/build.sh)
{% endhint %}

## Using CMake

Similarly to configure and make, we also have another command that will automatically compile all your projects made with CMake

```text
wasicmake cmake .
make
```

{% hint style="info" %}
Here's an [example project: quickjs](https://github.com/saghul/wasi-lab/tree/master/qjs-wasi) that was compiled to WebAssembly WASI thanks to Wasienv!

And the [build.sh file they used](https://github.com/saghul/wasi-lab/blob/master/qjs-wasi/build.sh) for generating the wasm file:

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
{% endhint %}

