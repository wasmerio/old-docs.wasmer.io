---
id: runtime-rust-integration-examples-handling-errors
title: Rust Integration: Handling Errors
sidebar_label: Handling Errors
---

## Strange as it may sound...

There will come a time where running a WebAssembly module will not work, and trying to figure out why it does not work can be a difficult task!

The ability to debug WebAssembly programs is continually changing.  Most recently, Mozilla has proposed the use of LLDB to debug WASM modules running outside of the browser (See Dan Callahan's blog post and video on this topic [here](https://hacks.mozilla.org/2019/09/debugging-webassembly-outside-of-the-browser/))

In this example, we will load a WebAssembly module that purposely calls `panic!()` in its exported function call. The host runtime environment (our Rust application) will pattern match for the error and output the error message returned from Wasmer.

We will then remove this pattern matching code in order to see what happens when Rust itself crashes due to an unhandled error originating in a non-Rust program.  This will require a brief venture into using the low-level debugger.

> ***WARNING***
>
> We make no attempt here to teach you the intricacies of system-level debugging!
>
> If you're not familiar with this aspect of debugging, then you should treat the final set of instructions below as if you're a Medieval sailor venturing a little too close to the edges of your map: ***Here Be Dragons!***

## Development Steps

1. ***Create a New Rust Project***  
    Create a new Rust project called `handling-errors` and change into the newly created directory.

    ```bash
    $ cargo new handling-errors
    $ cd handling-errors
    ```

1. ***Prepare the Guest WASM Module***  
    Follow the steps for [preparing WASM modules](./runtime-rust-integration-prepare-wasm-modules) in order to create the required `handling-errors-guest` WASM module that will be called below.

1. ***Add a Dependency for the Wasmer Runtime***  
    Insert the following line into the `[dependencies]` section of the `Cargo.toml` file:

    `wasmer-runtime = "0.13.1"`

1. ***Write the Rust Code to Invoke the WASM Module***  
    Now that the Rust compiler has been informed of our dependency on the Wasmer runtime functionality, we can write some Rust code that calls the WASM module.

    To do this, we need to modify our `src/main.rs` to the following ([handling_error.rs](https://github.com/wasmerio/docs.wasmer.io/tree/master/docs/runtime/rust-integration/examples/handling_errors.rs))

    ```rust
    // Import the Filesystem so we can read our .wasm file
    use std::fs::File;
    use std::io::prelude::*;
    
    // Import the wasmer runtime so we can use it
    use wasmer_runtime::{error, imports, instantiate, Func, error::{RuntimeError}};
    
    const WASM_FILE_PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/wasm32-unknown-unknown/release/handling_errors_guest.wasm"
    );
    
    // Our entry point to our application
    fn main() -> error::Result<()> {
        // Let's read in our .wasm file as bytes
    
        // Let's open the file.
        let mut file = File::open(WASM_FILE_PATH).expect(&format!("WASM file at {}", WASM_FILE_PATH));
    
        // Let's read the file into a Vec
        let mut wasm_vec = Vec::new();
        file.read_to_end(&mut wasm_vec)
            .expect("Error reading the WASM file");
    
        // Now that we have the WASM file as bytes, let's run it with the wasmer runtime
    
        // Our import object, that allows exposing functions to our WASM module.
        // We're not importing anything, so make an empty import object.
        let import_object = imports! {};
    
        // Let's create an instance of WASM module running in the wasmer-runtime
        let instance = instantiate(&wasm_vec, &import_object)?;
    
        // Let's call the exported "throw_error" function in the WASM module.
        let throw_error_func: Func<(), ()> = instance
            .func("throw_wasm_error")
            .expect("throw_wasm_error function was not found");
    
        let response = throw_error_func.call();
    
        match response {
           Ok(_) => {
                // This should have thrown an error, return an error
                panic!("throw_wasm_error did not error");
           },
            Err(RuntimeError::Trap { msg }) => {
               // Log the error
               println!("Trap caught from `throw_wasm_error`: {}", msg);
           },
            Err(RuntimeError::Error { .. }) => {
                panic!("Expected Trap, found Error with unknown data!");
            },
        }
    
        // Log a success message.
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
    }
    ```

1. ***Execute the Rust Host Application***  
    The Rust host application can be compiled and executed using the command `cargo run`.
    
    ```bash
    $ cargo run
       Compiling semver-parser v0.7.0
       Compiling cfg-if v0.1.10
    # Snip lots of library compilation messages...
       Compiling wasmer-runtime v0.13.1
       Compiling handling-errors v0.1.0
        Finished dev [unoptimized + debuginfo] target(s) in 36.00s
         Running `target/debug/handling-errors`
    Trap caught from `throw_wasm_error`: unknown
    Success!
    ```

## When Rust Does Not Handle The Error...

1. ***Passing Errors Back to the Calling Program***  
    A common occurrence during development is to pass errors back to the calling function (such as `main`) or to `unwrap` them.

    In this case, we will remove the Rust coding that pattern matches on the error thrown by the WASM module.  Instead, will will simply call the WASM function and hope for the best... and then see what happens when it fails!

    Modify the above coding as follows:
    
    1. Delete the entire `match` expression that follows the call to `throw_error_func.call()`
    1. Change the name of the `response` variable to `_response` to indicate that we don't care about this value
    1. Add a question mark character after the call to `throw_error_func.call()`.  
        This indicates to the compiler that we're aware of the fact that potentially, this function call will generate an error; but what the heck - we'll take the risk!

    The last lines of the program now look like this:

    ```rust
    // Snip...
        let _response = throw_error_func.call()?;
    
        // Log a success message.
        println!("Success!");
    
        // Return OK since everything executed successfully!
        Ok(())
    }
    ```

    Now rerun the code with `cargo run` (and ignore the compiler warning about the unused import)

    ```bash
    $ cargo run
       Compiling handling-errors v0.1.0
    warning: unused import: `RuntimeError`
     --> src/main.rs:6:65
      |
    6 | use wasmer_runtime::{error, imports, instantiate, Func, error::{RuntimeError}};
      |                                                                 ^^^^^^^^^^^^
      |
      = note: `#[warn(unused_imports)]` on by default
    
        Finished dev [unoptimized + debuginfo] target(s) in 1.99s
         Running `target/debug/handling-errors`
    Error: RuntimeError(WebAssembly trap occurred during runtime: unknown)
    ```

    As you can see, whilst this error message is completely correct, it is also completely unhelpful because it gives no information about where the error occured.  It may as well read:

    ```bash
    An error has occured.  Good luck...
    ```

1. ***So Where Did It All Go Wrong?***  
    We now need to discover where our program went wrong.  In this particular case, the error originates in some non-Rust code we have called.

    > ***IMPORTANT***
    >
    > The environment variable `RUST_BACKTRACE` is useful for discovering errors that originate in Rust.
    >
    > Had this particular error been caused by an error in the Rust coding, then we could have set the environment variable `RUST_BACKTRACE` to `1`, then rerun our program as follows:
    >
    > ```bash
    > $ RUST_BACKTRACE=1 cargo run
    > ```
    > 
    > However, due to the fact that this error does not originate in Rust, setting `RUST_BACKTRACE`to `1` has no effect

    Hmmm, what to do...

1. ***Rust and `LLDB`***  
    Let's rerun the program, but this time, we'll use the Rust Low-Level Debugger.

    ```bash
    $ rust-lldb ./target/debug/handling-errors
    (lldb) command script import "/Users/chris/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/etc/lldb_rust_formatters.py"
    (lldb) type summary add --no-value --python-function lldb_rust_formatters.print_val -x ".*" --category Rust
    (lldb) type category enable Rust
    (lldb) target create "./target/debug/handling-errors"
    Current executable set to './target/debug/handling-errors' (x86_64).
    (lldb) 
    ```

    The command prompt now belongs to the low-level debugger enabled for Rust.
    
    As soon as as you enter the command `process launch`, the Rust program will execute and crash immediately, dumping detailed stack frame information to the screen:

    ```lldb
    (llbd) process launch
    Process 3147 launched: '/Users/chris/Developer/Rust/handling-errors/target/debug/handling-errors' (x86_64)
    Process 3147 stopped
    * thread #1, queue = 'com.apple.main-thread', stop reason = EXC_BAD_INSTRUCTION (code=EXC_I386_INVOP, subcode=0x0)
        frame #0: 0x0000000100ffab64
    ->  0x100ffab64: ud2    
        0x100ffab66: int3   
        0x100ffab67: int3   
        0x100ffab68: pushq  %rbp
    Target 0: (handling-errors) stopped.
    (lldb) thread backtrace
    error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
    * thread #1, queue = 'com.apple.main-thread', stop reason = EXC_BAD_INSTRUCTION (code=EXC_I386_INVOP, subcode=0x0)
      * frame #0: 0x0000000100ffab64
        frame #1: 0x0000000100ffab5e
        frame #2: 0x0000000100ffab06
        frame #3: 0x0000000100ffa069
        frame #4: 0x0000000100ffa2c5
        frame #5: 0x0000000100fff006
        frame #6: 0x00000001000dfa77 handling-errors`_$LT$wasmer_clif_backend..signal..Caller$u20$as$u20$wasmer_runtime_core..backend..RunnableModule$GT$::get_trampoline::invoke::_$u7b$$u7b$closure$u7d$$u7d$::h2911daca16035d50 at mod.rs:78:16
        frame #7: 0x0000000100059390 handling-errors`wasmer_clif_backend::signal::unix::call_protected::h2a62ce2879606caf(handler_data=&0x105a006e0, f=closure-0(&    0x7ffeefbfecd8, &0x7ffeefbfece0, &0x7ffeefbfece8, &0x7ffeefbfecf0, &0x7ffeefbfecf8)) at unix.rs:121:22
        frame #8: 0x00000001000df896 handling-errors`_$LT$wasmer_clif_backend..signal..Caller$u20$as$u20$wasmer_runtime_core..backend..RunnableModule$GT$::get_trampoline::invoke::h524ad612848e620c(trampoline=&0x100fff000, ctx=&0x105a00910, func=NonNull<wasmer_runtime_core::vm::Func> {
    pointer: &0x100ffa298
    }, args=&0x7ffeefbfee60, rets=&0x7ffeefbfee68, trap_info=&0x7ffeefbfee6c, user_error=&0x7ffeefbfee70, invoke_env=Option<core::ptr::non_null::NonNull<core::ffi::c_void>> {

    }) at mod.rs:76:22
        frame #9: 0x000000010048c4f1 handling-errors`_$LT$$LP$$RP$$u20$as$u20$wasmer_runtime_core..typed_func..WasmTypeList$GT$::call::hb32b50274dca566f(self=<unavailable>, f=NonNull<wasmer_runtime_core::vm::Func> {
    pointer: &0x100ffa298
    }, wasm=Wasm {
    trampoline: &0x100fff000, 
    invoke: &0x1000df7a0, 
    invoke_env: Option<core::ptr::non_null::NonNull<core::ffi::c_void>> {
    
    }
    }, ctx=&0x105a00910) at typed_func.rs:407:19
        frame #10: 0x000000010048ccb7 handling-errors`wasmer_runtime_core::typed_func::Func$LT$$LP$$RP$$C$Rets$GT$::call::h0f1f9765031cd5c6(self=&0x7ffeefbff360)     at typed_func.rs:665:20
        frame #11: 0x0000000100001c02 handling-errors`handling_errors::main::h737fb5e67c519ca8 at main.rs:39:20
        frame #12: 0x0000000100005369 handling-errors`std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc3969c762ae692d6 at rt.rs:67:33
        frame #13: 0x0000000100573478 handling-errors`std::panicking::try::do_call::h77eb563f55a56484 [inlined]     std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::h04f5bc1b8502155f at rt.rs:52:12 [opt]
        frame #14: 0x000000010057346c handling-errors`std::panicking::try::do_call::h77eb563f55a56484 at panicking.rs:292 [opt]
        frame #15: 0x00000001005755df handling-errors`__rust_maybe_catch_panic at lib.rs:78:7 [opt]
        frame #16: 0x0000000100573e4e handling-errors`std::rt::lang_start_internal::h3d261fac4b6382f2 [inlined] std::panicking::try::h989c79f60ffdf02a at panicking.    rs:270:12 [opt]
        frame #17: 0x0000000100573e1b handling-errors`std::rt::lang_start_internal::h3d261fac4b6382f2 [inlined] std::panic::catch_unwind::hd3f56528916c87b0 at     panic.rs:394 [opt]
        frame #18: 0x0000000100573e1b handling-errors`std::rt::lang_start_internal::h3d261fac4b6382f2 at rt.rs:51 [opt]
        frame #19: 0x0000000100005342 handling-errors`std::rt::lang_start::h90894c962c24c170(main=&0x1000017c0, argc=1, argv=&0x7ffeefbff718) at rt.rs:67:4
        frame #20: 0x0000000100001ec2 handling-errors`main + 34
        frame #21: 0x00007fff6c12d7fd libdyld.dylib`start + 1
        frame #22: 0x00007fff6c12d7fd libdyld.dylib`start + 1
    ```

    Look at the line starting `frame #11`:

    ```
    frame #11: 0x0000000100001c02 handling-errors`handling_errors::main::h737fb5e67c519ca8 at main.rs:39:20
    ```

    This tells us that at line 39 of `main.rs`, we called a function that ultimately generated an unhandled error.  So now, at least we have a starting point for examining where things have gone wrong.

    Enter `exit` or `quit` to leave the low-level debugger.




Next, let's take a look at how we can interrupt an executing WASM module.
