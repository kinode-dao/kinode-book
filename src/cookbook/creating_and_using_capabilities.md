# Creating and Using Capabilities

Previous examples have shown how to acquire capabilities in a manifest file.
Here, we'll show how a package that does not come "built in" to Kinode can create its own capabilities and how other processes can use them.

Recall that capabilities are tokens of authority that processes can use to authorize behaviors and are managed by the kernel.
In userspace, there are two common patterns for using capabilities: requesting/granting them in the package manifest, or attaching them to a message. The first pattern is more common, and generally matches the intuition of the program's end-user: when they install an app, they are presented with a list of actions that the app will be able to perform, such as messaging the "eth" process to read blockchain data or access the files of a specific other package.

There is no need to register capabilities that can be granted.
When another process requests them, the package manager / app store, which has kernel-messaging authority, can spawn them if a user approves.
This allows capabilities to be granted before they are needed, even if the relevant package is not installed yet.

To require that a capability exist in order to fulfill a message, one can check for its existence in the `capabilities` field of the message.

```rust,noplayground,no_run
{{#include ../../code/capabilities/contacts/src/lib.rs:check_capabilities}}
```

This code is run on each incoming request to the `contacts` process.
Depending on the kind of request, the code generates one of four different required capabilities and checks whether the necessary one is present in the `capabilities` field of the message.
If not, the process responds with an error message.

This example uses the same API as the `contacts` app included in the default Kinode distribution: for a guide to use the *actual* contacts system primitive, see [Managing Contacts](managing_contacts.md).

Note that the format of the capability is presented in a [WIT API](../system/process/wit_apis.md) file alongside the request and response types.
This allows other processes to easily produce the correct capability when requesting it.

Now, take a look at the manifest for the `contacts-test` process.
```json,noplayground,no_run
{{#include ../../code/capabilities/pkg/manifest.json:20:48}}
```

This manifest requests all four capabilities from the `contacts` process.
Naturally, the correct package name and publisher must be used here.
The `"params"` field must match the JSON serialization of the capability type that lives in the WIT API:

```rust,noplayground,no_run
    enum capabilities {
        read-name-only,
        read,
        add,
        remove,
    }
```

Let's see `contacts-test` using these capabilities in action.

```rust,noplayground,no_run
{{#include ../../code/capabilities/contacts-test/src/lib.rs:use_capabilities}}
```

When building each request (except for the one that specifically does not attach a capability, and fails as a result), a capability is attached in the request's builder pattern.
Because the capabilities were requested in the manifest, they can be created here and used.
If a capability did not exist in the manifest, and was not otherwise acquired during runtime, the capability *would not* show up for the message receiver, because the kernel validates each capability attached to a message and filters out invalid ones.

Go ahead and use kit to install this package, available [here](https://github.com/kinode-dao/kinode-book/tree/main/code/capabilities), and see how `contacts-test` uses capabilities to interact with `contacts`.
