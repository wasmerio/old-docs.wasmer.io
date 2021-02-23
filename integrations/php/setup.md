# Setup your PHP environment

To be able to run Wasmer inside our PHP application, we will need PHP to be installed in our system.

The easiest way to get [PHP](https://php.net/) in your system is to follow the official [documentation](https://www.php.net/manual/en/getting-started.php).

To ensure it is installed, let's run the following:

```bash
php --version # This will show the PHP version
```

{% hint style="success" %}
If this command works, PHP is successfully installed!
{% endhint %}

## Start a PHP project

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

{% page-ref page="../examples/" %}

