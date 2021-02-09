# Setup your Python environment

To be able to run Wasmer inside our Python application, we will need Python to be installed in our system.

The easiest way to get [Python](https://www.python.org/) in your system is to follow the official [documentation](https://www.python.org/about/gettingstarted/).

To ensure it is installed, let's run the following:

```bash
python --version # This will show the Python version
```

{% hint style="success" %}
If this command work, Python is successfully installed!
{% endhint %}

## Start a Python project

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
pip install wasmer
pip install wasmer_compiler_cranelift
```

{% hint style="info" %}
For a detailed installation instructions, please refer to Wasmer Python integration [documentation](https://github.com/wasmerio/wasmer-python).
{% endhint %}

Next, let's take a look at some examples!

{% page-ref page="../examples/" %}

