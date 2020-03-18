# Compile Swift to Wasm WASI

Thanks to the great work of [Swift WASM](https://swiftwasm.org/) we can now compile Swift to WebAssembly WASI.

Wasienv makes very easy to get started with Swift and WebAssembly.

```text
wasienv install-swift
```

{% hint style="warning" %}
If you don't have wasienv installed in your system, please [follow the instructions here](getting-started.md)
{% endhint %}

Once that's done, just create an example Swift file:

{% tabs %}
{% tab title="example.swift" %}
```swift
if CommandLine.arguments.count < 2 {
    print("Hello, WASI!");
} else {
    print("Hello, \(CommandLine.arguments[1])");
}
```
{% endtab %}

{% tab title="fizzbuzz.swift" %}
```swift
for i in 1...100
{
    if i % 3 == 0 && i % 5 == 0 {
        print("FizzBuzz")
    } else if i % 3 == 0 {
        print("Fizz")
    } else if i % 5 == 0 {
        print("Buzz")
    } else {
        print(i)
    }
}
```
{% endtab %}
{% endtabs %}

Now let's Compile it!

```text
wasiswiftc example.swift -o example.wasm
```

Once the program finishes, you will have a new `example.wasm` file ready to be executed with your favorite WebAssembly runtime!

```bash
$ wasmer example.wasm
Hello, WASI!
```



