# Sending Some Messages, Using Some Tools

This chapter assumes you've completed the steps outlined in [Chapter 1](./chapter_1.md) to construct your dev environment or otherwise have a basic Kinode app open in your code editor of choice.
You should also be actively running an Kinode ([live](../login.md) or [fake](./chapter_1.md#booting-a-fake-kinode-node)) such that you can quickly compile and test your code!
Tight feedback loops when building: very important.

## Starting from Scratch

If you want to hit the ground running, you can take the template code or the [chess tutorial](../chess_app/start.md) and start hacking away.
Here, you'll start from scratch and learn about every line of boilerplate.

The last chapter explained packages, the package manifest, and metadata.
Every package contains one or more processes, which are the actual Wasm programs that will run on a node.
In order to compile properly to the Kinode environment, every process must generate the WIT bindings for the `process` "world".

```rust
wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});
```

After generating the bindings, every process must define a `Component` struct and implement the `Guest` trait for it defining a single function, `init()`.
This is the entry point for the process, and the `init()` function is the first function called by the Kinode runtime when the process is started.

The definition of the `Component` struct can be done manually, but it's easier to import the [`kinode_process_lib`](../process_stdlib/overview.md) crate (a sort of standard library for Kinode processes written in Rust) and use the `call_init!` macro.
Note that running the process below [can lead to an infinite loop](#aside-on_exit):

```rust
use kinode_process_lib::{call_init, Address};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");
}
```

Every Kinode process written in Rust will need code that does the same thing as the above.
The `Address` parameter tells our process what its globally-unique name is. (TODO: link to docs)

Let's fill out the init function with code that will stop it from exiting immediately.
Here's an infinite loop that will wait for a message and then print it out.
Note that you are importing a few more things from the [process_lib](../process_stdlib/overview.md) including a `println!` macro that replaces the standard Rust one.

```rust
use kinode_process_lib::{await_message, call_init, println, Address};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");

    loop {
        let next_message = await_message();
        println!("{our}: got message: {next_message:?}");
    }
}
```

See [kinode.wit](./apis/kinode_wit.md) for more details on what is imported by the WIT bindgen macro.
These imports are the necessary "system calls" for talking to other processes and runtime components in Kinode OS.

Run
```bash
kit build your_pkg_name
kit start-package your_pkg_name -p 8080
```

to see this code in the node you set up in the last chapter.

## Sending a Message

Let's send a message to another process.
The `Request` type in [process_lib](../process_stdlib/overview.md) will provide all the necessary functionality.
```rust
use kinode_process_lib::{await_message, call_init, println, Address, Request};
```

`Request` is a builder struct that abstracts over the raw interface presented in the WIT bindings.
It's very simple to use:
```rust
Request::new()
    .target(my_target_address)
    .body(my_body_bytes)
    .send();
```

Because this process might not have capabilities to message any other (local or remote) processes, just send the message to itself.

```rust
Request::new()
    .target(our)
    .body(b"hello world")
    .send();
```

Note that `send()` returns a Result.
If you know that a `target` and `body` was set, you can safely unwrap this: send will only fail if one of those two fields are missing.

Here's the full process code, with both sending and handling the message:
```rust
use kinode_process_lib::{await_message, call_init, println, Address, Request};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");

    Request::new()
        .target(&our)
        .body(b"hello world")
        .send()
        .unwrap();

    loop {
        let next_message = await_message();
        println!("{our}: got message: {next_message:?}");
    }
}
```

Using `kit build` and `kit start-package` like before, you should be able to see in your node's terminal the message being received in the loop.
However, you'll see the "hello world" message as a byte vector.

Let's modify our request to expect a response, and our message-handling to send one back, as well as parse the received request into a string.

```rust
Request::to(&our)
    .body(b"hello world")
    .expects_response(5)
    .send()
```

The `expects_response` method takes a timeout in seconds.
If the timeout is reached, the request will be returned to the process that sent it as an error.
If you add that to the code above, you'll see the error after 5 seconds in your node's terminal.

Now, let's add some code to handle the request. The `await_message()` function returns a type that looks like this:
```rust
Result<Message, SendError>
```

That `SendError` is what's coming when the request times out.
Let's add a `match` statement that first checks whether the incoming value is a message or an error, then see if the message is a request or a response.

```rust
loop {
    match await_message() {
        Ok(message) => {
            if message.is_request() {
                println!("{our}: got request: {message:?}");
            } else {
                println!("{our}: got response: {message:?}");
            }
        }
        Err(_send_error) => {
            println!("got send error!");
        }
    }
}
```

This code won't send a response back yet.
To do that, import the `Response` type from `process_lib` and fire one off inside the request branch.

```rust
use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};
// ...
if message.is_request() {
    println!("{our}: got request: {message:?}");
    Response::new()
        .body(b"hello world to you too!")
        .send()
        .unwrap();
}
// ...
```

Building and starting the package now will show the request and response in the node's terminal.
But it's still ugly.
Let's put it all together and add a bit more handling to show the `body` value as a string:

```rust
use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");

    Request::new()
        .target(&our)
        .body(b"hello world")
        .expects_response(5)
        .send()
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                if message.is_request() {
                    println!(
                        "{our}: got a message: {}",
                        String::from_utf8_lossy(message.body())
                    );
                    Response::new()
                        .body(b"hello world to you too!")
                        .send()
                        .unwrap();
                } else {
                    println!(
                        "{our}: got a response: {}",
                        String::from_utf8_lossy(message.body())
                    );
                }
            }
            Err(_send_error) => {
                println!("got send error!");
            }
        }
    }
}
```

This basic structure can be found in the majority of Kinode processes.
The other common structure is a thread-like process, that sends and handles a fixed series of messages and then exits.

In the next chapter, we will cover how to turn this very basic request-response pattern into something that can be extensible and composable.

## Aside: `on_exit`

As mentioned in the [previous chapter](./chapter_1.md#pkgmanifestjson), one of the fields in the `manifest.json` is `on_exit`.
When the process exits, it does one of:

`on_exit` setting | Behavior
----------------- | --------
`"None"`          | Do nothing
`"Restart"`       | Restart the process
JSON object       | Send the message described by the JSON object

A process intended to do something once and exit should have `"None"` or a JSON object `on_exit`.
If it has `"Restart"`, it will repeat in an infinite loop, as reference [above](#starting-from-scratch).

A process intended to run over a period of time and serve requests and responses will often have `"Restart"` `on_exit` so that, in case of crash, it will start again.
Alternatively, a JSON object `on_exit` can be used to inform another process of its untimely demise.
In this way, Kinode processes become quite similar to Erlang processes, and crashing can be [designed into your process to increase reliability](https://ferd.ca/the-zen-of-erlang.html).

