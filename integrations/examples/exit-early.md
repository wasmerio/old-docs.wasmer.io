# 🛑 Interrupting Execution

WebAssembly is currently always run in the same process synchronously. Thus, once WebAssembly starts executing, you have to wait for the execution to complete to continue running code on the host \(your application\).

However, there are cases where you may want to interrupt this synchronous execution while the guest Wasm module is calling a host function. This can be useful for saving resources, and not returning back to the guest Wasm for execution, when you already know the Wasm execution will fail, or no longer be needed.

In this example, we will run a Wasm module that calls the imported host function `interrupt_execution`. This host function will immediately stop executing the WebAssembly module.

First we are going to want to initialize a new project. To do this we can navigate to our project folder, or create one. In this example, we will create a new project. Lets create it and navigate to it:

{% tabs %}
{% tab title="Rust" %}
{% hint style="info" %}
The final **Rust** code for this example can be found on Github: [early_exit.rs](https://github.com/wasmerio/wasmer/blob/master/examples/early_exit.rs).

_Please take a look at the_ [_setup steps for Rust_](../rust/setup.md)_._
{% endhint %}

```bash
cargo new early-exit
cd early-exit
```

We have to modify `Cargo.toml` to add the Wasmer dependencies as shown below:

```yaml
[dependencies]
# The Wasmer API
wasmer = "2.0"
```
{% endtab %}

{% tab title="Go" %}
{% hint style="info" %}
The final **Go** code for this example can be found on Github: [early_exit.go](https://github.com/wasmerio/wasmer-go/blob/master/examples/example_early_exit_test.go).

_Please take a look at the_ [_setup steps for Go_](../go/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-early-exit
cd wasmer-example-early-exit
go mod init github.com/$USER/wasmer-example-early-exit
```
{% endtab %}

{% tab title="Python" %}
{% hint style="info" %}
The final **Python** code for this example can be found on Github: [imports_function_early_exit.py](https://github.com/wasmerio/wasmer-python/blob/master/examples/imports_function_early_exit.py).

_Please take a look at the_ [_setup steps for Python_](../python/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-early-exit
cd wasmer-example-early-exit
pip install wasmer wasmer_compiler_cranelift
```
{% endtab %}

{% tab title="PHP" %}
{% hint style="info" %}
The final **PHP** code for this example can be found on Github: [imports-function-early-exit.php](https://github.com/wasmerio/wasmer-php/blob/master/examples/imports-function-early-exit.php).

_Please take a look at the_ [_setup steps for PHP_](../php/setup.md)_._
{% endhint %}

```bash
mkdir wasmer-example-early-exit
cd wasmer-example-early-exit
composer init --name=wasmer-example-early-exit
composer require wasm/wasm
```
{% endtab %}

{% tab title="Ruby" %}
{% hint style="info" %}
The final **Ruby** code for this example can be found on Github: [imports_function_early_exit.rb](https://github.com/wasmerio/wasmer-ruby/blob/master/examples/imports_function_early_exit.rb).

_Please take a look at the_ [_setup steps for Ruby_](../ruby/setup.md)_._
{% endhint %}

```bash
gem install wasmer
```
{% endtab %}
{% endtabs %}

Now that we have everything set up, let's go ahead and try it out!

## Setting up

Before we start with the Wasm part we'll have to declare the error we'll use to terminate the execution of the guest module:

{% tabs %}
{% tab title="Rust" %}
```rust
#[derive(Debug, Clone, Copy)]
struct ExitCode(u32);

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExitCode {}
```
{% endtab %}

{% tab title="Go" %}
```go
type exitCode struct {
    code int32
}

func (self *exitCode) Error() string {
    return fmt.Sprintf("exit code: %d", self.code)
}
```
{% endtab %}

{% tab title="Python" %}
```python
class ExitCode(Exception):
    pass
```
{% endtab %}

{% tab title="PHP" %}
```php
class ExitCode extends Exception {}
```
{% endtab %}
{% endtabs %}

There is nothing special or Wasmer specific here but it will be required later in the example.

## Defining and importing the host function

To terminate the execution of the Wasm module we'll have to define a function on the host which will then be imported in the guest and called whenever execution is not required to continue. Let's do that:

{% tabs %}
{% tab title="Rust" %}
```rust
fn early_exit() {
    RuntimeError::raise(Box::new(ExitCode(1)));
}

let import_object = imports! {
    "env" => {
        "early_exit" => Function::new_native(&store, early_exit),
    }
};
```
{% endtab %}

{% tab title="Go" %}
```go
func earlyExit(args []wasmer.Value) ([]wasmer.Value, error) {
    return nil, &exitCode{1}
}

importObject := wasmer.NewImportObject()
importObject.Register(
    "env",
    map[string]wasmer.IntoExtern{
        "early_exit": wasmer.NewFunction(
            store,
            wasmer.NewFunctionType(wasmer.NewValueTypes(), wasmer.NewValueTypes()),
            earlyExit,
        ),
    },
)
```
{% endtab %}

{% tab title="Python" %}
```python
def early_exit():
    raise ExitCode("oops")

import_object = ImportObject()
import_object.register(
    "env",
    {
        "early_exit": Function(store, early_exit),
    }
)
```
{% endtab %}

{% tab title="PHP" %}
```php
function earlyExit() {
    throw new ExitCode(1);
}

$funcType = Wasm\Type\FuncType::new(new Wasm\Vec\ValType(), new Wasm\Vec\ValType());
$func = Wasm\Func::new($store, $funcType, 'earlyExit');
$extern = $func->asExtern();
$externs = new Wasm\Vec\Extern([$extern->inner()]);
```
{% endtab %}

{% tab title="Ruby" %}
```ruby
def early_exit
  raise "oops"
end

func_type = Wasmer::FunctionType.new([], [])
func = Wasmer::Function.new(store, method(:early_exit), func_type)
import_object = Wasmer::ImportObject.new
import_object.register("env", { :early_exit => func })
```
{% endtag %}
{% endtabs %}

As we saw in previous examples we defined a Rust function, wrap it in a native function definition and import it in the guest module, in the `env` namespace, using the `ImportObject`.

## Handling the error

Our module will call the `early_exit` function once we call its `run` function \(which is an exported function\). Let's get the function, call it and see how we can handle the error:

{% tabs %}
{% tab title="Rust" %}
```rust
let run_func: NativeFunc<(i32, i32), i32> = instance
    .exports
    .get_native_function("run")?;

match run_func.call(1, 7) {
    Ok(result) => {
        bail!("Expected early termination with `ExitCode`, found: {}", result);
    }   
    Err(e) => match e.downcast::<ExitCode>() {
        // We found the exit code used to terminate execution.
        Ok(exit_code) => {
            println!("Exited early with exit code: {}", exit_code);
            Ok(())
        }
        Err(e) => {
            bail!("Unknown error `{}` found. expected `ErrorCode`", e);
        }
    },
}
```

We expect to get an error when calling the `run` function so what we do here is look at the result and:

* if we get a success, our test will fail;
* if we get an error, we try to downcast to our `ExitCode` error.

If downcasting succeeds it means we actually got the expected error so we make the test pass. If it fails, it means the Wasm module reported an error but it wasn't the one we expected so we make the test fail.
{% endtab %}

{% tab title="Go" %}
```go
run, err := instance.Exports.GetFunction("run")

if err != nil {
    panic(fmt.Sprintln("Failed to retrieve the `run` function:", err))
}


_, err = run(1, 7)

if err == nil {
    panic(fmt.Sprintln("`run` did not error"))
}

fmt.Println("Exited early with:", err)
```
{% endtab %}

{% tab title="Python" %}
```go
try:
    instance.exports.run(1, 2)
except RuntimeError as err:
    assert "oops" in str(err)
else:
    assert False
```
{% endtab %}

{% tab title="PHP" %}
```go
try {
    $run($args);

    echo '`run` did not error'.PHP_EOL;

    exit(1);
} catch (ExitCode $exception) {
    echo 'Exited early with: '.$exception->getMessage().' '.$exception->getCode().PHP_EOL;
}
```
{% endtab %}

{% tab title="Ruby" %}
```ruby
begin
  instance.exports.run.(1, 2)
rescue RuntimeError => e
  puts e.message
else
  raise "There was no error"
end
```
{% endtab %}
{% endtabs %}

## Running

We now have everything we need to run the Wasm module, let's do it!

{% tabs %}
{% tab title="Rust" %}
You should be able to run it using the `cargo run` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `run` function...
Exited early with exit code: 1
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer.git
cd wasmer
cargo run --example early-exit --release --features "cranelift"
```
{% endhint %}
{% endtab %}

{% tab title="Go" %}
You should be able to run it using the `go run main.go` command. The output should look like this:

```text
Compiling module...
Instantiating module...
Calling `run` function...
Exited early with: exit code: 1
```

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-go/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-go.git
cd wasmer-go
go test examples/example_memory_test.go
```
{% endhint %}
{% endtab %}

{% tab title="Python" %}
You should be able to run it using the `python imports_function_early_exit.py` command.

{% hint style="info" %}
If you want to run the examples from the Wasmer [repository](https://github.com/wasmerio/wasmer-python/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-python.git
cd wasmer-python
python examples/imports_function_early_exit.py
```
{% endhint %}
{% endtab %}

{% tab title="PHP" %}
You should be able to run it using the `php imports-function-early-exit.php` command.

{% hint style="info" %}
If you want to run the examples from the Wasmer PHP [repository](https://github.com/wasmerio/wasmer-php/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-php.git
cd wasmer-php
make EXAMPLE=imports-function-early-exit test-doc-examples
```
{% endhint %}
{% endtab %}

{% tab title="Ruby" %}
You should be able to run it using the `ruby imports_function_early_exit.rb` command.

{% hint style="info" %}
If you want to run the examples from the Wasmer Ruby [repository](https://github.com/wasmerio/wasmer-ruby/) codebase directly, you can also do:

```bash
git clone https://github.com/wasmerio/wasmer-ruby.git
cd wasmer-ruby
ruby examples/imports_function_early_exit.rb
```
{% endhint %}
{% endtab %}
{% endtabs %}

