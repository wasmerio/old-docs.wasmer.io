---
id: wapm-reference-api
title: WAPM Reference API
sidebar_label: Reference API
---

# CLI Commands

`wapm login` - Logs the user in to the registry with the given credentials.

`wapm logout` - Logs the user out of the registry, resetting the token.

`wapm whoami` - Shows the current user logged in.

`wapm config set <key> <value>` - Sets a config `key` with the given `value`.

*Note: when setting the `registry.url`, the `registry.token` will reset automatically.*

`wapm config get <key>` - Gets the config `key` contents.

`wapm search <query>` - Search for packages related to the `query`.

`wapm install [package]` - Installs missing dependencies and the latest version of the package, optionally, specified. Install globally with the `-g` flag.

`wapm run` - Executes a package command with the `run` cli command. The command will be run with the wasmer runtime.

`wapm validate <wapm_package_location>` - Validate the sources of local wapm modules. Will display an error if the sources are not valid WebAssembly.

`wapm completions <shell>` - Generate a shell completion script for wapm for the specified shell.

`wapm init` - Initialize a new wapm project by generating a `wapm.toml` in the current directory.

`wapm list` - Prints all commands and modules for a package in the current directory.

`wapm uninstall` - The opposite of `wapm install`. Uninstall globally with the `-g` flag.

`wapm bin` - Print the `.bin` directory path for the local package. Get the global path with `-g` flag.

`wapm keys` - key related sub commands

`wapm keys list [-a]` - list personal keypairs and trusted public keys

`wapm keys generate <path>` - generate a key pair at location `<path>` and register them with wapm

`wapm keys import <public-key-value> --user-name=<user-name>` - import a public key for the given user

`wapm keys register --public <public-key-location> --private <private-key-location>` - register an existing keypair with wapm

`wapm keys delete <public-key-id>` - delete the keypair matching `<public-key-id>`

# Manifest

The manifest is optional for using the wapm CLI tool; it manages package dependencies, metadata, and commands.

However, a manifest is required to publish to the wapm registry.

The manifest contains 4 sections:

## [package]

Valid keys are:

### **Required:**

- `name`
- `version`
- `description`

### **Optional:**

- `license` (name)
- `license-file` (path, An override for the license file path used in publishing. Left undefined, the `LICENSE` file will be implicitly included in the package.)
- `readme` (path)
- `repository` (url)
- `homepage` (url)
- `wasmer-extra-flags` (extra arguments to pass via `wapm run`, for example: `"--backend=singlepass"`)

## [dependencies]

- `<namespace>/<name>" = "<version>`

## [[module]]

- `name` (the name of the module)
- `source` (path to Wasm file)
- `abi` (one of: `wasi`, `emscripten`, or `none`)

## [[command]]

### **Required:**

- `name` (the name of the command, invoked via `wapm run <command-name>`)
- `module` (the name of the module this command is running)
- `package` (the package name that the module is in)

### **Optional:**

- `main_args`
- `package`

## [fs]

- `"location/on/guest"="location/on/host"` # a mapping between paths

# Telemetry

During the alpha, telemetry (specifically error logging to Sentry) is enabled by default. We send and record information such as IP address, operating system name and version, and the error/panic message. To disable it, run wapm config set telemetry.enabled false or compile from source (the telemetry feature is disabled in the build by default).

