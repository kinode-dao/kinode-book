# Messaging with Larger Data Types

In this section, you will upgrade your app so that it can handle messages with more elaborate data types such as `enum`s and `struct`s.
Additionally, you will learn how to handle processes completing or crashing.

## (De)Serialization With Serde

In the last section, you created a simple request-response pattern that uses strings as a `body` field type.
This is fine for certain limited cases, but in practice, most Kinode processes written in Rust use a `body` type that is serialized and deserialized to bytes using [Serde](https://serde.rs/).
There are a multitude of libraries that implement Serde's `Serialize` and `Deserialize` traits, and the process developer is responsible for selecting a strategy that is appropriate for their use case.

Some popular options are `bincode`, [`rmp_serde`](https://docs.rs/rmp-serde/latest/rmp_serde/), and `serde_json`.
In this section, you will use `serde_json` to serialize your Rust structs to a byte vector of JSON.

### Defining the `body` Type

Our old request looked like this:
```rust
Request::to(&our)
    .body(b"hello world")
    .expects_response(5)
    .send()
    .unwrap();
```

What if you want to have two kinds of messages, which your process can handle differently?
Let's make a type that implements the `Serialize` and `Deserialize` traits, and use that as your `body` type.

```rust
use serde::{Serialize, Deserialize};

// ...

#[derive(Serialize, Deserialize)]
enum MyBody {
    Hello(String),
    Goodbye,
}

impl MyBody {
    fn hello(text: &str) -> Vec<u8> {
        serde_json::to_vec(&MyBody::Hello(text.to_string())).unwrap()
    }

    fn goodbye() -> Vec<u8> {
        serde_json::to_vec(&MyBody::Goodbye).unwrap()
    }

    fn parse(bytes: &[u8]) -> Result<MyBody, serde_json::Error> {
        serde_json::from_slice::<MyBody>(bytes)
    }
}
```

Now, when you form requests and responses, instead of sticking a string in the `body` field, you can use the new `MyBody` type.
This comes with a number of benefits:

- You can now use the `body` field to send arbitrary data, not just strings.
- Other programmers can look at your code and see what kinds of messages this process might send to their code.
- Other programmers can see what kinds of messages you expect to receive.
- By using an `enum`, you can exhaustively handle all possible message types, and handle unexpected messages with a default case or an error.

Defining `body` types is just one step towards writing interoperable code.
It's also critical to document the overall structure of the program along with message `blob`s and `metadata` used, if any.
Writing interoperable code is necessary for enabling permissionless composability, and Kinode OS aims to make this the default kind of program, unlike the centralized web.

### Handling Messages

In this example, you will learn how to handle a Request.
So, create a request that uses the new `body` type (you won't need to send a Response back, so we can remove `.expect_response()`):

```rust
Request::to(&our)
    .body(MyBody::hello("hello world"))
    .send()
    .unwrap();
```

Next, edit the way you handle a message in your process to use your new `body` type.
The process should attempt to parse every message into the `MyBody` enum, handle the two cases, and handle any message that doesn't comport to the type.
This piece of code goes into the `Ok(message)` case of the `match` statement on `await_message()`:
```rust
let Ok(body) = MyBody::parse(message.body()) else {
    println!("{our}: received a message with weird `body`!");
    continue;
};
if message.is_request() {
    // Respond to a Hello by printing it, and a Goodbye by exiting
    // the loop, which will cause the process to exit.
    match body {
        MyBody::Hello(text) => {
            println!("got a Hello: {text}");
        }
        MyBody::Goodbye => {
            println!("goodbye!");
            break;
        }
    }
} else {
    // we only expect Hello responses. If we get a Goodbye, ignore it.
    match body {
        MyBody::Hello(text) => {
            println!("got a Hello response: {text}");
        }
        MyBody::Goodbye => {}
    }
}
```

### Granting Capabilities

Finally, edit your `pkg/manifest.json` to grant the terminal process permission to send messages to this process.
That way, you can use the terminal to send `Hello` and `Goodbye` messages.
Go into the manifest, and under the process name, edit (or add) the `grant_capabilities` field like so:
```json
...
"grant_capabilities": [
    "terminal:terminal:sys"
],
...
```

### Build and Run the Code!

After all this, your code should look like:
```rust
use serde::{Serialize, Deserialize};
use kinode_process_lib::{await_message, call_init, println, Address, Request, Response};

#[derive(Serialize, Deserialize)]
enum MyBody {
    Hello(String),
    Goodbye,
}

impl MyBody {
    fn hello(text: &str) -> Vec<u8> {
        serde_json::to_vec(&MyBody::Hello(text.to_string())).unwrap()
    }

    fn goodbye() -> Vec<u8> {
        serde_json::to_vec(&MyBody::Goodbye).unwrap()
    }

    fn parse(bytes: &[u8]) -> Result<MyBody, serde_json::Error> {
        serde_json::from_slice::<MyBody>(bytes)
    }
}

wit_bindgen::generate!({
    path: "wit",
    world: "process",
});

call_init!(my_init_fn);
fn my_init_fn(our: Address) {
    println!("{our}: started");

    Request::to(&our)
        .body(MyBody::hello("hello world"))
        .send()
        .unwrap();

    loop {
        match await_message() {
            Ok(message) => {
                let Ok(body) = MyBody::parse(message.body()) else {
                    println!("{our}: received a message with weird `body`!");
                    continue;
                };
                if message.is_request() {
                    // Respond to a Hello by printing it, and a Goodbye by exiting
                    // the loop, which will cause the process to exit.
                    match body {
                        MyBody::Hello(text) => {
                            println!("got a Hello: {text}");
                        }
                        MyBody::Goodbye => {
                            println!("goodbye!");
                            break;
                        }
                    }
                } else {
                    // we only expect Hello responses. If we get a Goodbye, ignore it.
                    match body {
                        MyBody::Hello(text) => {
                            println!("got a Hello response: {text}");
                        }
                        MyBody::Goodbye => {}
                    }
                }
            }
            Err(_send_error) => {
                println!("got send error!");
            }
        }
    }
}
```
You should be able to build and start your package, then see that initial `Hello` message.
At this point, you can use the terminal to test your message types!

First, try sending a `Hello` using the [`m` terminal script](../terminal.md#m---message-a-process).
Get the address of your process by looking at the "started" printout that came from it in the terminal.
As a reminder, these values (`<your_process>`, `<your_package>`, `<your_publisher>`) can be found in the `metadata.json` and `manifest.json` package files.

```bash
m our@<your_process>:<your_package>:<your_publisher> '{"Hello": "hey there"}'
```

You should see the message text printed. Next, try a goodbye.
This will cause the process to exit.

```bash
m our@<your_process>:<your_package>:<your_publisher> '"Goodbye"'
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
