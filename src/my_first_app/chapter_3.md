# Messaging with More Complex Data Types

In this section, you will upgrade your app so that it can handle messages with more elaborate data types such as `enum`s and `struct`s.
Additionally, you will learn how to handle processes completing or crashing.

## (De)Serialization With Serde

In the last section, you created a simple request-response pattern that uses strings as a `body` field type.
This is fine for certain limited cases, but in practice, most Kinode processes written in Rust use a `body` type that is serialized and deserialized to bytes using [Serde](https://serde.rs/).
There are a multitude of libraries that implement Serde's `Serialize` and `Deserialize` traits, and the process developer is responsible for selecting a strategy that is appropriate for their use case.

Some popular options are [`bincode`](https://docs.rs/bincode/latest/bincode/), [`rmp_serde`](https://docs.rs/rmp-serde/latest/rmp_serde/) ([MessagePack](https://msgpack.org/index.html)), and [`serde_json`](https://docs.rs/serde_json/latest/serde_json/).
In this section, you will use `serde_json` to serialize your Rust structs to a byte vector of JSON.

### Defining the `body` Type

Our old request looked like this:
```rust
{{#include ../../code/mfa-message-demo/mfa-message-demo/src/lib.rs:12:16}}
```

What if you want to have two kinds of messages, which your process can handle differently?
You need a type that implements the `serde::Serialize` and `serde::Deserialize` traits, and use that as your `body` type.
You can define your types in Rust, but then:
1. Processes in other languages will then have to rewrite your types.
2. Importing types is haphazard and on a per-package basis.
3. Every package might place the types in a different place.

Instead, use the WIT language to define your API, discussed further [here](../system/process/wit_apis.md).
Briefly, WIT is a language-independent way to define types and functions for [Wasm components](https://component-model.bytecodealliance.org/design/why-component-model.html) like Kinode processes.
Kinode packages can define their API using a WIT file.
That WIT file is used to generate code in the given language during compile-time.
Kinode also defines a conventional place for these WIT APIs and provides infrastructure for viewing and importing the APIs of other packages.

```wit
{{#includehidetest ../../code/mfa-data-demo/api/mfa-data-demo:template.os-v0.wit}}
```

The `wit_bindgen::generate!()` macro changes slightly, since the `world` is now as defined in the API:
```rust
{{#include ../../code/mfa-data-demo/mfa-data-demo/src/lib.rs:4:9}}
```
which generates the types defined in the WIT API:
```rust
{{#include ../../code/mfa-data-demo/mfa-data-demo/src/lib.rs:1}}
```
It further adds the derives for `serde` so that these types can be used smoothly.

Now, when you form Requests and Responses, instead of putting a bytes-string in the `body` field, you can use the `MfaRequest`/`MfaResponse` type.
This comes with a number of benefits:

- You can now use the `body` field to send arbitrary data, not just strings.
- Other programmers can look at your code and see what kinds of messages this process might send to their code.
- Other programmers can see what kinds of messages you expect to receive.
- By using an `enum` ([WIT `variant`s become Rust `enum`s](https://component-model.bytecodealliance.org/design/wit.html#variants)), you can exhaustively handle all possible message types, and handle unexpected messages with a default case or an error.

Defining `body` types is just one step towards writing interoperable code.
It's also critical to document the overall structure of the program along with message `blob`s and `metadata` used, if any.
Writing interoperable code is necessary for enabling permissionless composability, and Kinode aims to make this the default kind of program, unlike the centralized web.

### Handling Messages

In this example, you will learn how to handle a Request.
So, create a request that uses the new `body` type:

```rust
{{#include ../../code/mfa-data-demo/mfa-data-demo/src/lib.rs:39:43}}
```

Next, change the way you handle a message in your process to use your new `body` type.
Break out the logic to handle a message into its own function, `handle_message()`.
`handle_message()` should branch on whether the message is a Request or Response.
Then, attempt to parse every message into the `MfaRequest`/`MfaResponse`, `enum` as appropriate, handle the two cases, and handle any message that doesn't comport to the type.
```rust
{{#include ../../code/mfa-data-demo/mfa-data-demo/src/lib.rs:11:34}}
```

### Granting Capabilities

Finally, edit your `pkg/manifest.json` to grant the terminal process permission to send messages to this process.
That way, you can use the terminal to send `Hello` and `Goodbye` messages.
Go into the manifest, and under the process name, edit (or add) the `grant_capabilities` field like so:

```json
...
{{#include ../../code/mfa-data-demo/pkg/manifest.json:10:13}}
...
```

### Build and Run the Code!

After all this, your code should look like:
```rust
{{#include ../../code/mfa-data-demo/mfa-data-demo/src/lib.rs}}
```
You should be able to build and start your package, then see that initial `Hello` message.
At this point, you can use the terminal to test your message types!

You can find the full code [here](https://github.com/kinode-dao/kinode-book/tree/main/code/mfa-data-demo).

First, try sending a `Hello` using the [`m` terminal script](../system/terminal.md#m---message-a-process).
Get the address of your process by looking at the "started" printout that came from it in the terminal.
As a reminder, these values (`<your_process>`, `<your_package>`, `<your_publisher>`) can be found in the `metadata.json` and `manifest.json` package files.

```bash
m our@<your-process>:<your-package>:<your-publisher> '{"Hello": "hey there"}'
```

You should see the message text printed.
To grab and print the Response, append a `-a 5` to the terminal command:
```bash
m our@<your-process>:<your-package>:<your-publisher> '{"Hello": "hey there"}' -a 5
```
Next, try a goodbye.
This will cause the process to exit.

```bash
m our@<your-process>:<your-package>:<your-publisher> '"Goodbye"'
```

If you try to send another `Hello` now, nothing will happen, because the process has exited [(assuming you have set `on_exit: "None"`; with `on_exit: "Restart"` it will immediately start up again)](#aside-on_exit).
Nice!
You can use `kit start-package` to try again.

## Aside: `on_exit`

As mentioned in the [previous section](./chapter_1.md#pkgmanifestjson), one of the fields in the `manifest.json` is `on_exit`.
When the process exits, it does one of:

`on_exit` Setting | Behavior When Process Exits
----------------- | ---------------------------
`"None"`          | Do nothing
`"Restart"`       | Restart the process
JSON object       | Send the requests described by the JSON object

A process intended to do something once and exit should have `"None"` or a JSON object `on_exit`.
If it has `"Restart"`, it will repeat in an infinite loop.

A process intended to run over a period of time and serve requests and responses will often have `"Restart"` `on_exit` so that, in case of crash, it will start again.
Alternatively, a JSON object `on_exit` can be used to inform another process of its untimely demise.
In this way, Kinode processes become quite similar to Erlang processes in that crashing can be [designed into your process to increase reliability](https://ferd.ca/the-zen-of-erlang.html).
