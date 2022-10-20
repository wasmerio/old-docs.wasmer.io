# Python

You can use Wasmer in your Python projects to execute WebAssembly modules securely and conveniently.

In this section we will go through the instructions on how to setup your Python environment, to then visit different examples of how to use Wasmer in Python.

## Setup your Python Environment

To be able to run Wasmer inside our Python application, we will need Python to be installed in our system. Both Python 2.7 and 3.x are compatible with Wasmer.

The easiest way to get [Python](https://www.python.org/) in your system is to follow the official [documentation](https://www.python.org/about/gettingstarted/).

To ensure it is installed, let's run the following:

```bash
python --version # This will display the Python version
```

{% hint style="success" %}
If this command work, Python is successfully installed!
{% endhint %}

## Start a Python project with Wasmer

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
pip install wasmer wasmer_compiler_cranelift
```

> More compilers are also available:
>
> * Singlepass: `pip install wasmer_compiler_singlepass`
> * LLVM: `pip install wasmer_compiler_llvm`

{% hint style="info" %}
For a detailed installation instructions, please refer to Wasmer Python integration [documentation](https://github.com/wasmerio/wasmer-python).
{% endhint %}

Next, let's take a look at some examples!

{% content-ref url="examples/" %}
[examples](examples/)
{% endcontent-ref %}
