# Defining Your Protocol

In the last chapter, you created a simple request-response pattern that uses strings as an IPC field type.
This is fine for certain limited cases, but in practice, most Nectar processes written in Rust use an IPC type that is serialized and deserialized to bytes using [Serde](https://serde.rs/).
There are a multitude of libraries that implement Serde's `Serialize` and `Deserialize` traits, and the process developer is responsible for selecting a strategy that is appropriate for their use case.

Some popular options are `bincode` and `serde_json`.
In this chapter, you will use `serde_json` to serialize our Rust structs to a byte vector of JSON.

Our old request looked like this:
```rust
Request::to(&our)
    .ipc(b"hello world")
    .expects_response(5)
    .send()
```

What if you want to have two kinds of messages, which our process can handle differently?
Let's make a type that implements the `Serialize` and `Deserialize` traits, and use that as our IPC type.

```rust
#[derive(Serialize, Deserialize)]
enum MyIPC {
    Hello(String),
    Goodbye,
}

impl MyIPC {
    fn hello(text: &str) -> Vec<u8> {
        serde_json::to_vec(&MyIPC::Hello(text.to_string())).unwrap()
    }

    fn goodbye() -> Vec<u8> {
        serde_json::to_vec(&MyIPC::Goodbye).unwrap()
    }

    fn parse(bytes: &[u8]) -> Result<MyIPC, serde_json::Error> {
        serde_json::from_slice::<MyIPC>(bytes)
    }
}
```

Now, when you form requests and response, instead of sticking a string in the `ipc` field, you can use the new IPC type.
This comes with a number of benefits:

- You can now use the `ipc` field to send arbitrary data, not just strings.
- Other programmers can look at our code and see what kinds of messages this process might send to their code.
- Other programmers can see what kinds of messages you expect to receive.
- By using an `enum`, you can exhaustively handle all possible message types, and handle unexpected messages with a default case or an error.

Defining IPC types is just one step towards writing interoperable code.
It's also critical to document the overall structure of the program along with message payloads and metadata used, if any.
Writing interoperable code is necessary for enabling permissionless composability, and NectarOS aims to make this the default kind of program, unlike the centralized web.

First, let's create a request that uses the new IPC type (and stop expecting a response):
```rust
Request::new()
    .target(&our)
    .ipc(MyIPC::hello("hello world"))
    .send();
```

Next, let's edit the way you handle a message in our process to use our new IPC type.
The process should attempt to parse every message into the `MyIPC` enum, handle the two cases, and handle any message that doesn't comport to the type.
This code goes into the `Ok(message)` case of the `match` statement on `await_message()`:
```rust
let Ok(ipc) = MyIPC::parse(message.ipc()) else {
    println!("{our}: received a message with weird IPC!");
    continue;
};
if message.is_request() {
    // Respond to a Hello by printing it, and a Goodbye by exiting
    // the loop, which will cause the process to exit.
    match ipc {
        MyIPC::Hello(text) => {
            println!("got a Hello: {text}");
        }
        MyIPC::Goodbye => {
            println!("goodbye!");
            break;
        }
    }
} else {
    // we only expect Hello responses. If we get a Goodbye, ignore it.
    match ipc {
        MyIPC::Hello(text) => {
            println!("got a Hello response: {text}");
        }
        MyIPC::Goodbye => {}
    }
}
```

Finally, let's edit our `pkg/manifest.json` to grant the terminal process permission to send messages to this process.
That way, you can use the terminal to send Hello and Goodbye messages.
Go into the manifest, and under the process name, edit (or add) the `grant_messaging` field like so:
```json
...
"grant_messaging": [
    "terminal:terminal:nectar"
],
...
```

After all this, you should be able to build and start your package, then see that initial Hello message.
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

If you try to send another Hello now, nothing will happen, because the process has exited. Nice! You can use `neddev start-package` to try again.

In the next chapter, you'll add some basic HTTP logic to serve a frontend from our simple process.
