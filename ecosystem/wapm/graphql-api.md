---
description: >-
  The most powerful API for accessing your WebAssembly modules, now completely
  open for all to use
---

# GraphQL API

All WAPM services \(both the registry frontend: [wapm.io](https://wapm.io/) and the CLI client [wapm-cli](https://github.com/wasmerio/wapm-cli)\) are using the WAPM GraphQL API to interact and retrieve data from the WAPM Registry.

### What is GraphQL?

> GraphQL is a query language for APIs and a runtime for fulfilling those queries with your existing data. GraphQL provides a complete and understandable description of the data in your API, gives clients the power to ask for exactly what they need and nothing more, makes it easier to evolve APIs over time, and enables powerful developer tools.

If you want to learn more about GraphQL, please visit the official Website: [GraphQL.org](https://graphql.org/)

{% hint style="info" %}
#### Did you know...?

[WebAssembly.sh](../webassembly.sh.md) is using the WAPM GraphQL API to expose the WASI modules in the shell automatically
{% endhint %}

### How I can use the WAPM API?

The WAPM GraphQL API \(and the GraphiQL IDE\) is online here:

{% embed url="https://registry.wapm.io/graphql" caption="https://registry.wapm.io/graphql" %}

Let's see now a few things that we can do with it!

## Examples

### Get a Package Version

We can do a POST to [https://registry.wapm.io/graphql](https://registry.wapm.io/graphql) with the the following as the `query` POST field:

```graphql
{
    getPackageVersion(name: "python") {
    version
    repository
    homepage
    distribution {
      downloadUrl
      size
    }
  }
}
```

Which should return something similar to:

```javascript
{
  "data": {
    "getPackageVersion": {
      "version": "0.1.0",
      "repository": "https://github.com/wapm-packages/python",
      "homepage": null,
      "distribution": {
        "downloadUrl": "https://registry-cdn.wapm.io/packages/_/python/python-0.1.0.tar.gz",
        "size": 5097541
      }
    }
  }
}
```

### Get all Packages for a given Interface

If we want to search all the packages published that have certain interface \(for example, WASI\), we can do a POST to [https://registry.wapm.io/graphql](https://registry.wapm.io/graphql) with the the following as the `query` POST field:

```graphql
{
  getInterfaceVersion(name: "wasi", version: "latest") {
    interface {
      name
      description
    }
    packageVersions {
      edges {
        node {
          version
          package {
            name
          }
          distribution {
            downloadUrl
          }
        }
      }
    }
  }
}
```

Which should return something similar to:

```javascript
{
  "data": {
    "getInterfaceVersion": {
      "interface": {
        "name": "wasi",
        "description": "The WebAssembly System Interface. WASI is a modular system interface for WebAssembly. It’s focused on security and portability."
      },
      "packageVersions": {
        "edges": [
          {
            "node": {
              "version": "0.1.0",
              "package": {
                "name": "_/python"
              },
              "distribution": {
                "downloadUrl": "https://registry-cdn.wapm.io/packages/_/python/python-0.1.0.tar.gz"
              }
            }
          },
          {
            "node": {
              "version": "0.0.2",
              "package": {
                "name": "JeremyLikness/wasi-ubasic"
              },
              "distribution": {
                "downloadUrl": "https://registry-cdn.wapm.io/packages/JeremyLikness/wasi-ubasic/wasi-ubasic-0.0.2.tar.gz"
              }
            }
          },
          {
            "node": {
              "version": "0.4.6",
              "package": {
                "name": "vshymanskyy/wasm3"
              },
              "distribution": {
                "downloadUrl": "https://registry-cdn.wapm.io/packages/vshymanskyy/wasm3/wasm3-0.4.6.tar.gz"
              }
            }
          }
        ]
      }
    }
  }
}
```

**Happy hacking! 🎉**

