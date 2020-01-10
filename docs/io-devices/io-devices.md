---
id: io-devices
title: IO Devices Introduction
sidebar_label: IO Devices Introduction
---

I/O Devices is feature of the [Wasmer Runtime](/runtime/runtime) and [Wasmer-JS](/wasmer-js/wasmer-js) that allows for building applications using the WASI API to display graphics, handle input, etc...

I/O devices works by creating virtual devices files in the runtime filesystem, similar to device file on a UNIX operating system. If a Wasm module uses the WASI bindings of reading/writing files, the runtime can react to the events and perform actions for the Wasm module.

There is an experimental Wasmer `io-devices-lib` for [Rust](/io-devices/io-devices-lib/io-devices-lib-rust) and [AssemblyScript](/io-devices/io-devices-lib/io-devices-lib-assemblyscript) for building WASI applications using I/O Devices. However, any language/toolchain that supports WASI output should be able to use I/O Devices as it only requires interfacing with the filesystem. To implement this yourself, please see below:

For graphics, I/O Devices defines a `/sys/class/graphics/wasmerfb0/virtual_size`, similar to `/sys/class/graphics/fb0` directory on a linux system. Reading from this file would return the current window size. Initially, this would be "0x0". Writing a value larger than "0x0" would open / resize the window, and writing the value "0x0" would close the window. 

I/O devices two additional files for graphics, `/dev/wasmerfb0` and `/sys/class/graphics/wasmerfb0/buffer_index_display`. `/dev/wasmerfb0` is a file that should contain a RGBA byte array of a buffer to be drawn. This is similar to writing directly to a linux framebuffer. `/sys/class/graphics/wasmerfb0/buffer_index_display` is a file that when written to, will draw the RGBA byte array in the `/dev/wasmerfb0` file, to the open framebuffer window!

For input, I/O Devices defines a `/dev/input` file. This file is filled by the runtime with bytes that represent the event type, and event value. For example, key down events are defined with a byte value of 1, followed by a byte defining the code of the key. This file is cleared whenever it is read by the Wasm module, and the Wasm module can use this information to respond to input events. You can see an implementatio of this input map, in the [io-devices-lib AssemblyScript implementation](https://github.com/wasmerio/io-devices-lib/blob/master/assemblyscript/lib/input-map.ts)

## Additional Resources

[Announcement Article]()

