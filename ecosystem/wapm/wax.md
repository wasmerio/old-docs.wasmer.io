# wax

wax is a tool that allows running any command available in wapm, directy in your shell, without global installations that pollute your directory.

It will be automatically available when you [install Wasmer](../wasmer/getting-started.md).

![](../../.gitbook/assets/wax-gif1.gif)



### Technical details

To improve the developer experience, wax will automatically preopen the current directory by default \(passing the `--dir=.` argument to the corresponding WASI runtime\).

Signed packages are automatically trusted \(if seen for the first time\).

Packages are installed in a temporary folder in your system, so it doesn't pollute your environment



