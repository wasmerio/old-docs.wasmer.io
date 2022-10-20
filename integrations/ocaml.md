# OCaml

You can use Wasmer in your OCaml projects to execute WebAssembly modules securely and conveniently.

In this section we will go through the instructions on how to setup your OCaml environment, to then visit different examples of how to use Wasmer in OCaml.

## Setup your OCaml Environment

To be able to run Wasmer inside our OCaml application, we will need OPAM (OCaml Package Manager) to be installed in our system.&#x20;

The easiest way to get [OCaml](https://ocaml.org/) in your system is to follow [the official documentation](https://ocaml.org/docs/up-and-running).

To ensure it is installed, let's run the following:

```bash
ocaml -version # This will display the OCaml version
```

{% hint style="success" %}
If this command work, OCaml is successfully installed!
{% endhint %}

## Start a OCaml project with Wasmer

Now it's time to create a new project and add Wasmer as a dependency:

```bash
mkdir wasmer-project
cd wasmer-project
opam install wasmer
```

Next, let's take a look at some examples!
