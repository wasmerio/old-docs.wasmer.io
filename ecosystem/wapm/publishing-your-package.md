# Publishing your Package

Let's say you have a WebAssembly application, and you would like to publish it to WAPM so more people can use it easily.

Let's say you have the following dir structure:

```text
/
  module.wasm
  
```



## **Package with `wapm.toml`**

The manifest file enables one to publish a package to the [wapm.io](https://wapm.io/) public registry.

Login is required for publishing. Signup for an account at [wapm.io](https://wapm.io/) and login to the server with `login` command.

The reference shows all the required fields for the manifest, but it's easy to get started with `init` command.

`wapm init`

This command generates a wapm manifest like the following:

```yaml
[package]
name = "username/my_package"
version = "0.1.0"
description = ""
```

All packages on [wapm.io](https://wapm.io/) are namespaced by username. This is the minimum required data for a manifest file. A module is required for publishing. Add a module section:

```text
[[module]]
name = "my_module"
source = "path/to/my_module.wasm"
```

Publish the project to [wapm.io](https://wapm.io/)!

`wapm publish`

## **Commands**

Commands \(not to be confused with _wapm-cli_ subcommands\) are a feature that enables easily executing wasm code from a wapm package.

Commands are what allows one to call the `run` subcommand, like above when running `wapm run cowsay hello wapm!`.

A command requires a name and module to reference:

```text
[[command]]
name = "my_cmd"
module = "my_module"
```

Now called `wapm run my_cmd` will execute the module defined with the name `my_module`. Under the hood, _wapm-cli_ calls _wasmer_, the WebAssembly server runtime.

