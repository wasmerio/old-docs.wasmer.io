---
id: wapm-manifest
title: WAPM Manifefst
sidebar_label: Manifest - wapm.toml
---

# Manifest

The manifest \(`wapm.toml`\) is optional for using the `wapm` CLI tool; it manages package dependencies, metadata and commands.

However, a manifest is required [to publish](publishing-your-package.md) to the [WAPM registry](https://wapm.io).

{% hint style="success" %}
If you want to get assistance creating the `wapm.toml` file you can simply run `wapm init` in your directory and the CLI will guide you
{% endhint %}

The manifest file contains 4 sections:

## \[package\]

Valid keys are:

* `name` \(string\) **required**
* `version` \([semver version](https://semver.org/)\) **required**: a valid Semantic Versioning version.
* `description` \(string\) **required**
* `license` \([spdx identifier](https://spdx.org/licenses/)\): can be MIT or GPL, for example.
* `license-file` \(path\): an override for the license file path used in publishing. Left undefined, the `LICENSE` file will be implicitly included in the package.
* `readme` \(path\)
* `repository` \(url\)
* `homepage` \(url\)
* `wasmer-extra-flags` \(string\): extra arguments to pass via `wapm run`, for example: `"--backend=singlepass"`\)

## \[dependencies\]

* `<namespace>/<name>" = "<version>`

## \[\[module\]\]

* `name` \(string\) **required**
* `source` \(path\) **required**: path to the `.wasm` file
* `abi` \(enum\): one of: `wasi`, `emscripten`, or `none`

## \[\[command\]\]

* `name` **required** \(string\): the name of the command, invoked via `wapm run <command-name>`
* `module` **required** \(string\): the name of the module this command is running.
* `package` **required** \(string\): the package name that the module is in.
* `main_args`
* `package`

## \[fs\]

* `"location/on/wasm"="location/on/host"`: a mapping between paths

