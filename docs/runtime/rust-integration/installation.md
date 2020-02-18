---
id: runtime-rust-integration-installation
title: Rust Integration: Installation
sidebar_label: Rust Installation
---

# Prerequisites - Install Rust

If you have already installed the Wasmer CLI tool, then Rust will already be installed.  Otherwise, the easiest way to install Rust is using the [`rustup`](https://rustup.rs/) tool.

## Windows

To install `rustup` on Windows, download and run [rustup-init.exe](https://win.rustup.rs/) then follow the onscreen instructions.

## *NIX Operating Systems

To get rustup on *NIX operating systems, run:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

To check that both the Rust compiler and the Rust Package Manager ([Cargo](https://doc.rust-lang.org/cargo/)) have been installed, run the following commands.  You should see output similar to the following:

```bash
$ rustc -V
rustc 1.41.0 (5e1a79984 2020-01-27)
$ cargo -V
cargo 1.41.0 (626f0f40e 2019-12-03)
```


Next, let's take a look at building a simple hello world!
