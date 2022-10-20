# PHP

You can use Wasmer in your PHP projects to execute WebAssembly modules securely and conveniently.

In this section we will go through the instructions on how to setup your PHP environment, to then visit different examples of how to use Wasmer in PHP.

## Setup your PHP environment

To be able to run Wasmer inside our PHP application, we will need PHP to be installed in our system.

The easiest way to get [PHP](https://php.net/) in your system is to follow the official [documentation](https://www.php.net/manual/en/getting-started.php).

To ensure it is installed, let's run the following:

```bash
php --version # This will show the PHP version
```

{% hint style="success" %}
If this command works, PHP is successfully installed!
{% endhint %}

## Start a PHP project with Wasmer

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
composer init --name=wasmer-project
composer require wasm/wasm
```

{% hint style="warning" %}
Before doing that you will have to install the PHP `wasm` extension.

For a detailed installation instructions, please refer to Wasmer PHP integration [documentation](https://github.com/wasmerio/wasmer-php).
{% endhint %}

Next, let's take a look at some examples!

{% content-ref url="../examples/" %}
[examples](../examples/)
{% endcontent-ref %}
