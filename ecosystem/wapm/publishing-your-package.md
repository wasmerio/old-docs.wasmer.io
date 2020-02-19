# Publishing your Package

Let's say you have a WebAssembly application, and you would like to publish it to WAPM so more people can use it easily.

You might have a similar project structure:

```text
/
  my_program_wasi.wasm
  my_other_program_wasi.wasm
  README.md
  LICENSE
```

Now, in the root of your project, you can simply run

```text
wapm init
```

And it will guide you the process of creating a new `wapm.toml` file for your project. At the end, it should have generated a manifest like the following:

```yaml
[package]
name = "username/my_package"
version = "0.1.0"
description = "The description of the package"

[[module]]
name = "my_program_wasi"
source = "my_program_wasi.wasm"

[[module]]
name = "my_other_program_wasi"
source = "my_other_program_wasi.wasm"
```

Now, you will need to:

1. [Create an account in WAPM](publishing-your-package.md#creating-an-account-in-wapm)
2. [Login in locally into your WAPM account](publishing-your-package.md#login-from-the-wapm-cli-into-your-local-account)
3. [Publish the package to WAPM](publishing-your-package.md#publish-the-package-to-wapm)

### Creating an account in WAPM

Creating an account in WAPM is very easy, just go to the following url and sign up!

{% embed url="https://wapm.io/signup" caption="" %}

### Login from the wapm CLI into your local account

Now that you are registered, you can just run the following command and it will ask for your credentials

```text
wapm login
```

### **Publish the package to WAPM**

Now that you have successfully created a `wapm.toml` manifest file for your project and have logged in into WAPM, you just need to run one more command!

```text
wapm publish
```

And you package should now be live in wapm.io 🎉

{% hint style="info" %}
All packages on [wapm.io](https://wapm.io/) are namespaced by username.
{% endhint %}

{% hint style="success" %}
If your program is a WASI program, it will be automatically available in the [WebAssembly shell](../webassembly.sh.md)
{% endhint %}

## **Commands**

Commands \(not to be confused with _wapm-cli_ subcommands\) are a feature that enables easily executing wasm code from a wapm package.

Commands are what allows one to call the `run` subcommand, like above when running `wapm run cowsay hello wapm!`.

A command requires a name and module to reference:

```text
[[command]]
name = "my_cmd"
module = "my_program_wasi"
```

Now called `wapm run my_cmd` will execute the module defined with the name `my_program_wasi`. Under the hood, `wapm` calls `wasmer`, the WebAssembly server runtime.

If you want to learn more about the manifest format, please jump into the next article!

