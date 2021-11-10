---
description: "The WebAssembly package Runner \U0001F3C3\U0001F3FD‍♂️"
---

# wax

wax allows running any command available in wapm, directly in your shell, without global installations that pollute your directory.

{% hint style="success" %}
wax is automatically available when you [install Wasmer](../wasmer/getting-started.md).
{% endhint %}

wax is a tool intended to ease the use of command-line WebAssembly applications on your system. Similarly to `wapm`, that allows the installation and usage of packages and commands, wax enables use of CLI tools without installing them globally or changing your `PATH`.

![](../../.gitbook/assets/wax-gif1.gif)

You can read more details about wax in our media announcement!

{% embed url="https://medium.com/wasmer/introducing-wax-a-webassembly-package-runner-d69943209d58" caption="" %}

## Technical details

To improve the developer experience, wax will automatically preopen the current directory by default \(passing the `--dir=.` argument to the corresponding WASI runtime\).

Signed packages are automatically trusted \(if seen for the first time\).

Packages are installed in a temporary folder in your system, so it doesn't pollute your environment

