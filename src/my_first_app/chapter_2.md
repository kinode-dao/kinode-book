# Sending and Responding to a Message

In this section you will learn how to use different parts of a process, how request-response handling works, and other implementation details with regards to messaging.
The process you will build will be simple — it messages itself and responds to itself, printing whenever it gets messages.

Note — the app you will build in Sections 2 through 5 is *not* `my_chat_app`; it is simply a series of examples designed to demonstrate how to use the system's features.

## Requirements

This section assumes you've completed the steps outlined in [Environment Setup](./chapter_1.md) to construct your development environment or otherwise have a basic Kinode app open in your code editor of choice.
You should also be actively running a Kinode ([live](../login.md) or [fake](./chapter_1.md#booting-a-fake-kinode-node)) such that you can quickly compile and test your code!
Tight feedback loops when building: very important.

## Starting from Scratch

If you want to hit the ground running by yourself, you can take the template code or the [chess tutorial](../chess_app/chess_engine.md) and start hacking away.
Here, you'll start from scratch and learn about every line of boilerplate.
Open `src/lib.rs`, clear its contents so it's empty, and code along!

The last section explained packages, the package manifest, and metadata.
Every package contains one or more processes, which are the actual Wasm programs that will run on a node.

The [Generating WIT Bindings](#generating-wit-bindings) and [`init()` Function](#init-function) subsections explain the boilerplate code in detail, so if you just want to run some code, you can skip to [Running First Bits of Code](#running-first-bits-of-code).

### Generating WIT Bindings

For the purposes of this tutorial, crucial information from this [Wasm documentation](https://component-model.bytecodealliance.org/design/why-component-model.html) has been abridged in this small subsection.

A [Wasm component](https://component-model.bytecodealliance.org/design/components.html) is a wrapper around a core module that specifies its imports and exports.
E.g. a Go component can communicate directly and safely with a C or Rust component.
It need not even know which language another component was written in — it needs only the component interface, expressed in WIT.

The external interface of a component - its imports and exports - is described by a [`world`](https://component-model.bytecodealliance.org/design/wit.html#worlds).
Exports are provided by the component, and define what consumers of the component may call; imports are things the component may call.
The component, however, internally defines how that `world` is implemented.
This interface is defined via [WIT](https://component-model.bytecodealliance.org/design/wit.html).

WIT bindings are the glue code that is necessary for the interaction between Wasm modules and their host environment.
They may be written in any Wasm-compatible language — Kinode offers the most support for Rust with [`kit`](../kit-dev-toolkit.md) and [`process_lib`](../process_stdlib/overview.md).
The `world`, types, imports, and exports are all declared in a [WIT file](https://github.com/kinode-dao/kinode-wit/blob/master/kinode.wit), and using that file, [`wit_bindgen`](https://github.com/bytecodealliance/wit-bindgen) generates the code for the bindings.

So, to bring it all together...

In order to compile properly to the Kinode environment, based on the WIT file, every process must generate the WIT bindings for the `process` `world`, which is an interface for the Kinode kernel.

```rust
{{#include ../code/mfa_message_demo/mfa_message_demo/src/lib.rs:3:6}}
```

### `init()` Function

The entrypoint of a process is the `init()` function.
You can use the `call_init!()` macro as shown below.
Skip to the code to see how to do so.
The following prose is an explanation of what is happening under-the-hood in the `call_init()` macro.

After generating the bindings, every process must define a `Component` struct which implements the `Guest` trait (i.e. a wrapper around the process which defines the export interface, as discussed [above](#generating-wit-bindings)).
The `Guest` trait should define a single function — `init()`.
This is the entry point for the process, and the `init()` function is the first function called by the Kinode runtime when the process is started.

The definition of the `Component` struct can be done manually, but it's easier to import the [`kinode_process_lib`](../process_stdlib/overview.md) crate (a sort of standard library for Kinode processes written in Rust) and use the `call_init!` macro.

```rust
{{#include ../code/mfa_message_demo/mfa_message_demo/src/lib.rs::9}}
...
```

Every Kinode process written in Rust will need code that does the same thing as the code above (i.e. use the `wit_bindgen::generate!()` and `call_init!()` macros).
See [kinode.wit](../apis/kinode_wit.md) for more details on what is imported by the WIT bindgen macro.
These imports are the necessary "system calls" for talking to other processes and runtime components in Kinode OS.
Note that there are a variety of imports from the [`process_lib`](../process_stdlib/overview.md) including a `println!` macro that replaces the standard Rust one.

The [`our` parameter](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/kinode/process/standard/struct.Address.html) tells the process what its globally-unique name is.

The `init()` function can either do one task and then return, or it can `loop`, waiting for a new message and then acting based on the nature of the message.
The first pattern is more usual for scripts that do one task and then exit.
The second pattern is more usual for long-lived state machine processes that, e.g., serve data over the network or over HTTP.

## Sending a Message

The [`Request`](https://docs.rs/kinode_process_lib/latest/kinode_process_lib/struct.Request.html) type from the [`process_lib`](../process_stdlib/overview.md) provides all the necessary functionality to send a Message.

`Request` is a builder struct that abstracts over the raw interface presented in the WIT bindings.
It's very simple to use:
```rust
    Request::to(my_target_address)
        .body(my_body_bytes)
        .send();
```

Because this process might not have [capabilities](../process/capabilities.md) to message any other (local or remote) processes, for the purposes of this tutorial, just send the message to itself.

```rust
    Request::to(&our)
        .body(b"hello world")
        .send();
```

Note that `send()` returns a Result.
If you know that a `target` and `body` was set, you can safely unwrap this: send will only fail if one of those two fields are missing.

You can modify your request to expect a response, and your message-handling to send one back, as well as parse the received request into a string.

```rust
{{#include ../code/mfa_message_demo/mfa_message_demo/src/lib.rs:12:16}}
```

The `expects_response` method takes a timeout in seconds.
If the timeout is reached, the request will be returned to the process that sent it as an error.
If you add that to the code above, you'll see the error after 5 seconds in your node's terminal.

## Responding to a Message

Now, consider how to handle the request.
The `await_message()` function returns a `Result` that looks like this:
```rust
Result<Message, SendError>
```

The `SendError` is returned when a request times out or, if the request passes over the network, in case of a networking issue.
Use a `match` statement to check whether the incoming value is a message or an error, then branch on whether the message is a `Request` or a `Response`.
To send a `Response` back, import the `Response` type from `process_lib` and send one from the `Request` branch.

```rust
{{#include ../code/mfa_message_demo/mfa_message_demo/src/lib.rs:18:34}}
```

Putting it all together:

```rust
{{#include ../code/mfa_message_demo/mfa_message_demo/src/lib.rs}}
```

Run
```bash
kit build your_pkg_directory
kit start-package your_pkg_directory -p 8080
```
to see the messages being sent by your process.

You can find the full code [here](https://github.com/kinode-dao/kinode-book/tree/main/src/code/mfa_message_demo).

This basic structure can be found in the majority of Kinode processes.
The other common structure is a script-like process, that sends and handles a fixed series of messages and then exits.

In the next section, you will learn how to turn this very basic request-response pattern into something that can be extensible and composable.
