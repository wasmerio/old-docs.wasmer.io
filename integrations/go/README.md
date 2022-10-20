# Go

You can use Wasmer in your Go projects to execute WebAssembly modules securely and conveniently.

In this section we will go through the instructions on how to setup your Go environment, to then visit different examples of how to use Wasmer in Go.

## Setup your Go environment

To be able to run Wasmer inside our Go application, we will need Go to be installed in our system.

The easiest way to get [Go](https://golang.org/) in your system is to follow the official [documentation](https://golang.org/doc/install).

To ensure it is installed, let's run the following:

```bash
go version # This will display the Go version
```

{% hint style="success" %}
If this command work, Go is successfully installed!
{% endhint %}

## Start a Go project with Wasmer

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
go mod init github.com/$USER/wasmer-project
```

Now, edit the `go.mod` file to add `wasmer-go` as a dependency:

```bash
require (
    github.com/wasmerio/wasmer-go v1.0.0
)
```

{% hint style="info" %}
For a detailed installation instructions, please refer to Wasmer Go integration [documentation](https://github.com/wasmerio/wasmer-go).
{% endhint %}

Next, let's take a look at some examples!

{% content-ref url="../examples/" %}
[examples](../examples/)
{% endcontent-ref %}
