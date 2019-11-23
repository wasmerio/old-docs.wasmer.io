---
id: runtime-rust-integration-installation
title: Runtime Rust Integration Installation
sidebar_label: Installation
---

The easiest way to install rust, is through [rustup](https://rustup.rs/).

To build install Rustup on Windows, download and run [rustup-init.exe](https://win.rustup.rs/) then follow the onscreen instructions.

To get rustup on other systems, run:

`curl [https://sh.rustup.rs](https://sh.rustup.rs/) -sSf | sh`

To ensure this is installed, let's run the following:

`rustc -v`

and

`cargo -v`

If these commands work, rust is successfully installed!

Next, let's take a look at building a simple hello world!
