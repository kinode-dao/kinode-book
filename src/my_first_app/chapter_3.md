# Defining Your Protocol

In the last chapter, you created a simple request-response pattern that uses strings as an `body` field type.
This is fine for certain limited cases, but in practice, most Nectar processes written in Rust use an `body` type that is serialized and deserialized to bytes using [Serde](https://serde.rs/).
There are a multitude of libraries that implement Serde's `Serialize` and `Deserialize` traits, and the process developer is responsible for selecting a strategy that is appropriate for their use case.

Some popular options are `bincode` and `serde_json`.
In this chapter, you will use `serde_json` to serialize your Rust structs to a byte vector of JSON.

Our old request looked like this:
```rust
Request::to(&our)
    .body(b"hello world")
    .expects_response(5)
    .send();
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

Now, when you form requests and response, instead of sticking a string in the `body` field, you can use the new `body` type.
This comes with a number of benefits:

- You can now use the `body` field to send arbitrary data, not just strings.
- Other programmers can look at your code and see what kinds of messages this process might send to their code.
- Other programmers can see what kinds of messages you expect to receive.
- By using an `enum`, you can exhaustively handle all possible message types, and handle unexpected messages with a default case or an error.

Defining `body` types is just one step towards writing interoperable code.
It's also critical to document the overall structure of the program along with message `blob`s and `metadata` used, if any.
Writing interoperable code is necessary for enabling permissionless composability, and NectarOS aims to make this the default kind of program, unlike the centralized web.

First, create a request that uses the new `body` type (and stop expecting a response):
```rust
Request::new()
    .target(&our)
    .body(MyBody::hello("hello world"))
    .send();
```

Next, edit the way you handle a message in your process to use your new `body` type.
The process should attempt to parse every message into the `MyBody` enum, handle the two cases, and handle any message that doesn't comport to the type.
This code goes into the `Ok(message)` case of the `match` statement on `await_message()`:
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

Finally, edit your `pkg/manifest.json` to grant the terminal process permission to send messages to this process.
That way, you can use the terminal to send Hello and Goodbye messages.
Go into the manifest, and under the process name, edit (or add) the `grant_capabilities` field like so:
```json
...
"grant_capabilities": [
    "terminal:terminal:nectar"
],
...
```

After all this, your code should look like:
```rust
use serde::{Serialize, Deserialize};
use nectar_process_lib::{await_message, call_init, println, Address, Request, Response};

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
    exports: {
        world: Component,
    },
});

call_init!(my_init_fn);

fn my_init_fn(our: Address) {
    println!("{our}: started");

    Request::new()
        .target(&our)
        .body(MyBody::hello("hello world"))
        .send();

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
You should be able to build and start your package, then see that initial Hello message.
At this point, you can use the terminal to test your message types!

First, try a hello. Get the address of your process by looking at the "started" printout that came from it in the terminal.
As a reminder, these values are set in the `metadata.json` and `manifest.json` package files.
```bash
/m our@<your_process>:<your_package>:<your_publisher> {"Hello": "hey there"}
```

You should see the message text printed. Next, try a goodbye.
This will cause the process to exit.
```bash
/m our@<your_process>:<your_package>:<your_publisher> "Goodbye"
```

If you try to send another Hello now, nothing will happen, because the process has exited [(assuming you have set `on_exit: "None"`; with `on_exit: "Restart"` it will immediately start up again)](./chapter_2.md#aside-on_exit).
Nice!
You can use `necdev start-package` to try again.

In the next chapter, you'll add some basic HTTP logic to serve a frontend from your simple process.
