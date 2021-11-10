---
id: wapm-cli
title: WAPM CLI
sidebar_label: CLI
---

# CLI Usage

The `wapm` Command Line tool has the following subcommands:

## `wapm login`

Logs the user in to the registry with the given credentials.

## `wapm logout`

Logs the user out of the registry, resetting the token.

## `wapm whoami`

Shows the current user logged in.

## `wapm config set <key> <value>`

Sets a config `key` with the given `value`.

_Note: when setting the `registry.url`, the `registry.token` will reset automatically._

## `wapm config get <key>`

Gets the config `key` contents.

## `wapm search <query>`

Search for packages related to the `query`.

## `wapm install <package>`

Installs missing dependencies and the latest version of the package, optionally, specified.

{% hint style="info" %}
A WAPM package can be installed globally by adding the `-g` flag

EG: `wapm install -g cowsay`
{% endhint %}

## `wapm run`

Executes a package command with the `run` cli command. The command will be run with the Wasmer runtime.

## `wapm validate <wapm_package_dir>`

Validate the sources of local WAPM modules. Will display an error if the sources are not valid WebAssembly.

## `wapm completions <shell>`

Generate a shell completion script for WAPM for the specified shell.

## `wapm init`

Initialize a new WAPM project by generating a `wapm.toml` in the current directory.

## `wapm list`

Prints all commands and modules for a package in the current directory.

## `wapm uninstall <package>`

The opposite of `wapm install`. Uninstall globally with the `-g` flag.

## `wapm bin`

Print the `.bin` directory path for the local package. Get the global path with `-g` flag.

## `wapm keys`

WAPM packages can be signed to ensure that they haven't been tampered with.

### `wapm keys list [-a]`

list personal key pairs and trusted public keys

### `wapm keys generate <path>`

generate a key pair at location `<path>` and register them with WAPM

### `wapm keys import <public-key-value> --user-name=<user-name>`

import a public key for the given user

### `wapm keys register --public <public-key-location> --private <private-key-location>`

register an existing key pair with WAPM

### `wapm keys delete <public-key-id>`

delete the key pair matching `<public-key-id>`

{% hint style="warning" %}
### Telemetry

During the alpha, telemetry (specifically error logging to Sentry) is enabled by default in the WAPM CLI. We send and record information such as IP address, operating system name and version, and the error/panic message.

To disable it, run `wapm config set telemetry.enabled false` or compile from source (the telemetry feature is disabled in the build by default).
{% endhint %}
